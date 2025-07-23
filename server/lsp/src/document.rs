use biome_diagnostics::diagnostic::Diagnostic as _;
use line_index::{LineColRange, LineIndex};
use m_lang::{
    parser::{ParseDiagnostic, parse},
    semantic::{AnyMDefinition, Definition, SemanticModel as MLangSemanticModel, semantics},
    syntax::{MFileSource, MLanguage, MSyntaxNode, SendNode, SyntaxNode},
};

use serde_json::Value;
use std::{any::type_name, path::PathBuf};
use tower_lsp::lsp_types::{
    CodeLens, Command, DocumentSymbolResponse, Location, Position, Range, SymbolInformation,
    SymbolKind, Url,
};

pub fn document_symbol_response(
    uri: &Url,
    semantic: &MLangSemanticModel,
) -> DocumentSymbolResponse {
    let mut response = vec![];

    let location = |range: LineColRange| {
        let LineColRange { start, end } = range;
        Location::new(
            uri.clone(),
            Range::new(
                Position::new(start.line, start.col),
                Position::new(end.line, end.col),
            ),
        )
    };

    #[allow(deprecated)]
    for def in semantic.definitions() {
        match def {
            AnyMDefinition::MFunctionDefinition(function) => {
                let func_def = SymbolInformation {
                    name: trim_quotes_from_id(function.id()),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    location: location(function.range()),
                    container_name: None,
                };
                response.push(func_def);
            }
            AnyMDefinition::MClassDefinition(class) => {
                let class_def = SymbolInformation {
                    name: trim_quotes_from_id(class.id()),
                    kind: SymbolKind::CLASS,
                    tags: None,
                    deprecated: None,
                    location: location(class.range()),
                    container_name: None,
                };
                response.push(class_def);

                for method in class.methods() {
                    let method_def = SymbolInformation {
                        name: trim_quotes_from_id(method.id()),
                        kind: SymbolKind::FUNCTION,
                        tags: None,
                        deprecated: None,
                        location: location(method.range()),
                        container_name: Some(class.id()),
                    };
                    response.push(method_def);
                }
            }
            AnyMDefinition::MReportDefinition(report) => {
                let rep_def = SymbolInformation {
                    name: report.id(),
                    kind: SymbolKind::CONSTANT,
                    tags: None,
                    deprecated: None,
                    location: location(report.range()),
                    container_name: None,
                };
                response.push(rep_def);

                for section in report.sections() {
                    let section_def = SymbolInformation {
                        name: section.id(),
                        kind: SymbolKind::FIELD,
                        tags: None,
                        deprecated: None,
                        location: location(section.range()),
                        container_name: Some(report.id()),
                    };
                    response.push(section_def);
                }
            }
        }
    }

    DocumentSymbolResponse::Flat(response)
}

pub fn code_lens(semantic: &MLangSemanticModel) -> Vec<CodeLens> {
    let mut response = vec![];

    let command = String::from("stack.movetoLine");

    let lsp_range = |range: LineColRange| {
        Range::new(
            Position::new(range.start.line, range.start.col),
            Position::new(range.end.line, range.end.col),
        )
    };

    for def in semantic.definitions() {
        match def {
            AnyMDefinition::MClassDefinition(class) => {
                let title = class.id();
                let line = class.range().start.line;
                let args = vec![Value::Number(line.into())];

                let command = Some(Command::new(title, command.clone(), Some(args)));

                let mut lenses = class
                    .methods()
                    .iter()
                    .map(|m| CodeLens {
                        range: lsp_range(m.range()),
                        command: command.clone(),
                        data: None,
                    })
                    .collect::<Vec<_>>();
                response.append(&mut lenses);
            }
            AnyMDefinition::MReportDefinition(report) => {
                let title = report.id();
                let line = report.range().start.line;
                let args = vec![Value::Number(line.into())];

                let command = Some(Command::new(title, command.clone(), Some(args)));

                let mut lenses = report
                    .sections()
                    .iter()
                    .map(|s| CodeLens {
                        range: lsp_range(s.range()),
                        command: command.clone(),
                        data: None,
                    })
                    .collect::<Vec<_>>();
                response.append(&mut lenses);
            }
            _ => (),
        }
    }

    response
}

fn trim_quotes_from_id(id: String) -> String {
    if id.starts_with('\'') && id.ends_with('\'') {
        return id[1..id.len() - 1].to_string();
    }
    id
}

pub struct CurrentDocument {
    uri: Url,
    root: SendNode,
    line_index: LineIndex,
    semantics: MLangSemanticModel,
    diagnostics: Vec<ParseDiagnostic>,
}

impl CurrentDocument {
    pub fn new(uri: Url, text: &String, file_source: MFileSource) -> CurrentDocument {
        let parsed = parse(&text, file_source);
        let diagnostics = parsed.diagnostics();

        Self::from_root(uri, text, parsed.syntax(), diagnostics)
    }

    pub fn from_root(
        uri: Url,
        text: &String,
        root: MSyntaxNode,
        diagnostics: &[ParseDiagnostic],
    ) -> CurrentDocument {
        let semantics = semantics(&text, root.clone());
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

    pub fn semantics(&self) -> &MLangSemanticModel {
        &self.semantics
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
