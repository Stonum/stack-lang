use super::position;
use m_lang::{
    semantic::{AnyMDefinition, Definition, SemanticModel as MLangSemanticModel},
    syntax::{ModuleKind, TextRange},
};

use ropey::Rope;
use serde_json::Value;
use std::path::PathBuf;
use tower_lsp::lsp_types::{
    CodeLens, Command, DocumentSymbolResponse, Location, SymbolInformation, SymbolKind, Url,
};

pub struct Document<T> {
    uri: Url,
    text: Rope,
    semantics: T,
}

impl<T> Document<T> {
    pub fn new(uri: Url, text: Rope, semantics: T) -> Document<T> {
        Document {
            uri,
            text,
            semantics,
        }
    }
    pub fn path(&self) -> PathBuf {
        self.uri.to_file_path().unwrap_or_default()
    }
    pub fn uri(&self) -> &Url {
        &self.uri
    }
    pub fn text(&self) -> &Rope {
        &self.text
    }
}

impl Document<MLangSemanticModel> {
    pub fn definitions(&self) -> &Vec<AnyMDefinition> {
        &self.semantics.definitions
    }
    pub fn kind(&self) -> ModuleKind {
        self.semantics.kind
    }
}

impl From<&Document<MLangSemanticModel>> for DocumentSymbolResponse {
    fn from(val: &Document<MLangSemanticModel>) -> Self {
        let mut response = vec![];

        let uri = val.uri();
        let text = val.text();

        let location = move |range: TextRange| {
            Location::new(uri.clone(), position(text, range).unwrap_or_default())
        };

        #[allow(deprecated)]
        for def in val.definitions() {
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
}

impl From<&Document<MLangSemanticModel>> for Vec<CodeLens> {
    fn from(val: &Document<MLangSemanticModel>) -> Self {
        let mut response = vec![];

        let text = val.text();

        let start_line =
            move |range: TextRange| position(text, range).unwrap_or_default().start.line;

        let command = String::from("stack.movetoLine");

        for def in val.definitions() {
            match def {
                AnyMDefinition::MClassDefinition(class) => {
                    let title = class.id();
                    let line = start_line(class.range());
                    let args = vec![Value::Number(line.into())];

                    let command = Some(Command::new(title, command.clone(), Some(args)));

                    let mut lenses = class
                        .methods()
                        .iter()
                        .filter_map(|m| {
                            let range = position(text, m.range())?;
                            Some(CodeLens {
                                range,
                                command: command.clone(),
                                data: None,
                            })
                        })
                        .collect::<Vec<_>>();
                    response.append(&mut lenses);
                }
                AnyMDefinition::MReportDefinition(report) => {
                    let title = report.id();
                    let line = start_line(report.range());
                    let args = vec![Value::Number(line.into())];

                    let command = Some(Command::new(title, command.clone(), Some(args)));

                    let mut lenses = report
                        .sections()
                        .iter()
                        .filter_map(|s| {
                            let range = position(text, s.range())?;
                            Some(CodeLens {
                                range,
                                command: command.clone(),
                                data: None,
                            })
                        })
                        .collect::<Vec<_>>();
                    response.append(&mut lenses);
                }
                _ => (),
            }
        }

        response
    }
}

fn trim_quotes_from_id(id: String) -> String {
    if id.starts_with('\'') && id.ends_with('\'') {
        return id[1..id.len() - 1].to_string();
    }
    id
}
