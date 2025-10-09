use std::time::Duration;
use std::{path::PathBuf, sync::Arc};

use dashmap::DashMap;
use ini::Ini;
use line_index::{LineCol, LineIndex};
use log::{error, info};
use serde_json::Value;
use thiserror::Error;

use mlang_core::{AnyMCoreDefinition, load_core_api};
use mlang_lsp_definition::{
    CodeSymbolDefinition as _, CodeSymbolInformation as _, LocationDefinition as _, SemanticInfo,
    StringLowerCase, get_declaration, get_hover, get_reference, get_symbols,
};
use mlang_parser::parse;
use mlang_semantic::{SemanticModel, identifier_for_offset, semantics};
use mlang_syntax::MFileSource;

use tokio::runtime::Handle;
use tokio::sync::Semaphore;
use tokio::task::JoinError;

use tower_lsp::lsp_types::{
    CodeLens, Command, DocumentSymbolResponse, GotoDefinitionResponse, Hover, HoverContents,
    Location, Position, Range, SemanticTokens, SymbolInformation, TextDocumentItem, Url,
    WorkspaceFolder,
};

use crate::document::CurrentDocument;
use crate::tokens::semantic_tokens;

#[derive(Debug, Error)]
pub enum WorkspaceInitializationError {
    #[error("{0}")]
    Ini(#[from] ini::Error),
    #[error("Section {0} not found")]
    SectionNotFound(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Folders not found")]
    FoldersNotFound,
}

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Failed convert Url to file path {0}")]
    UrlConvertation(Url),
    #[error("{0}")]
    FileSource(#[from] mlang_syntax::FileSourceError),
    #[error("{0}")]
    JoinHandle(#[from] JoinError),
}

pub struct Workspace {
    opened_files: DashMap<Url, Arc<CurrentDocument>>,
    mlang_semantics: DashMap<PathBuf, Option<Arc<SemanticModel>>>,
    core: Vec<AnyMCoreDefinition>,
}

impl Workspace {
    pub fn new() -> Workspace {
        Workspace {
            opened_files: DashMap::new(),
            mlang_semantics: DashMap::new(),
            core: load_core_api(),
        }
    }

    pub async fn init_with_workspace_folders(
        &self,
        folders: Option<Vec<WorkspaceFolder>>,
    ) -> Result<(), WorkspaceInitializationError> {
        let folders = folders.ok_or(WorkspaceInitializationError::FoldersNotFound)?;
        info!("Get files from workspace folders!");

        let folders = folders
            .into_iter()
            .filter_map(|f| f.uri.to_file_path().ok())
            .map(|path| (path, true)) // recursively all folders in workspace
            .collect::<Vec<_>>();

        let files = self.get_files(folders).await?;

        self.mlang_semantics.clear();
        for path in files {
            self.mlang_semantics.insert(path, None);
        }

        info!("Found {} files", self.mlang_semantics.len());
        Ok(())
    }

    pub async fn init_with_settings_file(
        &self,
        path: &str,
    ) -> Result<(), WorkspaceInitializationError> {
        let mut path = PathBuf::from(path);
        if !&path.is_file() {
            path.push("stack.ini");
        }

        info!(
            "Get files from ini file {}!",
            path.to_str().unwrap_or_default()
        );

        let ini = Ini::load_from_file_noescape(path)?;
        let app_path =
            ini.section(Some("AppPath"))
                .ok_or(WorkspaceInitializationError::SectionNotFound(
                    "AppPath".to_string(),
                ))?;

        let folders = app_path
            .get_all("PRG")
            .filter_map(|s| {
                let mut path = PathBuf::from(s);

                // recursively only folders ends with **
                let recursively = path.ends_with("**");
                if recursively {
                    path.pop();
                }
                Some((path, recursively))
            })
            .collect::<Vec<_>>();

        let files = self.get_files(folders).await?;

        self.mlang_semantics.clear();
        for path in files {
            self.mlang_semantics.insert(path, None);
        }

        info!("Found {} files", self.mlang_semantics.len());

        Ok(())
    }

    pub async fn update_semantic_information(&self) {
        let mut handles = vec![];

        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let current = Handle::current();
        let semaphore = Arc::new(Semaphore::new(num_cores * 2));

        for document in self.mlang_semantics.iter() {
            let path = document.key().to_path_buf();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let handle = current.spawn_blocking(move || {
                let _ = permit;
                let text = std::fs::read_to_string(&path).ok()?;

                let file_extension = path.extension()?;
                let file_source = MFileSource::try_from_extension(file_extension).ok()?;

                let parsed = parse(&text, MFileSource::module());
                let semantics = semantics(&text, parsed.syntax(), file_source);
                Some((path, semantics))
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(Some((path, semantics))) = handle.await {
                self.mlang_semantics.insert(path, Some(Arc::new(semantics)));
            }
        }
    }

    pub async fn get_opened_document(&self, uri: &Url) -> Option<Arc<CurrentDocument>> {
        // deadlock guard
        while self.opened_files.try_get(uri).is_locked() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        if let Some(document) = self.opened_files.get(uri) {
            return Some(Arc::clone(document.value()));
        }

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConvertation(uri.clone())))
            .ok()?;
        let file_source = MFileSource::try_from(path.as_path()).ok()?;

        let text = tokio::fs::read_to_string(&path).await.ok()?;
        let document = Arc::new(CurrentDocument::new(uri.clone(), &text, file_source));

        self.opened_files.insert(uri.clone(), Arc::clone(&document));

        Some(document)
    }

    pub async fn hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        let semantic_info = self.identifier_from_position(uri, position).await?;

        let core_markups = get_hover(&semantic_info, &self.core);
        if core_markups.len() > 0 {
            return Some(Hover {
                contents: HoverContents::Array(core_markups),
                range: None,
            });
        }

        let semantics = self
            .mlang_semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (_path, Some(definitions)) => Some(Arc::clone(definitions)),
                _ => None,
            })
            .collect::<Vec<_>>();

        let definitions = semantics.iter().flat_map(|arc| arc.definitions());

        let markups = get_hover(&semantic_info, definitions);
        Some(Hover {
            contents: HoverContents::Array(markups),
            range: None,
        })
    }

    pub async fn goto_definition(
        &self,
        uri: &Url,
        position: Position,
    ) -> Option<GotoDefinitionResponse> {
        let semantic_info = self.identifier_from_position(uri, position).await?;

        let semantics = self
            .mlang_semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (path, Some(semantics)) => {
                    let uri = Url::from_file_path(path).ok()?;
                    let semantics = Arc::clone(semantics);
                    Some((uri, semantics))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let definitions = semantics
            .iter()
            .map(|(uri, arc)| arc.definitions().map(|d| (uri.clone(), d)))
            .flatten();

        let locations = get_declaration(&semantic_info, definitions);
        Some(GotoDefinitionResponse::Array(locations))
    }

    pub async fn references(&self, uri: &Url, position: Position) -> Option<Vec<Location>> {
        let semantic_info = self.identifier_from_position(uri, position).await?;

        let locations = self
            .mlang_semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (path, Some(semantics)) => {
                    let uri = Url::from_file_path(path).ok()?;
                    let references = semantics.references();

                    let locations = get_reference(&semantic_info, &uri, references);
                    Some(locations)
                }
                _ => None,
            })
            .flatten()
            .collect::<Vec<_>>();

        Some(locations)
    }

    pub async fn document_symbol_response(&self, uri: &Url) -> Option<DocumentSymbolResponse> {
        let document = self.get_opened_document(uri).await?;

        let definitions = document.definitions();
        let response = get_symbols(uri, definitions);

        Some(DocumentSymbolResponse::Flat(response))
    }

    pub async fn symbol_information(&self, query: &str) -> Option<Vec<SymbolInformation>> {
        let semantics = self.mlang_semantics.iter().filter_map(|r| match r.pair() {
            (path, Some(definitions)) => {
                let uri = Url::from_file_path(path).ok()?;
                Some((uri, Arc::clone(definitions)))
            }
            _ => None,
        });

        let information = semantics
            .map(|(uri, semantics)| {
                if query != "" {
                    let query = StringLowerCase::new(query);
                    get_symbols(
                        &uri,
                        semantics
                            .definitions()
                            .filter(|d| d.partial_compare_with(&query)),
                    )
                } else {
                    get_symbols(&uri, semantics.definitions())
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        Some(information)
    }

    pub async fn code_lens(&self, uri: &Url) -> Option<Vec<CodeLens>> {
        let document = self.get_opened_document(uri).await?;
        let command = String::from("stack.movetoLine");

        let definitions = document.definitions();
        let response = definitions
            .filter_map(|def| {
                let container = def.container()?;
                let title = container.symbol_name();
                let line = container.range().start.line;
                let args = vec![Value::Number(line.into())];

                Some(CodeLens {
                    range: def.lsp_range(),
                    command: Some(Command::new(title, command.clone(), Some(args))),
                    data: None,
                })
            })
            .collect();

        Some(response)
    }

    pub async fn semantic_tokens(
        &self,
        uri: &Url,
        tokens_range: Option<Range>,
    ) -> Option<SemanticTokens> {
        let document = self.get_opened_document(uri).await?;
        let syntax = document.syntax();
        let line_index = document.line_index();

        Some(SemanticTokens {
            result_id: None,
            data: semantic_tokens(syntax, &line_index, tokens_range),
        })
    }
}

impl Workspace {
    pub async fn open_document(
        &self,
        document: TextDocumentItem,
    ) -> Result<Vec<(Range, String)>, WorkspaceError> {
        let uri = document.uri;

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConvertation(uri.clone())))?;
        let file_source = MFileSource::try_from(path.as_path())?;

        let document_uri = uri.clone();
        let handle = tokio::task::spawn_blocking(move || {
            CurrentDocument::new(document_uri, &document.text, file_source)
        });

        let document = handle.await?;
        let diagnostics = document.diagnostics();

        self.opened_files.insert(uri, Arc::new(document));

        Ok(diagnostics)
    }

    pub async fn close_document(&self, document_url: &Url) {
        self.opened_files.remove(document_url);
    }

    pub async fn change_document(
        &self,
        document: TextDocumentItem,
    ) -> Result<Vec<(Range, String)>, WorkspaceError> {
        let uri = document.uri;

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConvertation(uri.clone())))?;
        let file_source = MFileSource::try_from(path.as_path())?;

        // block file to read another treads
        let opened_file = self.opened_files.get_mut(&uri);

        let document_uri = uri.clone();
        let handle = tokio::task::spawn_blocking(move || {
            let parsed = parse(&document.text, file_source);
            let semantics = semantics(&document.text, parsed.syntax(), file_source);

            let document = CurrentDocument::from_root(
                document_uri,
                file_source,
                &document.text,
                parsed.syntax(),
                parsed.diagnostics(),
            );

            (document, semantics)
        });

        let (document, semantics) = handle.await?;

        if file_source.is_module() || file_source.is_handler() {
            self.mlang_semantics.insert(path, Some(Arc::new(semantics)));
        }

        let mut diagnostics = vec![];
        if let Some(mut opened_file) = opened_file {
            diagnostics = document.diagnostics();
            *opened_file = Arc::new(document);
        }

        Ok(diagnostics)
    }
    pub async fn delete_document(&self, document_url: &Url) {
        self.opened_files.remove(&document_url);

        let path = document_url.to_file_path();
        if let Ok(path) = path {
            self.mlang_semantics.remove(&path);
        }
    }
}

impl Workspace {
    async fn get_files(&self, to_visit: Vec<(PathBuf, bool)>) -> std::io::Result<Vec<PathBuf>> {
        let mut files = vec![];
        let mut to_visit = to_visit;

        while let Some((path, recursively)) = to_visit.pop() {
            if path.is_dir() {
                let mut dir = tokio::fs::read_dir(path).await?;
                while let Some(entry) = dir.next_entry().await? {
                    let entry = entry.path();

                    if entry.is_dir() {
                        // visit nested dirs only with recursively flag
                        if recursively {
                            to_visit.push((entry, recursively));
                        }
                        continue;
                    }

                    // only modules and handlers needs to definitions
                    if MFileSource::try_from(entry.as_path())
                        .is_ok_and(|m| m.is_module() || m.is_handler())
                    {
                        files.push(entry);
                    }
                }
            }
        }

        Ok(files)
    }

    async fn identifier_from_position(
        &self,
        uri: &Url,
        position: Position,
    ) -> Option<SemanticInfo> {
        let document = self.get_opened_document(uri).await?;
        let syntax = document.syntax();
        let text = syntax.text().to_string();

        let line_index = LineIndex::new(&text);
        let offset = line_index.offset(LineCol {
            line: position.line,
            col: position.character,
        })?;

        identifier_for_offset(syntax, offset)
    }
}
