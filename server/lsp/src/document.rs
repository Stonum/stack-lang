use super::position;
use m_lang::{
    semantic::{AnyMDefinition, Definition, SemanticModel as MLangSemanticModel},
    syntax::TextRange,
};

use ropey::Rope;
use std::path::PathBuf;
use tower_lsp::lsp_types::{DocumentSymbolResponse, Location, SymbolInformation, SymbolKind, Url};

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
        &self.semantics.module_definitions
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
                            container_name: None,
                        };
                        response.push(method_def);
                    }
                }
            }
        }

        DocumentSymbolResponse::Flat(response)
    }
}

fn trim_quotes_from_id(id: String) -> String {
    if id.starts_with('\'') && id.ends_with('\'') {
        return id[1..id.len() - 1].to_string();
    }
    id
}
