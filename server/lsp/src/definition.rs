use std::{path::PathBuf, sync::Arc};

use line_index::LineColRange;
use m_lang::semantic::{Definition, SemanticInfo, SemanticModel as MLangSemanticModel};
use tower_lsp::lsp_types::{Position, Range, Url};

#[derive(Debug)]
pub struct LspDefinition {
    pub uri: Url,
    pub range: Range,
    pub markup: String,
}

pub fn find_definitions<I>(
    identifier: &str,
    semantic_info: &SemanticInfo,
    semantics: I,
) -> Vec<LspDefinition>
where
    I: IntoIterator<Item = (PathBuf, Arc<MLangSemanticModel>)>,
{
    let mut definitions = vec![];

    let semantics = semantics.into_iter().collect::<Vec<_>>();

    match semantic_info {
        SemanticInfo::FunctionCall => {
            for (path, doc) in semantics.iter() {
                let uri = if let Ok(uri) = Url::from_file_path(path) {
                    uri
                } else {
                    continue;
                };
                let doc_def = doc.definitions();

                let mut functions = doc_def
                    .into_iter()
                    .filter(|d| d.is_function() && d.id().eq_ignore_ascii_case(identifier))
                    .map(|d| convert_to_lsp(d as &dyn Definition, uri.clone()))
                    .collect::<Vec<_>>();

                definitions.append(&mut functions);
            }
        }
        SemanticInfo::NewExpression => {
            for (path, doc) in semantics.iter() {
                let uri = if let Ok(uri) = Url::from_file_path(path) {
                    uri
                } else {
                    continue;
                };
                let doc_def = doc.definitions();

                let classes = doc_def
                    .into_iter()
                    .filter(|d| d.is_class() && d.id().eq_ignore_ascii_case(identifier));

                for c in classes {
                    definitions.push(convert_to_lsp(c as &dyn Definition, uri.clone()));

                    if let Some(methods) = c.methods() {
                        let mut constructors = methods
                            .into_iter()
                            .filter(|d| d.is_constructor())
                            .map(|d| convert_to_lsp(d as &dyn Definition, uri.clone()))
                            .collect::<Vec<_>>();

                        definitions.append(&mut constructors);
                    }
                }
            }
        }
        SemanticInfo::MethodCall(None) => {
            for (path, doc) in semantics.iter() {
                let uri = if let Ok(uri) = Url::from_file_path(path) {
                    uri
                } else {
                    continue;
                };
                let doc_def = doc.definitions();

                let classes = doc_def.into_iter().filter(|c| c.is_class());

                for c in classes {
                    if let Some(methods) = c.methods() {
                        let mut methods = methods
                            .into_iter()
                            .filter(|d| d.id().eq_ignore_ascii_case(identifier))
                            .map(|d| convert_to_lsp(d as &dyn Definition, uri.clone()))
                            .collect::<Vec<_>>();

                        if methods.len() > 0 {
                            definitions.push(convert_to_lsp(c as &dyn Definition, uri.clone()));
                        }

                        definitions.append(&mut methods);
                    }
                }
            }
        }
        SemanticInfo::MethodCall(Some(class_name)) => {
            let mut class_names = vec![class_name];

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                for (path, doc) in semantics.iter() {
                    let uri = if let Ok(uri) = Url::from_file_path(path) {
                        uri
                    } else {
                        continue;
                    };
                    let doc_def = doc.definitions();

                    let classes = doc_def
                        .into_iter()
                        .filter(|c| c.is_class() && classes_for_filter.contains(&&c.id()))
                        .collect::<Vec<_>>();

                    for class in classes {
                        if let Some(methods) = class.methods() {
                            let mut methods = methods
                                .into_iter()
                                .filter(|d| d.id().eq_ignore_ascii_case(identifier))
                                .map(|d| convert_to_lsp(d as &dyn Definition, uri.clone()))
                                .collect::<Vec<_>>();

                            if methods.len() > 0 {
                                definitions
                                    .push(convert_to_lsp(class as &dyn Definition, uri.clone()));
                            }

                            definitions.append(&mut methods);
                            if let Some(super_class) = class.extends() {
                                class_names.push(super_class);
                            }
                        }
                    }
                }

                if definitions.len() > 0 {
                    class_names.clear();
                }
            }
        }
    }

    definitions
}

fn convert_to_lsp(d: &dyn Definition, uri: Url) -> LspDefinition {
    let LineColRange { start, end } = d.id_range();
    LspDefinition {
        uri,
        range: Range::new(
            Position::new(start.line, start.col),
            Position::new(end.line, end.col),
        ),
        markup: d.to_markdown(),
    }
}
