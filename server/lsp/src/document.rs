use biome_diagnostics::diagnostic::Diagnostic as _;
use line_index::{LineColRange, LineIndex};

use mlang_parser::{ParseDiagnostic, parse};
use mlang_semantic::{AnyMDefinition, SemanticModel, semantics};
use mlang_syntax::{MFileSource, MLanguage, MSyntaxNode, SendNode, SyntaxNode};

use std::{any::type_name, path::PathBuf};
use tower_lsp::lsp_types::{Position, Range, Url};

pub struct CurrentDocument {
    uri: Url,
    root: SendNode,
    line_index: LineIndex,
    semantics: SemanticModel,
    diagnostics: Vec<ParseDiagnostic>,
}

impl CurrentDocument {
    pub fn new(uri: Url, text: &str, file_source: MFileSource) -> CurrentDocument {
        let parsed = parse(text, file_source);
        let diagnostics = parsed.diagnostics();

        Self::from_root(uri, file_source, text, parsed.syntax(), diagnostics)
    }

    pub fn from_root(
        uri: Url,
        file_source: MFileSource,
        text: &str,
        root: MSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let semantics = semantics(text, root.clone(), file_source);
        let root = root.as_send().unwrap_or_else(|| {
            panic!(
                "could not upcast root node from language {}",
                type_name::<MLanguage>()
            )
        });
        let line_index = LineIndex::new(text);
        let diagnostics = diagnostics.to_vec();

        CurrentDocument {
            uri,
            root,
            semantics,
            line_index,
            diagnostics,
        }
    }

    pub fn path(&self) -> PathBuf {
        self.uri.to_file_path().unwrap_or_default()
    }

    pub fn uri(&self) -> &Url {
        &self.uri
    }

    pub fn syntax(&self) -> SyntaxNode<MLanguage> {
        self.root.clone().into_node().unwrap_or_else(|| {
            panic!(
                "could not downcast root node to language {}",
                type_name::<MLanguage>()
            )
        })
    }

    pub fn definitions(&self) -> core::slice::Iter<'_, AnyMDefinition> {
        self.semantics.definitions()
    }

    pub fn line_index(&self) -> &LineIndex {
        &self.line_index
    }

    pub fn diagnostics(&self) -> Vec<(Range, String)> {
        let line_index = &self.line_index;
        self.diagnostics
            .iter()
            .filter_map(|error| {
                let text_range = error.location().span?;
                let LineColRange { start, end } = line_index.line_col_range(text_range)?;
                let range = Range::new(
                    Position::new(start.line, start.col),
                    Position::new(end.line, end.col),
                );
                let error = error.message.to_string();
                Some((range, error))
            })
            .collect::<Vec<_>>()
    }
}
