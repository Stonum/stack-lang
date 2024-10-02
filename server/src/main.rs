#![allow(deprecated)]
use std::path::PathBuf;

use log::error;
use log::info;

use ini::Ini;

use dashmap::DashMap;
use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use stack_lang_server::def::Definition;
use stack_lang_server::lexer::{Lexer, Token};
use stack_lang_server::parser;
use stack_lang_server::position;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
    definitions_map: DashMap<PathBuf, Vec<Definition>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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
                // document_range_formatting_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "stack lang server initialized!")
            .await;

        self.client
            .send_notification::<StatusBarNotification>(StatusBarParams {
                text: "parse definitions".to_string(),
            })
            .await;

        let mut folders: Option<Vec<Url>> = None;

        let settings_path = async {
            let params = vec![ConfigurationItem {
                scope_uri: None,
                section: Some("stack.iniPath".to_owned()),
            }];
            let cfg = self.client.configuration(params).await.ok()?;
            match cfg.get(0).map(|s| s.to_owned()) {
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
                    .filter_map(|s| Url::from_file_path(s).ok())
                    .collect::<Vec<_>>();

                Some(folders)
            }
            .await;
        }

        // get all workspace folders if we don't have them yet
        if folders.is_none() {
            folders = async {
                let f = self.client.workspace_folders().await.ok()?;
                f.map(|f| f.into_iter().map(|f| f.uri).collect())
            }
            .await;
        }

        let mut files = vec![];
        if let Some(folders) = folders {
            for folder in folders {
                if let Err(e) = get_files(folder, &mut files).await {
                    error!("{:?}", e);
                }
            }
        }

        self.client
            .log_message(MessageType::INFO, format!("found {} files", files.len()))
            .await;

        for (i, file) in files.iter().enumerate() {
            self.client
                .send_notification::<StatusBarNotification>(StatusBarParams {
                    text: format!("parse definitions from files: {}/{}", i, files.len()),
                })
                .await;

            if let Ok(text) = tokio::fs::read_to_string(&file).await {
                self.set_definition(TextDocumentItem {
                    uri: Url::from_file_path(&file).unwrap(),
                    text,
                    version: 0,
                })
                .await;
            }
        }

        self.client
            .send_notification::<StatusBarNotification>(StatusBarParams {
                text: "".to_string(),
            })
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

        self.set_definition(text_document.clone()).await;
        self.on_change(text_document).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let text_document = TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        };
        self.set_definition(text_document.clone()).await;
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
                        self.set_definition(text_document.clone()).await;
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

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let definition = async {
            let uri = params.text_document_position_params.text_document.uri;

            let Position { character, line } = params.text_document_position_params.position;

            let rope = self.document_map.get(uri.as_str())?;
            let source = rope.line(line as usize).to_string();

            let mut lexer = Lexer::new(&source);
            let rope = Rope::from(source.as_str());

            let mut identifier = None;
            while let Some(token) = lexer.next() {
                if let Ok(token) = token {
                    let range = position(&rope, lexer.span()).unwrap_or_default();
                    if character >= range.start.character && character <= range.end.character {
                        if let Token::Identifier(id) = token {
                            identifier = Some(id.to_string());
                            break;
                        };
                    }
                }
            }

            let identifier = identifier?.to_lowercase();

            let mut loc: Vec<Location> = vec![];
            for m in self.definitions_map.iter() {
                let (path, v) = m.pair();
                let uri = Url::from_file_path(path).unwrap_or(Url::parse("file:///").unwrap());

                let mut locations = v
                    .iter()
                    .filter_map(|def| {
                        if def.identifier().to_lowercase() == identifier {
                            return Some(Location::new(uri.clone(), def.position()));
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                loc.append(&mut locations);
            }
            Some(GotoDefinitionResponse::Array(loc))
        }
        .await;
        Ok(definition)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let file_uri = params.text_document.uri;

        let document_symbol = || -> Option<DocumentSymbolResponse> {
            let mut vec = vec![];

            let defs = self
                .definitions_map
                .get(&file_uri.to_file_path().unwrap_or_default())?;

            for def in defs.iter() {
                match def {
                    Definition::Func {
                        identifier, pos, ..
                    } => {
                        let symbol = SymbolInformation {
                            name: identifier.clone(),
                            kind: SymbolKind::FUNCTION,
                            tags: None,
                            deprecated: None,
                            location: Location::new(file_uri.clone(), *pos),
                            container_name: None,
                        };

                        vec.push(symbol);
                    }
                    Definition::Class {
                        identifier,
                        pos,
                        methods,
                        ..
                    } => {
                        let symbol = SymbolInformation {
                            name: identifier.clone(),
                            kind: SymbolKind::CLASS,
                            tags: None,
                            deprecated: None,
                            location: Location::new(file_uri.clone(), *pos),
                            container_name: None,
                        };

                        vec.push(symbol);

                        for method in methods.iter() {
                            match method {
                                Definition::Func {
                                    identifier: method_identifier,
                                    pos,
                                    ..
                                } => {
                                    let symbol = SymbolInformation {
                                        name: method_identifier.clone(),
                                        kind: SymbolKind::FUNCTION,
                                        tags: None,
                                        deprecated: None,
                                        location: Location::new(file_uri.clone(), *pos),
                                        container_name: Some(identifier.clone()),
                                    };

                                    vec.push(symbol);
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                };
            }

            Some(DocumentSymbolResponse::Flat(vec))
        }();

        Ok(document_symbol)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let hover = async {
            let uri = params.text_document_position_params.text_document.uri;

            let Position { character, line } = params.text_document_position_params.position;

            let rope = self.document_map.get(uri.as_str())?;
            let source = rope.line(line as usize).to_string();

            let mut lexer = Lexer::new(&source);
            let rope = Rope::from(source.as_str());

            let mut identifier = None;
            while let Some(token) = lexer.next() {
                if let Ok(token) = token {
                    let range = position(&rope, lexer.span()).unwrap_or_default();
                    if character >= range.start.character && character <= range.end.character {
                        if let Token::Identifier(id) = token {
                            identifier = Some(id.to_string());
                        };
                    }
                }
            }

            let identifier = identifier?.to_lowercase();

            let mut loc: Vec<String> = vec![];
            for m in self.definitions_map.iter() {
                let (_path, v) = m.pair();

                let mut locations = v
                    .iter()
                    .filter_map(|def| {
                        if def.identifier().to_lowercase() == identifier {
                            return Some(format!(
                                "**{}{}**  \n{}  \n{}",
                                def.identifier(),
                                def.parameters(),
                                def.description(),
                                def.doc_string()
                            ));
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                loc.append(&mut locations);
            }
            Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: loc.join("\n\n"),
                }),
                range: None,
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
        self.client
            .log_message(MessageType::INFO, "formatting triggered!")
            .await;

        info!("range_formatting {:?}", params);
        Ok(None)
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct InlayHintParams {
    path: String,
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
impl Notification for StatusBarNotification {
    type Params = StatusBarParams;
    const METHOD: &'static str = "custom/statusBar";
}

impl Backend {
    async fn set_definition(&self, params: TextDocumentItem) {
        let TextDocumentItem { uri, text, .. } = params;

        let rope = Rope::from(text.as_str());
        let (decl, errs) = parser::parse(&text).into_output_errors();

        let mut diagnostics = vec![];

        if decl.is_none() {
            let mut diagnostic = Diagnostic::new_simple(Range::default(), format!("Parse error"));
            if let Some(error) = errs.last() {
                let pos = position(&rope, (*error.span()).into());
                diagnostic = Diagnostic::new_simple(
                    pos.unwrap_or_default(),
                    format!("Parse error: {error}"),
                );
            }
            diagnostics.push(diagnostic);
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, Some(params.version))
            .await;

        if let Some(decl) = decl {
            let definitions = decl
                .into_iter()
                .map(|d| Definition::try_from_declaration(d, &rope))
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();

            self.definitions_map
                .insert(uri.to_file_path().unwrap_or_default(), definitions);
        }
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());
    }

    async fn on_delete(&self, params: TextDocumentItem) {
        self.document_map.remove(params.uri.as_str());
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
        definitions_map: DashMap::new(),
    })
    .finish();

    serde_json::json!({"test": 20});
    Server::new(stdin, stdout, socket).serve(service).await;
}

async fn get_files(dir: Url, files: &mut Vec<String>) -> std::io::Result<()> {
    let mut to_visit: Vec<Url> = vec![dir];

    while !to_visit.is_empty() {
        let path = to_visit.pop().unwrap().to_file_path().unwrap();

        if path.is_dir() {
            let mut dir = tokio::fs::read_dir(path).await?;
            while let Some(entry) = dir.next_entry().await? {
                let path = entry.path();

                if path.is_dir() {
                    to_visit.push(Url::from_file_path(path).unwrap());
                } else {
                    if let Some(ext) = path.extension() {
                        match ext.to_str().unwrap() {
                            "prg" | "hdl" => files.push(path.to_str().unwrap().to_string()),
                            _ => (),
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
