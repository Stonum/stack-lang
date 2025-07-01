#![allow(deprecated)]
use std::ops::Deref;
use std::path::PathBuf;

use log::error;

use ini::Ini;

use biome_diagnostics::diagnostic::Diagnostic as _;
use dashmap::DashMap;

use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use m_lang::{
    parser::parse,
    semantic::{SemanticModel, identifier_for_offset, semantics},
    syntax::MFileSource,
};

use stack_lang_server::{
    definition::find_definitions, document::Document, format::format, get_text_size_from_position,
    position,
};
use tokio::runtime::Handle;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use std::time::Instant;

struct Backend {
    client: Client,
    document_map: DashMap<PathBuf, Document<SemanticModel>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut settings = ServerSettings::default();

        // Get initial settings from initialization_options if available
        if let Some(opts) = params.initialization_options {
            if let Ok(new_settings) = serde_json::from_value(opts) {
                settings = new_settings;
            }
        }

        let mut capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: vec!["dummy.do_something".to_string()],
                work_done_progress_options: Default::default(),
            }),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                file_operations: None,
            }),
            definition_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            ..ServerCapabilities::default()
        };

        if settings.lens_enabled {
            capabilities.code_lens_provider = Some(CodeLensOptions {
                resolve_provider: Some(false),
            })
        }

        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "stack lang server initialized!")
            .await;

        let start = Instant::now();
        self.client
            .send_notification::<StatusBarNotification>(StatusBarNotification::create(
                "parse definitions - load settings",
            ))
            .await;

        let mut folders: Option<Vec<(Url, bool)>> = None;

        let settings_path = async {
            let params = vec![ConfigurationItem {
                scope_uri: None,
                section: Some("stack.iniPath".to_owned()),
            }];
            let cfg = self.client.configuration(params).await.ok()?;
            match cfg.first().map(|s| s.to_owned()) {
                Some(Value::String(s)) => Some(s),
                _ => None,
            }
        }
        .await;

        if let Some(path) = settings_path {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("parse definitions from ini file {}!", path),
                )
                .await;

            folders = async {
                let ini = Ini::load_from_file_noescape(path + "\\stack.ini").ok()?;
                let app_path = ini.section(Some("AppPath"))?;
                let folders = app_path
                    .get_all("PRG")
                    .filter_map(|s| {
                        let url = Url::from_file_path(s).ok()?;
                        Some((url, s.ends_with("**")))
                    })
                    .collect::<Vec<_>>();

                Some(folders)
            }
            .await;
        }

        // get all workspace folders if we don't have them yet
        if folders.is_none() {
            self.client
                .log_message(
                    MessageType::INFO,
                    "parse definitions from workspace folder!".to_string(),
                )
                .await;

            folders = async {
                let f = self.client.workspace_folders().await.ok()?;
                f.map(|f| f.into_iter().map(|f| (f.uri, true)).collect())
            }
            .await;
        }

        let mut files = vec![];
        if let Some(folders) = folders {
            for (folder, recursively) in folders {
                if let Err(e) = get_files(folder, recursively, &mut files).await {
                    error!("{:?}", e);
                }
            }
        }

        self.client
            .log_message(MessageType::INFO, format!("found {} files", files.len()))
            .await;

        let mut handles = vec![];
        let current = Handle::current();
        for (i, file) in files.iter().enumerate() {
            self.client
                .send_notification::<StatusBarNotification>(StatusBarNotification::create(
                    &format!("parse definitions from files: {}/{}", i, files.len()),
                ))
                .await;

            if let Ok(text) = tokio::fs::read_to_string(&file).await {
                let handle = current.spawn_blocking(move || {
                    let parsed = parse(&text, MFileSource::module());

                    let semantics = semantics(parsed.syntax(), MFileSource::module());
                    let rope = Rope::from_str(&text);
                    (rope, semantics)
                });

                handles.push((Url::from_file_path(file).unwrap(), handle));
            }
        }

        for (uri, handle) in handles {
            let (rope, semantics) = handle.await.unwrap();
            let document = Document::new(uri, rope, semantics);
            self.document_map.insert(document.path(), document);
        }

        self.client
            .send_notification::<StatusBarNotification>(StatusBarNotification::clear())
            .await;

        self.client
            .log_message(
                MessageType::INFO,
                format!("parse definitions completed for {:?}", start.elapsed()),
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        };

        self.on_change(text_document).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let text_document = TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        };
        self.on_change(text_document).await;
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            match change.typ {
                FileChangeType::CREATED | FileChangeType::CHANGED => {
                    let text_document: Option<TextDocumentItem> = async {
                        let file = change.uri.to_file_path().ok()?;
                        let text = tokio::fs::read_to_string(&file).await.ok()?;
                        Some(TextDocumentItem {
                            uri: change.uri,
                            text,
                            version: 0,
                        })
                    }
                    .await;

                    if let Some(text_document) = text_document {
                        self.on_change(text_document).await;
                    }
                }
                FileChangeType::DELETED => {
                    let text_document = TextDocumentItem {
                        uri: change.uri,
                        text: String::new(),
                        version: 0,
                    };
                    self.on_delete(text_document).await;
                }
                _ => {}
            }
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let definition = async {
            let uri = params.text_document_position_params.text_document.uri;

            let pos = params.text_document_position_params.position;
            let document = self
                .document_map
                .get(&uri.to_file_path().unwrap_or_default())?;
            let source = document.text();

            let parsed = parse(&source.to_string(), MFileSource::script());
            let syntax = parsed.syntax();

            let offset = get_text_size_from_position(source, pos)?;

            let (identifier, semantic_info) = identifier_for_offset(syntax, offset)?;
            let identifier = identifier.trim();

            let definitions = find_definitions(&identifier, &semantic_info, &self.document_map);
            let locations = definitions
                .into_iter()
                .map(|d| Location::new(d.uri, d.range))
                .collect::<Vec<_>>();

            Some(GotoDefinitionResponse::Array(locations))
        }
        .await;
        Ok(definition)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let file_uri = params.text_document.uri;

        let document_symbol = async || -> Option<DocumentSymbolResponse> {
            let document = self
                .document_map
                .get(&file_uri.to_file_path().unwrap_or_default())?;

            let document = document.deref();
            Some(document.into())
        }()
        .await;

        Ok(document_symbol)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let hover = async {
            let uri = params.text_document_position_params.text_document.uri;

            let pos = params.text_document_position_params.position;

            let document = self
                .document_map
                .get(&uri.to_file_path().unwrap_or_default())?;
            let source = document.text();

            let parsed = parse(&source.to_string(), MFileSource::script());
            let syntax = parsed.syntax();

            let offset = get_text_size_from_position(source, pos)?;

            let (identifier, semantic_info) = identifier_for_offset(syntax, offset)?;
            let identifier = identifier.trim();

            let definitions = find_definitions(&identifier, &semantic_info, &self.document_map);
            let range = definitions.first().map(|d| d.range);
            let locations = definitions
                .into_iter()
                .map(|d| d.markup)
                .collect::<Vec<_>>();

            Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: locations.join("\n\n"),
                }),
                range,
            })
        }
        .await;
        Ok(hover)
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let edited = async {
            let DocumentRangeFormattingParams {
                text_document,
                range,
                options,
                ..
            } = params;

            let uri = text_document.uri;
            let document = self
                .document_map
                .get(&uri.to_file_path().unwrap_or_default())?;

            format(document.text(), options, range).await
        }
        .await;

        Ok(edited)
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let file_uri = params.text_document.uri;

        let code_lens = async || -> Option<Vec<CodeLens>> {
            let document = self
                .document_map
                .get(&file_uri.to_file_path().unwrap_or_default())?;

            let document = document.deref();
            Some(document.into())
        }()
        .await;

        Ok(code_lens)
    }
}

#[derive(Debug, Clone)]
struct TextDocumentItem {
    uri: Url,
    text: String,
    version: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct StatusBarParams {
    text: String,
}

struct StatusBarNotification;
impl StatusBarNotification {
    fn create(text: &str) -> StatusBarParams {
        StatusBarParams {
            text: text.to_string(),
        }
    }
    fn clear() -> StatusBarParams {
        StatusBarParams {
            text: String::new(),
        }
    }
}

impl Notification for StatusBarNotification {
    type Params = StatusBarParams;
    const METHOD: &'static str = "custom/statusBar";
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct ServerSettings {
    lens_enabled: bool,
}

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        let uri = params.uri;
        let text = params.text;

        let path = uri.to_file_path().unwrap_or_default();
        let mut file_source = None;
        if let Some(ext) = path.extension() {
            match MFileSource::try_from_extension(ext) {
                Ok(src) => file_source = Some(src),
                Err(err) => {
                    let err = Diagnostic::new_simple(Default::default(), err.to_string());
                    self.client.publish_diagnostics(uri, vec![err], None).await;
                    return;
                }
            }
        }

        if file_source.is_none() {
            return;
        }

        let file_source = file_source.unwrap();

        let diagnostics;
        {
            let parsed = parse(&text, file_source);
            let semantics = semantics(parsed.syntax(), file_source);
            let rope = Rope::from_str(&text);

            diagnostics = parsed
                .diagnostics()
                .iter()
                .map(|error| {
                    let range = error
                        .location()
                        .span
                        .map(|range| position(&rope, range).unwrap_or_default());

                    Diagnostic::new_simple(range.unwrap_or_default(), format!("{}", error.message))
                })
                .collect();

            let document = Document::new(uri.clone(), rope, semantics);
            self.document_map.insert(document.path(), document);
        }

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn on_delete(&self, params: TextDocumentItem) {
        self.document_map
            .remove(&params.uri.to_file_path().unwrap_or_default());

        self.client
            .publish_diagnostics(params.uri, vec![], Some(params.version))
            .await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

async fn get_files(dir: Url, recursively: bool, files: &mut Vec<String>) -> std::io::Result<()> {
    let mut to_visit: Vec<Url> = vec![dir];

    while !to_visit.is_empty() {
        let path = to_visit.pop().unwrap().to_file_path().unwrap();

        if path.is_dir() {
            let mut dir = tokio::fs::read_dir(path).await?;
            while let Some(entry) = dir.next_entry().await? {
                let path = entry.path();

                // visit nested dirs only with recursively flag
                if path.is_dir() && recursively {
                    to_visit.push(Url::from_file_path(path).unwrap());
                } else if let Some(ext) = path.extension() {
                    match ext.to_str().unwrap() {
                        "prg" => files.push(path.to_str().unwrap().to_string()),
                        _ => (),
                    }
                }
            }
        }
    }

    Ok(())
}
