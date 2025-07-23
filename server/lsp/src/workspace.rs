use std::{ffi::OsStr, path::PathBuf, sync::Arc};

use dashmap::DashMap;
use ini::Ini;
use line_index::{LineCol, LineIndex};
use log::{error, info};
use thiserror::Error;

use m_lang::{
    parser::parse,
    semantic::{SemanticModel, identifier_for_offset, semantics},
    syntax::MFileSource,
};
use tokio::runtime::Handle;
use tower_lsp::lsp_types::{Position, Range, TextDocumentItem, Url, WorkspaceFolder};

use crate::{
    definition::{LspDefinition, find_definitions},
    document::CurrentDocument,
};

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
    FileSource(#[from] m_lang::syntax::FileSourceError),
}

pub struct Workspace {
    opened_files: DashMap<Url, Arc<CurrentDocument>>,
    semantics: DashMap<PathBuf, Option<Arc<SemanticModel>>>,
}

impl Workspace {
    pub fn new() -> Workspace {
        Workspace {
            opened_files: DashMap::new(),
            semantics: DashMap::new(),
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

        self.semantics.clear();
        for path in files {
            self.semantics.insert(path, None);
        }

        info!("Found {} files", self.semantics.len());
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

        self.semantics.clear();
        for path in files {
            self.semantics.insert(path, None);
        }

        info!("Found {} files", self.semantics.len());

        Ok(())
    }

    pub async fn update_semantic_information(&self) {
        let mut handles = vec![];
        let current = Handle::current();
        for (_i, document) in self.semantics.iter().enumerate() {
            let path = document.key().to_path_buf();

            let handle = current.spawn_blocking(move || {
                let text = std::fs::read_to_string(&path).ok()?;
                let parsed = parse(&text, MFileSource::module());
                let semantics = semantics(&text, parsed.syntax());
                Some((path, semantics))
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(Some((path, semantics))) = handle.await {
                self.semantics.insert(path, Some(Arc::new(semantics)));
            }
        }
    }

    pub async fn get_opened_document(&self, uri: &Url) -> Option<Arc<CurrentDocument>> {
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

    pub async fn find_definitions(
        &self,
        uri: &Url,
        position: Position,
    ) -> Option<Vec<LspDefinition>> {
        let document = self.get_opened_document(uri).await?;
        let syntax = document.syntax();
        let text = syntax.text().to_string();

        let line_index = LineIndex::new(&text);
        let offset = line_index.offset(LineCol {
            line: position.line,
            col: position.character,
        })?;

        let (identifier, semantic_info) = identifier_for_offset(syntax, offset)?;
        let identifier = identifier.trim();

        let semantics = self
            .semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (path, Some(doc)) => Some((path.to_owned(), Arc::clone(doc))),
                _ => None,
            })
            .collect::<Vec<_>>();

        let definitions = find_definitions(&identifier, &semantic_info, semantics);
        Some(definitions)
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

        let document = CurrentDocument::new(uri.clone(), &document.text, file_source);
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

        let parsed = parse(&document.text, MFileSource::module());
        let semantics = semantics(&document.text, parsed.syntax());

        let mut diagnostics = vec![];

        if let Some(mut opened_file) = self.opened_files.get_mut(&uri) {
            let document = CurrentDocument::from_root(
                uri,
                &document.text,
                parsed.syntax(),
                parsed.diagnostics(),
            );
            diagnostics = document.diagnostics();
            *opened_file = Arc::new(document);
        }

        if file_source.is_module() {
            self.semantics.insert(path, Some(Arc::new(semantics)));
        }

        Ok(diagnostics)
    }
    pub async fn delete_document(&self, document_url: &Url) {
        self.opened_files.remove(&document_url);

        let path = document_url.to_file_path();
        if let Ok(path) = path {
            self.semantics.remove(&path);
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

                    // only modules needs to definitions
                    if MFileSource::try_from(entry.as_path()).is_ok_and(|m| m.is_module()) {
                        files.push(entry);
                    }
                }
            }
        }

        Ok(files)
    }
}
