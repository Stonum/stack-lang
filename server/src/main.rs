use std::path::PathBuf;

use log::debug;
use log::error;
use log::info;

use ini::Ini;

use dashmap::DashMap;
use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use stack_language_server::lexer::{Lexer, Token};
use stack_language_server::parser::{Parser, Stmt};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
    definitions_map: DashMap<PathBuf, Vec<(String, Range)>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            offset_encoding: None,
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
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
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "stack lang server initialized!")
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

        self.client
            .log_message(MessageType::INFO, "start parse definitions!")
            .await;

        let mut files = vec![];
        if let Some(folders) = folders {
            for folder in folders {
                if let Err(e) = get_files(folder, &mut files).await {
                    error!("{:?}", e);
                }
            }
        }

        for file in files {
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
            .log_message(MessageType::INFO, "end parse definitions!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("did open {}", params.text_document.uri);

        let text_document = TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        };

        self.set_definition(text_document.clone()).await;
        self.on_change(text_document).await;

        info!("did open done ");
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

    // async fn did_save(&self, _: DidSaveTextDocumentParams) {}

    // async fn did_close(&self, _: DidCloseTextDocumentParams) {}

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

            let mut identifier = None;
            while let Some(token) = &lexer.next() {
                if let Ok(token) = token {
                    let range = lexer.position().unwrap_or_default();
                    if character >= range.start.character && character <= range.end.character {
                        if let Token::Identifier(id) = token {
                            identifier = Some(id.to_string());
                        };
                    }
                }
            }

            let identifier = identifier.unwrap().to_lowercase();

            let mut loc: Vec<Location> = vec![];
            for m in self.definitions_map.iter() {
                let (path, v) = m.pair();
                let uri = Url::from_file_path(path).unwrap_or(Url::parse("file:///").unwrap());

                let mut locations = v
                    .iter()
                    .filter_map(|(def, range)| {
                        if def.to_lowercase() == identifier {
                            return Some(Location::new(uri.clone(), *range));
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

            for (def, range) in defs.iter() {
                vec.push(SymbolInformation {
                    name: def.to_string(),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    location: Location::new(file_uri.clone(), *range),
                    container_name: None,
                });
            }

            Some(DocumentSymbolResponse::Flat(vec))
        }();

        Ok(document_symbol)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        info!("references: {:?}", params);
        Ok(None)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri.to_string();
        info!("semantic_tokens_full: {:?}", uri);
        Ok(None)
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let uri = params.text_document.uri.to_string();
        info!("semantic_tokens_range: {}", uri);
        Ok(None)
    }

    async fn inlay_hint(
        &self,
        params: tower_lsp::lsp_types::InlayHintParams,
    ) -> Result<Option<Vec<InlayHint>>> {
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        info!("completion: {:?}", params);

        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        info!("rename: {:?}", params);

        Ok(None)
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

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
        info!("did_change_watched_files");
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

impl Backend {
    async fn set_definition(&self, params: TextDocumentItem) {
        let mut parser = Parser::new(&params.text);
        let definitions = parser.parse_definitions();

        self.definitions_map
            .insert(params.uri.to_file_path().unwrap_or_default(), definitions);
    }

    async fn on_change(&self, params: TextDocumentItem) {
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map
            .insert(params.uri.to_string(), rope.clone());

        info!("{}", params.uri.to_string());
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
