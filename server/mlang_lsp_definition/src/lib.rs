use line_index::LineColRange;
use tower_lsp::lsp_types::{Location, MarkedString, Position, Range, SymbolInformation, Url};

pub use tower_lsp::lsp_types::SymbolKind;

pub struct StringLowerCase(String);
impl StringLowerCase {
    pub fn new(s: &str) -> Self {
        StringLowerCase(s.to_lowercase())
    }
}

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
    fn partial_compare_with(&self, another: &StringLowerCase) -> bool {
        self.id().to_lowercase().contains(&another.0)
    }
}

pub trait CodeSymbolInformation: CodeSymbolDefinition {
    fn symbol_kind(&self) -> SymbolKind;
    fn symbol_name(&self) -> String {
        let mut name = self.id();
        if name.starts_with('\'') && name.ends_with('\'') {
            name = &name[1..name.len() - 1];
        } else if name.starts_with('\"') && name.ends_with('\"') {
            name = &name[1..name.len() - 1];
        }
        name.to_string()
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
            uri,
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
            uri,
            range: self.id_lsp_range(),
        }
    }
}

const SPECIAL_CHARS: [char; 2] = ['\\', '#'];

pub trait MarkupDefinition {
    fn markdown(&self) -> String;

    fn escape_markdown_with_newlines(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len() * 2);

        for c in s.chars() {
            match c {
                '\r' => {}
                '\n' => {
                    result.push_str("  \n");
                }
                _ if SPECIAL_CHARS.contains(&c) => {
                    result.push('\\');
                    result.push(c);
                }
                _ => result.push(c),
            }
        }
        result
    }
}

pub type Identifier = String;
pub type Class = String;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum SemanticInfo {
    // function zzzzz();
    // contains function name
    FunctionDeclaration(Identifier),

    // function class x {}
    // contains class name
    ClassDeclaration(Identifier),

    // cmenthod() { }
    // contains method and class name
    MethodDeclaration(Identifier, Class),

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

pub fn get_declaration<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + LocationDefinition + 'a,
{
    let mut locations = vec![];

    match semantic_info {
        SemanticInfo::FunctionCall(ident) | SemanticInfo::FunctionDeclaration(ident) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_function() && d.compare_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),
        SemanticInfo::NewExpression(ident) | SemanticInfo::ClassDeclaration(ident) => {
            let definitions = Vec::from_iter(definitions);
            let classes = definitions
                .iter()
                .filter(|(_, d)| d.is_class() && d.compare_with(ident));
            for (uri, class) in classes {
                let mut constructors = definitions
                    .iter()
                    .filter_map(|(uri, d)| {
                        if !d.is_constructor() {
                            return None;
                        }
                        let container = d.container()?;
                        if &&container != class {
                            return None;
                        }

                        Some(d.id_location(uri.clone()))
                    })
                    .collect::<Vec<_>>();

                if !constructors.is_empty() {
                    locations.append(&mut constructors);
                } else {
                    locations.push(class.id_location(uri.clone()));
                }
            }
            locations
        }
        SemanticInfo::ClassExtends(ident) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_class() && d.compare_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),
        SemanticInfo::MethodCall(ident, None) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_method() && d.compare_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),
        SemanticInfo::MethodCall(ident, Some(class_name))
        | SemanticInfo::MethodDeclaration(ident, class_name) => {
            let mut class_names: Vec<String> = vec![class_name.to_owned()];

            // local copy
            let definitions = Vec::from_iter(definitions);

            while !class_names.is_empty() {
                let classes_for_filter = class_names.clone();
                class_names.clear();

                let mut methods = definitions
                    .iter()
                    // find by method name
                    .filter(|(_, d)| d.is_method() && d.compare_with(ident))
                    // find by class name
                    .filter(|(_, d)| {
                        d.container().is_some_and(|c| {
                            classes_for_filter.iter().any(|cff| c.compare_with(cff))
                        })
                    })
                    .map(|(uri, d)| d.id_location(uri.clone()))
                    .collect::<Vec<_>>();

                if !methods.is_empty() {
                    locations.append(&mut methods);
                    break;
                }

                // append super classes
                let mut super_classes = definitions
                    .iter()
                    .filter_map(|(_, d)| {
                        if d.is_class() && classes_for_filter.iter().any(|cff| d.compare_with(cff))
                        {
                            return d.parent().map(str::to_string);
                        }
                        None
                    })
                    .collect::<Vec<_>>();

                super_classes.dedup();
                class_names.append(&mut super_classes);
            }

            locations
        }
        SemanticInfo::SuperCall(_ident, class_name) => {
            // local copy
            let definitions = Vec::from_iter(definitions);

            let classes = definitions
                .iter()
                .filter(|(_, d)| d.is_class() && d.compare_with(class_name));

            for (uri, class) in classes {
                let mut constructors = definitions
                    .iter()
                    .filter_map(|(uri, d)| {
                        if !d.is_constructor() {
                            return None;
                        }
                        let container = d.container()?;
                        if &&container != class {
                            return None;
                        }

                        Some(d.id_location(uri.clone()))
                    })
                    .collect::<Vec<_>>();

                if !constructors.is_empty() {
                    locations.append(&mut constructors);
                } else {
                    locations.push(class.id_location(uri.clone()));
                }
            }

            locations
        }
    }
}

pub fn get_reference<'a, I, R>(
    semantic_info: &SemanticInfo,
    uri: &Url,
    references: I,
) -> Vec<Location>
where
    I: IntoIterator<Item = (&'a SemanticInfo, &'a Vec<R>)>,
    R: LocationDefinition + 'a,
{
    match semantic_info {
        SemanticInfo::FunctionCall(ident) | SemanticInfo::FunctionDeclaration(ident) => {
            let call_info = SemanticInfo::FunctionCall(ident.clone());
            references
                .into_iter()
                .filter(|(info, _)| info.eq(&&call_info))
                .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
                .collect::<Vec<_>>()
        }

        SemanticInfo::NewExpression(ident) | SemanticInfo::ClassDeclaration(ident) => {
            let call_info = SemanticInfo::NewExpression(ident.clone());
            references
                .into_iter()
                .filter(|(info, _)| info.eq(&&call_info))
                .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
                .collect::<Vec<_>>()
        }

        SemanticInfo::MethodCall(ident, Some(class_name))
        | SemanticInfo::MethodDeclaration(ident, class_name) => {
            let call_info = (
                SemanticInfo::MethodCall(ident.clone(), None),
                SemanticInfo::MethodCall(ident.clone(), Some(class_name.clone())),
            );
            references
                .into_iter()
                .filter(|(info, _)| info.eq(&&call_info.0) || info.eq(&&call_info.1))
                .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
                .collect::<Vec<_>>()
        }

        SemanticInfo::MethodCall(ident, None) => {
            let call_info = SemanticInfo::MethodCall(ident.clone(), None);
            references
                .into_iter()
                .filter(|(info, _)| info.eq(&&call_info) || info.eq(&semantic_info))
                .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
                .collect::<Vec<_>>()
        }
        _ => vec![],
    }
}

pub fn get_hover<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<MarkedString>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolDefinition + MarkupDefinition + 'a,
{
    match semantic_info {
        SemanticInfo::FunctionCall(ident) | SemanticInfo::FunctionDeclaration(ident) => definitions
            .into_iter()
            .filter(|d| d.is_function() && d.compare_with(ident))
            .map(|d| MarkedString::String(d.markdown()))
            .collect::<Vec<_>>(),
        SemanticInfo::NewExpression(ident) | SemanticInfo::ClassDeclaration(ident) => {
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
            markups
        }
        SemanticInfo::ClassExtends(ident) => definitions
            .into_iter()
            .filter(|d| d.is_class() && d.compare_with(ident))
            .map(|d| MarkedString::String(d.markdown()))
            .collect::<Vec<_>>(),
        SemanticInfo::MethodCall(ident, None) => definitions
            .into_iter()
            .filter(|d| d.is_method() && d.compare_with(ident))
            .map(|d| MarkedString::String(d.markdown()))
            .collect::<Vec<_>>(),
        SemanticInfo::MethodCall(ident, Some(class_name))
        | SemanticInfo::MethodDeclaration(ident, class_name) => {
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

                if !methods.is_empty() {
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

            markups
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
            markups
        }
    }
}

pub fn get_symbols<'a, I, D>(uri: &Url, definitions: I) -> Vec<SymbolInformation>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolInformation + LocationDefinition + 'a,
{
    definitions
        .into_iter()
        .map(|def| {
            #[allow(deprecated)]
            SymbolInformation {
                name: def.symbol_name(),
                kind: def.symbol_kind(),
                tags: None,
                deprecated: None,
                location: def.location(uri.clone()),
                container_name: def.container().map(|c| c.symbol_name()),
            }
        })
        .collect::<Vec<_>>()
}
