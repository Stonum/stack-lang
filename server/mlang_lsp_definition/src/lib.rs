use line_index::LineColRange;
use std::{collections::HashMap, ops::Not};
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, Location, MarkedString, MarkupContent,
    MarkupKind, Position, Range, SymbolInformation, Url,
};

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
    fn is_getter(&self) -> bool {
        false
    }
    fn is_setter(&self) -> bool {
        false
    }
    fn is_constructor(&self) -> bool {
        false
    }
    fn id(&self) -> &str;
    fn parameters(&self) -> Option<&str>;
    fn container(&self) -> Option<Self>;
    fn parent(&self) -> Option<&str>;

    fn can_be_overridden(&self, another: &Self) -> bool {
        if self.compare_id_with(another.id()) {
            return false;
        }

        self.parameters() == another.parameters()
    }

    fn compare_parameters_with(&self, count: usize) -> bool;

    fn compare_id_with(&self, another: &str) -> bool {
        unicase::eq(self.id(), another)
    }
    fn partial_compare_id_with(&self, another: &StringLowerCase) -> bool {
        self.id().to_lowercase().contains(&another.0)
    }
}

pub trait CodeSymbolInformation: CodeSymbolDefinition {
    fn symbol_kind(&self) -> SymbolKind;
    fn symbol_name(&self) -> String {
        let mut name = self.id();

        let double_quoted = name.starts_with('\"') && name.ends_with('\"');
        let single_quoted = name.starts_with('\'') && name.ends_with('\'');

        if double_quoted || single_quoted {
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
pub type ParametersCount = usize;

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
    FunctionCall(Identifier, ParametersCount),

    // like z.cmethod() or this.method()
    // contains method name and optionally contains class name
    MethodCall(Identifier, ParametersCount, Option<Class>),

    // like new MyClass();
    // contains class name
    NewExpression(Option<Identifier>, ParametersCount),

    // like class A extends B
    // contains class name
    ClassExtends(Identifier),

    // like super(x);
    // contains super class name
    SuperCall(Identifier, ParametersCount, Class),

    // like this or other refs on class instance
    // contains class name
    RefClass(Class),

    // like z
    // where z is reference for result function
    // contains function identifier
    RefFunctionResult(Identifier, ParametersCount),

    // like z
    // where z is reference for result any method
    // contains method identifier and class name
    RefMethodResult(Identifier, ParametersCount, Option<Class>),
}

pub fn get_declaration<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + LocationDefinition + 'a,
{
    let mut locations = vec![];

    match semantic_info {
        SemanticInfo::FunctionDeclaration(ident) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_function() && d.compare_id_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),

        SemanticInfo::FunctionCall(ident, params)
        | SemanticInfo::RefFunctionResult(ident, params) => definitions
            .into_iter()
            .filter(|(_, d)| {
                d.is_function() && d.compare_id_with(ident) && d.compare_parameters_with(*params)
            })
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),

        SemanticInfo::NewExpression(None, _params) => locations,

        SemanticInfo::ClassDeclaration(ident) => {
            let definitions = Vec::from_iter(definitions);
            let classes = definitions
                .iter()
                .filter(|(_, d)| d.is_class() && d.compare_id_with(ident));
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

        SemanticInfo::NewExpression(Some(ident), params) => {
            let definitions = Vec::from_iter(definitions);
            let classes = definitions
                .iter()
                .filter(|(_, d)| d.is_class() && d.compare_id_with(ident));
            for (uri, class) in classes {
                let mut constructors = definitions
                    .iter()
                    .filter_map(|(uri, d)| {
                        if !d.is_constructor() || !d.compare_parameters_with(*params) {
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

        SemanticInfo::RefClass(ident) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_class() && d.compare_id_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),

        SemanticInfo::ClassExtends(ident) => definitions
            .into_iter()
            .filter(|(_, d)| d.is_class() && d.compare_id_with(ident))
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),

        SemanticInfo::MethodCall(ident, params, None)
        | SemanticInfo::RefMethodResult(ident, params, None) => definitions
            .into_iter()
            .filter(|(_, d)| {
                d.is_method() && d.compare_id_with(ident) && d.compare_parameters_with(*params)
            })
            .map(|(uri, d)| d.id_location(uri.clone()))
            .collect::<Vec<_>>(),

        SemanticInfo::MethodDeclaration(ident, class_name) => {
            let members = get_class_members_definition(definitions, class_name);

            members
                .into_iter()
                .filter(|(_, d)| d.is_method() && d.compare_id_with(ident))
                .map(|(uri, d)| d.id_location(uri.clone()))
                .collect::<Vec<_>>()
        }

        SemanticInfo::MethodCall(ident, params, Some(class_name))
        | SemanticInfo::RefMethodResult(ident, params, Some(class_name)) => {
            let members = get_class_members_definition(definitions, class_name);

            members
                .into_iter()
                .filter(|(_, d)| {
                    d.is_method() && d.compare_id_with(ident) && d.compare_parameters_with(*params)
                })
                .map(|(uri, d)| d.id_location(uri.clone()))
                .collect::<Vec<_>>()
        }

        SemanticInfo::SuperCall(_ident, params, class_name) => {
            // local copy
            let definitions = Vec::from_iter(definitions);

            let classes = definitions
                .iter()
                .filter(|(_, d)| d.is_class() && d.compare_id_with(class_name));

            for (uri, class) in classes {
                let mut constructors = definitions
                    .iter()
                    .filter_map(|(uri, d)| {
                        if !d.is_constructor() || !d.compare_parameters_with(*params) {
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
        SemanticInfo::FunctionDeclaration(ident) => references
            .into_iter()
            .filter(|(info, _)| {
                if let SemanticInfo::FunctionCall(func, _) = info {
                    return func.eq(ident);
                }
                false
            })
            .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
            .collect::<Vec<_>>(),

        SemanticInfo::ClassDeclaration(ident) => references
            .into_iter()
            .filter(|(info, _)| {
                if let SemanticInfo::NewExpression(Some(class), _) = info {
                    return class.eq(ident);
                }
                false
            })
            .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
            .collect::<Vec<_>>(),

        SemanticInfo::MethodDeclaration(ident, class_name) => references
            .into_iter()
            .filter(|(info, _)| {
                if let SemanticInfo::MethodCall(method, _, Some(class)) = info {
                    return method.eq(ident) && class.eq(class_name);
                }
                if let SemanticInfo::MethodCall(method, _, None) = info {
                    return method.eq(ident);
                }
                false
            })
            .flat_map(|(_, refs)| refs.iter().map(|r| r.location(uri.clone())))
            .collect::<Vec<_>>(),

        SemanticInfo::SuperCall(_, _, _) | SemanticInfo::ClassExtends(_) => vec![],
        SemanticInfo::NewExpression(_, _) => vec![],

        SemanticInfo::FunctionCall(_, _) | SemanticInfo::MethodCall(_, _, _) => vec![],

        SemanticInfo::RefFunctionResult(_, _)
        | SemanticInfo::RefClass(_)
        | SemanticInfo::RefMethodResult(_, _, _) => vec![],
    }
}

pub fn get_hover<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<MarkedString>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + 'a,
{
    match semantic_info {
        SemanticInfo::FunctionDeclaration(ident) => definitions
            .into_iter()
            .map(|(_, d)| d)
            .filter(|d| d.is_function() && d.compare_id_with(ident))
            .map(|d| MarkedString::String(d.markdown()))
            .collect::<Vec<_>>(),

        SemanticInfo::FunctionCall(ident, params)
        | SemanticInfo::RefFunctionResult(ident, params) => {
            let mut functions = definitions
                .into_iter()
                .map(|(_, d)| d)
                .filter(|d| d.is_function() && d.compare_id_with(ident))
                .map(|d| {
                    (
                        MarkedString::String(d.markdown()),
                        d.compare_parameters_with(*params), // <-- sort priority
                    )
                })
                .collect::<Vec<_>>();
            functions.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            functions.into_iter().map(|(f, _)| f).collect::<Vec<_>>()
        }

        SemanticInfo::ClassDeclaration(ident) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .map(|(_, d)| d)
                .filter(|d| d.is_class() && d.compare_id_with(ident));

            for c in classes {
                markups.push(MarkedString::String(c.markdown()));

                let mut constructors = definitions
                    .iter()
                    .map(|(_, d)| d)
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

        SemanticInfo::NewExpression(None, _params) => vec![],

        SemanticInfo::NewExpression(Some(ident), params) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .map(|(_, d)| d)
                .filter(|d| d.is_class() && d.compare_id_with(ident));

            for c in classes {
                markups.push(MarkedString::String(c.markdown()));

                let mut constructors = definitions
                    .iter()
                    .filter_map(|(_, d)| {
                        if !d.is_constructor() {
                            return None;
                        }
                        let container = d.container()?;
                        if &&container != c {
                            return None;
                        }

                        Some((
                            MarkedString::String(d.markdown()),
                            d.compare_parameters_with(*params), // <-- sort priority
                        ))
                    })
                    .collect::<Vec<_>>();

                constructors.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                markups.extend(constructors.into_iter().map(|(c, _)| c));
            }

            markups
        }

        SemanticInfo::ClassExtends(ident) | SemanticInfo::RefClass(ident) => definitions
            .into_iter()
            .map(|(_, d)| d)
            .filter(|d| d.is_class() && d.compare_id_with(ident))
            .map(|d| MarkedString::String(d.markdown()))
            .collect::<Vec<_>>(),

        SemanticInfo::MethodCall(ident, params, None)
        | SemanticInfo::RefMethodResult(ident, params, None) => {
            let mut methods = definitions
                .into_iter()
                .map(|(_, d)| d)
                .filter(|d| d.is_method() && d.compare_id_with(ident))
                .map(|d| {
                    (
                        MarkedString::String(d.markdown()),
                        d.compare_parameters_with(*params), // <-- sort priority
                    )
                })
                .collect::<Vec<_>>();
            methods.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            methods.into_iter().map(|(m, _)| m).collect::<Vec<_>>()
        }

        SemanticInfo::MethodDeclaration(ident, class_name) => {
            let members = get_class_members_definition(definitions, class_name);

            members
                .into_iter()
                .filter(|(_, d)| d.is_method() && d.compare_id_with(ident))
                .map(|(_, d)| MarkedString::String(d.markdown()))
                .collect::<Vec<_>>()
        }

        SemanticInfo::MethodCall(ident, params, Some(class_name))
        | SemanticInfo::RefMethodResult(ident, params, Some(class_name)) => {
            let members = get_class_members_definition(definitions, class_name);

            let mut methods = members
                .into_iter()
                .filter(|(_, d)| d.is_method() && d.compare_id_with(ident))
                .map(|(_, d)| {
                    (
                        MarkedString::String(d.markdown()),
                        d.compare_parameters_with(*params), // <-- sort priority
                    )
                })
                .collect::<Vec<_>>();

            methods.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            methods.into_iter().map(|(m, _)| m).collect::<Vec<_>>()
        }

        SemanticInfo::SuperCall(_ident, params, class_name) => {
            let mut markups = vec![];

            let definitions = definitions.into_iter().collect::<Vec<_>>();

            let classes = definitions
                .iter()
                .map(|(_, d)| d)
                .filter(|d| d.is_class() && d.compare_id_with(class_name));

            for c in classes {
                markups.push(MarkedString::String(c.markdown()));

                let mut constructors = definitions
                    .iter()
                    .filter_map(|(_, d)| {
                        if !d.is_constructor() {
                            return None;
                        }
                        let container = d.container()?;
                        if &&container != c {
                            return None;
                        }

                        Some((
                            MarkedString::String(d.markdown()),
                            d.compare_parameters_with(*params), // <-- sort priority
                        ))
                    })
                    .collect::<Vec<_>>();

                constructors.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                markups.extend(constructors.into_iter().map(|(c, _)| c));
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

pub fn get_completion<'a, I, D>(semantic_info: &SemanticInfo, definitions: I) -> Vec<CompletionItem>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + CodeSymbolInformation + MarkupDefinition + LocationDefinition + 'a,
{
    match semantic_info {
        SemanticInfo::RefClass(class_name)
        | SemanticInfo::SuperCall(_, _, class_name)
        | SemanticInfo::NewExpression(Some(class_name), _) => {
            let members = get_class_members_definition(definitions, class_name);
            let completions: Vec<CompletionItem> =
                get_completion_items_from_definitions(members.into_iter().map(|(_, d)| d));
            completions
        }
        SemanticInfo::NewExpression(None, _) => {
            let classes = definitions
                .into_iter()
                .filter_map(|(_, d)| d.is_class().then_some(d));
            get_completion_items_from_definitions(classes)
        }
        _ => vec![],
    }
}

fn get_class_members_definition<'a, I, D>(definitions: I, class_name: &str) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let definitions = Vec::from_iter(definitions);
    let classes = definitions
        .iter()
        .filter_map(|(_, d)| d.is_class().then_some(d))
        .collect::<Vec<_>>();

    // collect hierarchy
    let mut classes_hier = vec![class_name];

    let mut stack = vec![class_name];
    while let Some(current_class) = stack.pop() {
        for class in classes.iter().filter(|c| c.compare_id_with(current_class)) {
            if let Some(parent) = class.parent() {
                classes_hier.push(parent);
                stack.push(parent);
            }
        }
    }

    classes_hier.dedup(); // remove duplicates

    // collect all class and super class methods
    let mut members: Vec<(Url, &D)> = vec![];
    for current_class in classes_hier {
        let current_members = definitions
            .iter()
            // find by class name
            .filter(|(_uri, member)| {
                member
                    .container()
                    .is_some_and(|c| c.compare_id_with(current_class))
            })
            // filter out already added members
            .filter(|(_uri, current_member)| {
                members
                    .iter()
                    .any(|(_uri, member)| member.can_be_overridden(*current_member))
                    .not()
            })
            .map(|(uri, member)| (uri.clone(), *member))
            .collect::<Vec<_>>();

        members.extend(current_members);
    }

    members
}

fn get_completion_items_from_definitions<'a, I, D>(definitions: I) -> Vec<CompletionItem>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolDefinition + CodeSymbolInformation + MarkupDefinition + 'a,
{
    let mut def_groups: HashMap<&str, Vec<&D>> = HashMap::new();
    for d in definitions
        .into_iter()
        .filter(|d| d.is_method() || d.is_getter() || d.is_setter())
    {
        def_groups.entry(d.id()).or_default().push(d);
    }

    def_groups
        .into_iter()
        .map(|def_group| {
            let first_def = def_group.1.first().unwrap();
            let completion_label = first_def.symbol_name();
            let mut completion_item = CompletionItem::new_simple(
                completion_label.to_string(),
                first_def.parent().unwrap_or_default().to_string(),
            );

            if first_def.is_method() {
                completion_item.kind = Some(CompletionItemKind::METHOD);
            } else if first_def.is_getter() || first_def.is_setter() {
                completion_item.kind = Some(CompletionItemKind::PROPERTY);
            } else if first_def.is_class() {
                completion_item.kind = Some(CompletionItemKind::CLASS);
            }

            if completion_label.starts_with("_") {
                completion_item.sort_text = Some(format!("я{}", completion_label));
            }

            let markdown_strs: Vec<String> = def_group.1.iter().map(|d| d.markdown()).collect();
            let markdown = MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown_strs.join("  \n"),
            };
            completion_item.documentation = Some(Documentation::MarkupContent(markdown));
            completion_item
        })
        .collect()
}
