#![allow(deprecated)]
use std::env;

use env_logger::Builder;
use log::{LevelFilter, error, info, trace};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use stack_lang_server::document::{code_lens, document_symbol_response};
use stack_lang_server::{format::format, workspace::Workspace};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use std::time::Instant;

struct Backend {
    client: Client,
    workspace: Workspace,
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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Stack lang server initialized!");

        let start = Instant::now();
        info!("Start workspace initialization");
        self.send_status_bar_notofication("Workspace initialization - loading settings")
            .await;

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

        let get_folders = async || {
            self.client.workspace_folders().await.unwrap_or_else(|e| {
                error!("Error receiving workspace folders: {e}");
                None
            })
        };

        match settings_path {
            Some(path) if path != "" => {
                if let Err(error) = self.workspace.init_with_settings_file(&path).await {
                    error!("Initialization error: {error}");

                    // try init from workspace folders
                    info!("Trying initialization from workspace folders");
                    let folders = get_folders().await;
                    if let Err(error) = self.workspace.init_with_workspace_folders(folders).await {
                        error!("Initialization error: {error}");
                        return;
                    }
                }
            }
            _ => {
                let folders = get_folders().await;
                if let Err(error) = self.workspace.init_with_workspace_folders(folders).await {
                    error!("{error}");
                    return;
                }
            }
        }

        self.send_status_bar_notofication(
            "Workspace initialization - updating semantic information",
        )
        .await;

        self.workspace.update_semantic_information().await;

        info!(
            "Workspae initialization completed for {:?}",
            start.elapsed()
        );
        self.send_status_bar_notofication("").await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document;
        let file_uri = text_document.uri.clone();
        trace!("did_open {}", &file_uri);

        match self.workspace.open_document(text_document).await {
            Ok(diagnostics) => self.publish_diagnostics(file_uri, diagnostics).await,
            Err(e) => error!("Open workspace document: {e}"),
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let file_uri = params.text_document.uri;
        trace!("did_close {}", &file_uri);

        self.workspace.close_document(&file_uri).await;
        self.publish_diagnostics(file_uri, vec![]).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let text_document = TextDocumentItem {
            uri: params.text_document.uri,
            language_id: String::from(""),
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        };
        let file_uri = text_document.uri.clone();
        trace!("did_change {}", &file_uri);

        match self.workspace.change_document(text_document).await {
            Ok(diagnostics) => self.publish_diagnostics(file_uri, diagnostics).await,
            Err(e) => error!("Change workspace document: {e}"),
        }
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            trace!(
                "did_change_watched_files {} - {:?}",
                &change.uri, change.typ
            );

            match change.typ {
                FileChangeType::CREATED | FileChangeType::CHANGED => {
                    let text_document: Option<TextDocumentItem> = async {
                        let file = change.uri.to_file_path().ok()?;
                        let text = tokio::fs::read_to_string(&file).await.ok()?;
                        Some(TextDocumentItem {
                            uri: change.uri,
                            language_id: String::from(""),
                            text,
                            version: 0,
                        })
                    }
                    .await;

                    if let Some(text_document) = text_document {
                        let file_uri = text_document.uri.clone();

                        match self.workspace.change_document(text_document).await {
                            Ok(diagnostics) => {
                                self.publish_diagnostics(file_uri, diagnostics).await
                            }
                            Err(e) => error!("Change workspace document: {e}"),
                        }
                    }
                }
                FileChangeType::DELETED => {
                    self.workspace.delete_document(&change.uri).await;
                    self.publish_diagnostics(change.uri, vec![]).await;
                }
                _ => {}
            }
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let file_uri = params.text_document.uri;
        trace!("document_symbol {}", &file_uri);

        let document_symbol: Option<DocumentSymbolResponse> = async {
            let document = self.workspace.get_opened_document(&file_uri).await?;
            Some(document_symbol_response(
                document.uri(),
                &document.semantics(),
            ))
        }
        .await;

        Ok(document_symbol)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let file_uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        trace!("goto_definition {} {:?}", &file_uri, &pos);

        let definition: Option<GotoDefinitionResponse> = async {
            let definitions = self.workspace.find_definitions(&file_uri, pos).await?;
            let locations = definitions
                .into_iter()
                .map(|d| Location::new(d.uri, d.range))
                .collect::<Vec<_>>();
            Some(GotoDefinitionResponse::Array(locations))
        }
        .await;

        Ok(definition)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let file_uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        trace!("hover {} {:?}", &file_uri, &pos);

        let hover: Option<Hover> = async {
            let definitions = self.workspace.find_definitions(&file_uri, pos).await?;
            let markups = definitions
                .into_iter()
                .map(|d| MarkedString::String(d.markup))
                .collect::<Vec<_>>();

            Some(Hover {
                contents: HoverContents::Array(markups),
                range: None,
            })
        }
        .await;

        Ok(hover)
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let file_uri = params.text_document.uri;
        let range = params.range;
        let options = params.options;
        trace!("range_formatting {} {:?}", &file_uri, &range);

        let edited = async {
            let document = self.workspace.get_opened_document(&file_uri).await?;

            format(&document, options, range).await
        }
        .await;

        Ok(edited)
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        let file_uri = params.text_document.uri;
        trace!("code_lens {}", &file_uri);

        let code_lens: Option<Vec<CodeLens>> = async {
            let document = self.workspace.get_opened_document(&file_uri).await?;

            Some(code_lens(&document.semantics()))
        }
        .await;

        Ok(code_lens)
    }
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
    async fn publish_diagnostics(&self, uri: Url, diagnostic: Vec<(Range, String)>) {
        let diagnostics = diagnostic
            .into_iter()
            .map(|(range, message)| Diagnostic::new_simple(range, message))
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn send_status_bar_notofication(&self, msg: &str) {
        self.client
            .send_notification::<StatusBarNotification>(StatusBarNotification::create(msg))
            .await;
    }
}

#[tokio::main]
async fn main() {
    init_logger();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        workspace: Workspace::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn init_logger() {
    let mut builder = Builder::from_default_env();

    // Set default level to info if not specified
    if env::var("RUST_LOG").is_err() {
        builder.filter_level(LevelFilter::Info);
    }

    builder
        .format_module_path(false)
        .format_target(false)
        .format_timestamp_millis()
        .init();
}
