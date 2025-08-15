use std::sync::Arc;

use line_index::LineColRange;
use tower_lsp::lsp_types::{Location, MarkedString, Position, Range, Url};

pub trait CodeSymbolDefinition: Sized + PartialEq {
    fn is_function(&self) -> bool {
        false
    }
    fn is_class(&self) -> bool {
        false
    }
    fn is_method(&self) -> bool {
        false
    }
    fn is_constructor(&self) -> bool {
        false
    }
    fn id(&self) -> &str;
    fn container(&self) -> Option<Self>;
    fn parent(&self) -> Option<&str>;
    fn compare_with(&self, another: &str) -> bool {
        unicase::eq(self.id(), another)
    }
}

pub trait LocationDefinition {
    fn range(&self) -> LineColRange;
    fn lsp_range(&self) -> Range {
        let LineColRange { start, end } = self.range();
        Range::new(
            Position::new(start.line, start.col),
            Position::new(end.line, end.col),
        )
    }
    fn location(&self, uri: Url) -> Location {
        Location {
            uri: uri,
            range: self.lsp_range(),
        }
    }
    fn id_range(&self) -> LineColRange {
        self.range()
    }
    fn id_lsp_range(&self) -> Range {
        let LineColRange { start, end } = self.id_range();
        Range::new(
            Position::new(start.line, start.col),
            Position::new(end.line, end.col),
        )
    }
    fn id_location(&self, uri: Url) -> Location {
        Location {
            uri: uri,
            range: self.id_lsp_range(),
        }
    }
}

pub trait MarkupDefinition {
    fn markdown(&self) -> String;
}

pub type Identifier = String;
pub type Class = String;

#[derive(Debug, Eq, PartialEq)]
pub enum SemanticInfo {
    // like zzzzz();
    // contains function name
    FunctionCall(Identifier),

    // like z.cmethod() or this.method()
    // contains method name and optionally contains class name
    MethodCall(Identifier, Option<Class>),

    // like new MyClass();
    // contains class name
    NewExpression(Identifier),

    // like class A extends B
    // contains class name
    ClassExtends(Identifier),

    // like super(x);
    // contains super class name
    SuperCall(Identifier, Class),
}

pub fn get_locations<'a, I, D>(semantic_info: &SemanticInfo, semantics: I) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, Arc<Vec<D>>)>,
    D: CodeSymbolDefinition + LocationDefinition + 'a,
{
    let mut locations = vec![];

    match semantic_info {
        SemanticInfo::FunctionCall(ident) => {
            for (uri, definitions) in semantics.into_iter() {
                let mut functions = definitions
                    .iter()
                    .filter(|d| d.is_function() && d.compare_with(ident))
                    .map(|d| d.id_location(uri.clone()))
                    .collect::<Vec<_>>();

                locations.append(&mut functions);
            }
        }
        SemanticInfo::NewExpression(ident) => {
            for (uri, definitions) in semantics.into_iter() {
                let classes = definitions
                    .iter()
                    .filter(|d| d.is_class() && d.compare_with(ident));

                for c in classes {
                    let mut constructors = definitions
                        .iter()
                        .filter_map(|d| {
                            if !d.is_constructor() {
                                return None;
                            }
                            let container = d.container()?;
                            // if &container != c {
                            return None;
                            // }

                            Some(d.id_location(uri.clone()))
                        })
                        .collect::<Vec<_>>();

                    if constructors.len() > 0 {
                        locations.append(&mut constructors);
                    } else {
                        locations.push(c.id_location(uri.clone()));
                    }
                }
            }
        }
        SemanticInfo::ClassExtends(ident) => {
            for (uri, definitions) in semantics.into_iter() {
                let mut classes = definitions
                    .iter()
                    .filter(|d| d.is_class() && d.compare_with(ident))
                    .map(|d| d.id_location(uri.clone()))
                    .collect::<Vec<_>>();

                locations.append(&mut classes);
            }
        }
        SemanticInfo::MethodCall(ident, None) => {
            for (uri, definitions) in semantics.into_iter() {
                let mut methods = definitions
                    .iter()
                    .filter(|d| d.is_method() && d.compare_with(ident))
                    .map(|d| d.id_location(uri.clone()))
                    .collect::<Vec<_>>();

                locations.append(&mut methods);
            }
        }
        SemanticInfo::MethodCall(ident, Some(class_name)) => {
            let semantics = semantics.into_iter().collect::<Vec<_>>();
            let mut class_names: Vec<&str> = vec![class_name];

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                for (uri, definitions) in semantics.iter() {
                    let mut methods = definitions
                        .iter()
                        // find by method name
                        .filter(|d| d.is_method() && d.compare_with(ident))
                        // find by class name
                        .filter(|d| {
                            d.container().is_some_and(|c| {
                                classes_for_filter.iter().any(|cff| c.compare_with(cff))
                            })
                        })
                        .map(|d| d.id_location(uri.clone()))
                        .collect::<Vec<_>>();

                    if methods.len() > 0 {
                        locations.append(&mut methods);
                        break;
                    }

                    // append super classes
                    let mut super_classes = definitions
                        .iter()
                        .filter_map(|d| {
                            if d.is_class()
                                && classes_for_filter.iter().any(|cff| d.compare_with(cff))
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
        SemanticInfo::SuperCall(_ident, class_name) => {
            for (uri, definitions) in semantics.into_iter() {
                let classes = definitions
                    .iter()
                    .filter(|d| d.is_class() && d.compare_with(class_name));

                for c in classes {
                    let mut constructors = definitions
                        .iter()
                        .filter_map(|d| {
                            if !d.is_constructor() {
                                return None;
                            }
                            let container = d.container()?;
                            // if &container != c {
                            return None;
                            // }

                            Some(d.id_location(uri.clone()))
                        })
                        .collect::<Vec<_>>();

                    if constructors.len() > 0 {
                        locations.append(&mut constructors);
                    } else {
                        locations.push(c.id_location(uri.clone()));
                    }
                }
            }
        }
    }

    locations
}

pub fn get_hover<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<MarkedString>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolDefinition + MarkupDefinition + 'a,
{
    match semantic_info {
        SemanticInfo::FunctionCall(ident) => {
            let functions = definitions
                .into_iter()
                .filter(|d| d.is_function() && d.compare_with(ident))
                .map(|d| MarkedString::String(d.markdown()))
                .collect::<Vec<_>>();

            return functions;
        }
        SemanticInfo::NewExpression(ident) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .filter(|d| d.is_class() && d.compare_with(ident));

            for c in classes {
                markups.push(MarkedString::String(c.markdown()));

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

                        Some(MarkedString::String(d.markdown()))
                    })
                    .collect::<Vec<_>>();

                markups.append(&mut constructors);
            }
            return markups;
        }
        SemanticInfo::ClassExtends(ident) => {
            let classes = definitions
                .into_iter()
                .filter(|d| d.is_class() && d.compare_with(ident))
                .map(|d| MarkedString::String(d.markdown()))
                .collect::<Vec<_>>();

            return classes;
        }
        SemanticInfo::MethodCall(ident, None) => {
            let methods = definitions
                .into_iter()
                .filter(|d| d.is_method() && d.compare_with(ident))
                .map(|d| MarkedString::String(d.markdown()))
                .collect::<Vec<_>>();

            return methods;
        }
        SemanticInfo::MethodCall(ident, Some(class_name)) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();
            let mut class_names: Vec<&str> = vec![class_name];

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                let mut methods = definitions
                    .iter()
                    // find by method name
                    .filter(|d| d.is_method() && d.compare_with(ident))
                    // find by class name
                    .filter(|d| {
                        d.container().is_some_and(|c| {
                            classes_for_filter.iter().any(|cff| c.compare_with(cff))
                        })
                    })
                    .map(|d| MarkedString::String(d.markdown()))
                    .collect::<Vec<_>>();

                if methods.len() > 0 {
                    markups.append(&mut methods);
                    break;
                }

                // append super classes
                let mut super_classes = definitions
                    .iter()
                    .filter_map(|d| {
                        if d.is_class() && classes_for_filter.iter().any(|cff| d.compare_with(cff))
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
        SemanticInfo::SuperCall(_ident, class_name) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .filter(|d| d.is_class() && d.compare_with(class_name));

            for c in classes {
                markups.push(MarkedString::String(c.markdown()));

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

                        Some(MarkedString::String(d.markdown()))
                    })
                    .collect::<Vec<_>>();

                markups.append(&mut constructors);
            }
            return markups;
        }
    }
}
