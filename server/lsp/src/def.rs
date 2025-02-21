use m_lang::{
    semantic::{AnyMDefinition, Definition},
    syntax::TextRange,
};
use tower_lsp::lsp_types::{DocumentSymbolResponse, Location, SymbolInformation, SymbolKind, Url};

use super::position;

pub fn from_mlang(
    file: Url,
    definitions: &Vec<AnyMDefinition>,
    rope: &ropey::Rope,
) -> DocumentSymbolResponse {
    let mut response = vec![];

    let location = move |range: TextRange| {
        Location::new(file.clone(), position(rope, range).unwrap_or_default())
    };

    for def in definitions {
        match def {
            AnyMDefinition::MFunctionDefinition(function) => {
                let func_def = SymbolInformation {
                    name: function.id(),
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
                    name: class.id(),
                    kind: SymbolKind::CLASS,
                    tags: None,
                    deprecated: None,
                    location: location(class.range()),
                    container_name: None,
                };
                response.push(class_def);

                for method in class.methods() {
                    let method_def = SymbolInformation {
                        name: method.id(),
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
