use std::sync::Arc;

use line_index::LineColRange;
use m_lang::semantic::{
    AnyMDefinition, Definition, SemanticInfo, SemanticModel as MLangSemanticModel,
};
use tower_lsp::lsp_types::{Location, MarkedString, Position, Range, Url};

#[derive(Debug)]
pub struct LspDefinition {
    pub uri: Url,
    pub range: Range,
    pub markup: String,
}

fn to_location(uri: Url, range: LineColRange) -> Location {
    let LineColRange { start, end } = range;
    Location {
        uri: uri,
        range: Range::new(
            Position::new(start.line, start.col),
            Position::new(end.line, end.col),
        ),
    }
}

pub fn get_locations<I>(
    identifier: &str,
    semantic_info: &SemanticInfo,
    semantics: I,
) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, Arc<MLangSemanticModel>)>,
{
    let mut locations = vec![];

    match semantic_info {
        SemanticInfo::FunctionCall => {
            for (uri, doc) in semantics.into_iter() {
                let doc_def = doc.definitions();

                let mut functions = doc_def
                    .into_iter()
                    .filter(|d| d.is_function() && d.id().eq_ignore_ascii_case(identifier))
                    .map(|d| to_location(uri.clone(), d.id_range()))
                    .collect::<Vec<_>>();

                locations.append(&mut functions);
            }
        }
        SemanticInfo::NewExpression => {
            for (uri, doc) in semantics.into_iter() {
                let doc_def = doc.definitions();

                let classes = doc_def
                    .into_iter()
                    .filter(|d| d.is_class() && d.id().eq_ignore_ascii_case(identifier));

                for c in classes {
                    let mut constructors = doc_def
                        .into_iter()
                        .filter_map(|d| {
                            if !d.is_constructor() {
                                return None;
                            }
                            let container = d.container()?;
                            if &container != c {
                                return None;
                            }

                            Some(to_location(uri.clone(), d.id_range()))
                        })
                        .collect::<Vec<_>>();

                    if constructors.len() > 0 {
                        locations.append(&mut constructors);
                    } else {
                        locations.push(to_location(uri.clone(), c.id_range()));
                    }
                }
            }
        }
        SemanticInfo::MethodCall(None) => {
            for (uri, doc) in semantics.into_iter() {
                let doc_def = doc.definitions();

                let mut methods = doc_def
                    .into_iter()
                    .filter(|d| d.is_method() && d.id().eq_ignore_ascii_case(identifier))
                    .map(|d| to_location(uri.clone(), d.id_range()))
                    .collect::<Vec<_>>();

                locations.append(&mut methods);
            }
        }
        SemanticInfo::MethodCall(Some(class_name)) => {
            let semantics = semantics.into_iter().collect::<Vec<_>>();
            let mut class_names = vec![class_name];

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                for (uri, doc) in semantics.iter() {
                    let doc_def = doc.definitions();

                    let mut methods = doc_def
                        .into_iter()
                        // find by method name
                        .filter(|d| d.is_method() && d.id().eq_ignore_ascii_case(identifier))
                        // find by class name
                        .filter(|d| {
                            d.container().is_some_and(|c| {
                                classes_for_filter
                                    .iter()
                                    .any(|cff| c.id().eq_ignore_ascii_case(cff))
                            })
                        })
                        .map(|d| to_location(uri.clone(), d.id_range()))
                        .collect::<Vec<_>>();

                    if methods.len() > 0 {
                        locations.append(&mut methods);
                        break;
                    }

                    // append super classes
                    let mut super_classes = doc_def
                        .into_iter()
                        .filter_map(|d| {
                            if d.is_class()
                                && classes_for_filter
                                    .iter()
                                    .any(|cff| d.id().eq_ignore_ascii_case(cff))
                            {
                                return d.parent();
                            }
                            None
                        })
                        .collect::<Vec<_>>();
                    super_classes.dedup();
                    class_names.append(&mut super_classes);
                }
            }
        }
    }

    locations
}

pub fn get_hover<'a, I>(
    identifier: &str,
    semantic_info: &SemanticInfo,
    definitions: I,
) -> Vec<MarkedString>
where
    I: IntoIterator<Item = &'a AnyMDefinition>,
{
    match semantic_info {
        SemanticInfo::FunctionCall => {
            let functions = definitions
                .into_iter()
                .filter(|d| d.is_function() && d.id().eq_ignore_ascii_case(identifier))
                .map(|d| MarkedString::String(d.to_markdown()))
                .collect::<Vec<_>>();

            return functions;
        }
        SemanticInfo::NewExpression => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .filter(|d| d.is_class() && d.id().eq_ignore_ascii_case(identifier));

            for c in classes {
                markups.push(MarkedString::String(c.to_markdown()));

                let mut constructors = definitions
                    .iter()
                    .filter_map(|d| {
                        if !d.is_constructor() {
                            return None;
                        }
                        let container = d.container()?;
                        if &&container != c {
                            return None;
                        }

                        Some(MarkedString::String(d.to_markdown()))
                    })
                    .collect::<Vec<_>>();

                markups.append(&mut constructors);
            }
            return markups;
        }
        SemanticInfo::MethodCall(None) => {
            let methods = definitions
                .into_iter()
                .filter(|d| d.is_method() && d.id().eq_ignore_ascii_case(identifier))
                .map(|d| MarkedString::String(d.to_markdown()))
                .collect::<Vec<_>>();

            return methods;
        }
        SemanticInfo::MethodCall(Some(class_name)) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();
            let mut class_names = vec![class_name];

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                let mut methods = definitions
                    .iter()
                    // find by method name
                    .filter(|d| d.is_method() && d.id().eq_ignore_ascii_case(identifier))
                    // find by class name
                    .filter(|d| {
                        d.container().is_some_and(|c| {
                            classes_for_filter
                                .iter()
                                .any(|cff| c.id().eq_ignore_ascii_case(cff))
                        })
                    })
                    .map(|d| MarkedString::String(d.to_markdown()))
                    .collect::<Vec<_>>();

                if methods.len() > 0 {
                    markups.append(&mut methods);
                    break;
                }

                // append super classes
                let mut super_classes = definitions
                    .iter()
                    .filter_map(|d| {
                        if d.is_class()
                            && classes_for_filter
                                .iter()
                                .any(|cff| d.id().eq_ignore_ascii_case(cff))
                        {
                            return d.parent();
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                super_classes.dedup();
                class_names.append(&mut super_classes);
            }

            return markups;
        }
    }
}
