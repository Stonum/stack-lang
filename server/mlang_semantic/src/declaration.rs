use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{AstNode, AstNodeList, TriviaPieceKind};
use std::collections::HashSet;
use std::sync::{Arc, Weak};

use line_index::{LineColRange, LineIndex};

use mlang_lsp_definition::{
    CodeSymbolDefinition, CodeSymbolInformation, LocationDefinition, MarkupDefinition, SymbolKind,
};
use mlang_syntax::{
    AnyMClassMember, AnyMFunction, AnyMLiteralExpression, AnyMParameterList, AnyMSwitchClause,
    MClassDeclaration, MFileSource, MFunctionDeclaration, MLanguage, MReport, MReportSection,
    MSyntaxNode,
};

use crate::SemanticModel;

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMDefinition {
    MFunctionDefinition(MFunctionDefinition),
    MClassDefinition(Arc<MClassDefinition>),
    MClassMemberDefinition(MClassMemberDefinition),
    MReportDefinition(Arc<MReportDefinition>),
    MReportSectionDefiniton(MReportSectionDefiniton),
    MHandlerDefinition(Arc<MHandlerDefinition>),
    MHandlerEventDefinition(MHandlerEventDefinition),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct DefinitionId {
    name: String,
    range: LineColRange,
}

impl CodeSymbolDefinition for AnyMDefinition {
    fn is_function(&self) -> bool {
        matches!(self, AnyMDefinition::MFunctionDefinition(_))
    }

    fn is_class(&self) -> bool {
        matches!(self, AnyMDefinition::MClassDefinition(_))
    }

    fn is_constructor(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.m_type == MClassMethodType::Constructor
            }
            _ => false,
        }
    }

    fn is_method(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.m_type == MClassMethodType::Method
            }
            _ => false,
        }
    }

    fn is_getter(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.m_type == MClassMethodType::Getter
            }
            _ => false,
        }
    }

    fn is_setter(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.m_type == MClassMethodType::Setter
            }
            _ => false,
        }
    }

    fn is_property(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.m_type == MClassMethodType::Property
            }
            _ => false,
        }
    }

    fn id(&self) -> &str {
        match self {
            AnyMDefinition::MFunctionDefinition(f) => &f.id.name,
            AnyMDefinition::MClassDefinition(c) => &c.id.name,
            AnyMDefinition::MClassMemberDefinition(m) => &m.id.name,
            AnyMDefinition::MReportDefinition(r) => &r.id.name,
            AnyMDefinition::MReportSectionDefiniton(s) => &s.id.name,
            AnyMDefinition::MHandlerDefinition(h) => &h.id.name,
            AnyMDefinition::MHandlerEventDefinition(e) => &e.id.name,
        }
    }

    fn container(&self) -> Option<AnyMDefinition> {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => {
                member.class.upgrade().map(AnyMDefinition::MClassDefinition)
            }
            AnyMDefinition::MReportSectionDefiniton(section) => section
                .report
                .upgrade()
                .map(AnyMDefinition::MReportDefinition),
            AnyMDefinition::MHandlerEventDefinition(event) => event
                .handler
                .upgrade()
                .map(AnyMDefinition::MHandlerDefinition),
            _ => None,
        }
    }

    fn parent(&self) -> Option<&str> {
        match self {
            AnyMDefinition::MClassDefinition(class) => class.extends.as_deref(),
            _ => None,
        }
    }

    fn parameters(&self) -> Option<&str> {
        match self {
            AnyMDefinition::MClassMemberDefinition(member) => Some(&member.params.text),
            AnyMDefinition::MFunctionDefinition(funct) => Some(&funct.params.text),
            AnyMDefinition::MHandlerDefinition(handler) => Some(&handler.params.text),
            _ => None,
        }
    }

    fn can_be_overridden(&self, another: &Self) -> bool {
        use AnyMDefinition::*;

        let id_is_equal = self.compare_id_with(another.id());

        match (self, another) {
            (MFunctionDefinition(a), MFunctionDefinition(b)) => {
                id_is_equal && a.params.can_be_overridden(&b.params)
            }

            (MClassDefinition(_), MClassDefinition(_)) => id_is_equal,
            (MClassMemberDefinition(a), MClassMemberDefinition(b)) => match (a.m_type, b.m_type) {
                (MClassMethodType::Getter, MClassMethodType::Getter) => id_is_equal,
                (MClassMethodType::Setter, MClassMethodType::Setter) => id_is_equal,
                (MClassMethodType::Constructor, MClassMethodType::Constructor) => {
                    a.params.can_be_overridden(&b.params)
                }
                (MClassMethodType::Method, MClassMethodType::Method) => {
                    id_is_equal && a.params.can_be_overridden(&b.params)
                }
                _ => false,
            },

            (MHandlerDefinition(a), MHandlerDefinition(b)) => {
                id_is_equal && a.params.can_be_overridden(&b.params)
            }
            (MHandlerEventDefinition(a), MHandlerEventDefinition(b)) => {
                let left: Option<AnyMDefinition> =
                    a.handler.upgrade().map(AnyMDefinition::MHandlerDefinition);
                let right: Option<AnyMDefinition> =
                    b.handler.upgrade().map(AnyMDefinition::MHandlerDefinition);
                match (left, right) {
                    (Some(left), Some(right)) => left.can_be_overridden(&right) && id_is_equal,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn can_be_called(&self, count: usize) -> bool {
        match self {
            AnyMDefinition::MFunctionDefinition(f) => f.params.can_be_called(count),
            AnyMDefinition::MClassMemberDefinition(m)
                if m.m_type == MClassMethodType::Method
                    || m.m_type == MClassMethodType::Constructor =>
            {
                m.params.can_be_called(count)
            }
            _ => true,
        }
    }

    fn call_priority(&self, another: &Self, count: usize) -> core::cmp::Ordering {
        use AnyMDefinition::*;

        match (self, another) {
            (MFunctionDefinition(a), MFunctionDefinition(b)) => {
                a.params.call_priority(&b.params, count)
            }
            (MClassMemberDefinition(a), MClassMemberDefinition(b)) => match (a.m_type, b.m_type) {
                (MClassMethodType::Constructor, MClassMethodType::Constructor) => {
                    a.params.call_priority(&b.params, count)
                }
                (MClassMethodType::Method, MClassMethodType::Method) => {
                    a.params.call_priority(&b.params, count)
                }
                _ => core::cmp::Ordering::Less,
            },
            (MHandlerDefinition(a), MHandlerDefinition(b)) => {
                a.params.call_priority(&b.params, count)
            }
            _ => core::cmp::Ordering::Less,
        }
    }
}

impl LocationDefinition for AnyMDefinition {
    fn range(&self) -> LineColRange {
        match self {
            AnyMDefinition::MFunctionDefinition(function) => function.range,
            AnyMDefinition::MClassDefinition(class) => class.range,
            AnyMDefinition::MClassMemberDefinition(member) => member.range,
            AnyMDefinition::MReportDefinition(report) => report.range,
            AnyMDefinition::MReportSectionDefiniton(section) => section.range,
            AnyMDefinition::MHandlerDefinition(handler) => handler.range,
            AnyMDefinition::MHandlerEventDefinition(event) => event.range,
        }
    }
    fn id_range(&self) -> LineColRange {
        match self {
            AnyMDefinition::MFunctionDefinition(function) => function.id.range,
            AnyMDefinition::MClassDefinition(class) => class.id.range,
            AnyMDefinition::MClassMemberDefinition(member) => member.id.range,
            AnyMDefinition::MReportDefinition(report) => report.id.range,
            AnyMDefinition::MReportSectionDefiniton(section) => section.id.range,
            AnyMDefinition::MHandlerDefinition(handler) => handler.id.range,
            AnyMDefinition::MHandlerEventDefinition(event) => event.id.range,
        }
    }
}

impl CodeSymbolInformation for AnyMDefinition {
    fn symbol_kind(&self) -> SymbolKind {
        match self {
            AnyMDefinition::MFunctionDefinition(_) => SymbolKind::FUNCTION,
            AnyMDefinition::MClassDefinition(_) => SymbolKind::CLASS,
            AnyMDefinition::MClassMemberDefinition(member) => match member.m_type {
                MClassMethodType::Constructor => SymbolKind::CONSTRUCTOR,
                MClassMethodType::Getter => SymbolKind::PROPERTY,
                MClassMethodType::Setter => SymbolKind::PROPERTY,
                MClassMethodType::Method => SymbolKind::METHOD,
                MClassMethodType::Property => SymbolKind::VARIABLE,
            },
            AnyMDefinition::MReportDefinition(_) => SymbolKind::CONSTANT,
            AnyMDefinition::MReportSectionDefiniton(_) => SymbolKind::FIELD,
            AnyMDefinition::MHandlerDefinition(_) => SymbolKind::FUNCTION,
            AnyMDefinition::MHandlerEventDefinition(_) => SymbolKind::EVENT,
        }
    }
}

impl MarkupDefinition for AnyMDefinition {
    fn markdown(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(function) => format!(
                "```\n{} {}{}\n```  \n{}",
                function.keyword,
                function.id.name,
                function.params.text,
                function
                    .description
                    .as_deref()
                    .map(|s| self.escape_markdown_with_newlines(s))
                    .unwrap_or_default()
            ),
            AnyMDefinition::MClassDefinition(class) => format!(
                "```\n{} {}\n```  \n{}",
                class.keyword,
                class.id.name,
                class
                    .description
                    .as_deref()
                    .map(|s| self.escape_markdown_with_newlines(s))
                    .unwrap_or_default()
            ),
            AnyMDefinition::MClassMemberDefinition(member) => match member.m_type {
                MClassMethodType::Method if member.class.upgrade().is_some() => {
                    let class = member.class.upgrade().unwrap();

                    format!(
                        "```\n{} {}\n\n\t{}{}\n```  \n{}",
                        class.keyword,
                        class.id.name,
                        member.id.name,
                        member.params.text,
                        member
                            .description
                            .as_deref()
                            .map(|s| self.escape_markdown_with_newlines(s))
                            .unwrap_or_default()
                    )
                }

                MClassMethodType::Getter | MClassMethodType::Setter
                    if member.class.upgrade().is_some() =>
                {
                    let class = member.class.upgrade().unwrap();

                    format!(
                        "```\n{} {}\n\n\t{} {}{}\n```  \n{}",
                        class.keyword,
                        class.id.name,
                        member.keyword.as_deref().unwrap_or_default(),
                        member.id.name,
                        member.params.text,
                        member
                            .description
                            .as_deref()
                            .map(|s| self.escape_markdown_with_newlines(s))
                            .unwrap_or_default()
                    )
                }

                _ => format!(
                    "```\n{}{}\n```  \n{}",
                    member.id.name,
                    member.params.text,
                    member
                        .description
                        .as_deref()
                        .map(|s| self.escape_markdown_with_newlines(s))
                        .unwrap_or_default()
                ),
            },
            AnyMDefinition::MReportDefinition(report) => report.id.name.to_string(),
            AnyMDefinition::MReportSectionDefiniton(section) => section.id.name.to_string(),
            AnyMDefinition::MHandlerDefinition(handler) => {
                format!("```\n{} {}\n```", handler.keyword, handler.id.name)
            }
            AnyMDefinition::MHandlerEventDefinition(event) => event.id.name.to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MParameters {
    text: String,
    total_count: usize,
    optional_count: usize,
    has_rest: bool,
}

impl MParameters {
    pub fn can_be_overridden(&self, another: &Self) -> bool {
        self.total_count == another.total_count && self.has_rest == another.has_rest
    }

    pub fn can_be_called(&self, count: usize) -> bool {
        let strict_count = self.total_count - self.optional_count - self.has_rest as usize;

        // (a, b, c) && count < 3
        if count < strict_count {
            return false;
        }

        // (a, ...) && count >= 1
        if self.has_rest {
            return true;
        }

        // (a, b, c, d = 1) && count == 3 || count == 4
        count - strict_count <= self.optional_count
    }

    pub fn call_priority(&self, another: &Self, count: usize) -> core::cmp::Ordering {
        // first check: if we can call both functions
        if !self.can_be_called(count) || !another.can_be_called(count) {
            return self
                .can_be_called(count)
                .cmp(&another.can_be_called(count))
                .reverse();
        }

        let possible_count_self = match self.has_rest {
            true => usize::MAX,
            false => self.total_count,
        };
        let possible_count_another = match another.has_rest {
            true => usize::MAX,
            false => another.total_count,
        };

        // let's compare the difference between the possible number of parameters and the incoming one
        // the smaller the difference, the higher the priority
        (possible_count_self - count).cmp(&(possible_count_another - count))
    }
}

impl From<AnyMParameterList> for MParameters {
    fn from(value: AnyMParameterList) -> Self {
        let (has_rest, optional_count) = value.iter().fold((false, 0), |(has_rest, count), par| {
            let is_rest = par.as_ref().is_ok_and(|p| p.is_rest());
            let is_optional = par.as_ref().is_ok_and(|p| p.is_optional());

            (has_rest || is_rest, count + if is_optional { 1 } else { 0 })
        });

        MParameters {
            text: value.to_string().trim().to_string(),
            total_count: value.len(),
            optional_count,
            has_rest,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MFunctionDefinition {
    keyword: String,
    id: DefinitionId,
    params: MParameters,
    description: Option<String>,
    range: LineColRange,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassDefinition {
    keyword: String,
    id: DefinitionId,
    methods: Vec<Arc<MClassMemberDefinition>>,
    description: Option<String>,
    range: LineColRange,
    extends: Option<String>,
}

#[derive(Debug, Default)]
pub struct MClassMemberDefinition {
    keyword: Option<String>,
    id: DefinitionId,
    class: Weak<MClassDefinition>,
    params: MParameters,
    description: Option<String>,
    range: LineColRange,
    m_type: MClassMethodType,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
enum MClassMethodType {
    Constructor,
    Getter,
    Setter,
    Property,
    #[default]
    Method,
}

impl PartialEq for MClassMemberDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.params == other.params
            && self.description == other.description
            && self.range == other.range
            && self.m_type == other.m_type
    }
}
impl Eq for MClassMemberDefinition {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MReportDefinition {
    id: DefinitionId,
    sections: Vec<Arc<MReportSectionDefiniton>>,
    range: LineColRange,
}

#[derive(Debug, Default)]
pub struct MReportSectionDefiniton {
    id: DefinitionId,
    report: Weak<MReportDefinition>,
    range: LineColRange,
}

impl PartialEq for MReportSectionDefiniton {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.range == other.range
    }
}
impl Eq for MReportSectionDefiniton {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MHandlerDefinition {
    keyword: String,
    id: DefinitionId,
    params: MParameters,
    events: Vec<Arc<MHandlerEventDefinition>>,
    range: LineColRange,
}

#[derive(Debug, Default)]
pub struct MHandlerEventDefinition {
    id: DefinitionId,
    handler: Weak<MHandlerDefinition>,
    range: LineColRange,
}

impl PartialEq for MHandlerEventDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.range == other.range
    }
}
impl Eq for MHandlerEventDefinition {}

pub(crate) fn prepare_definitions(
    model: &mut SemanticModel,
    source_type: MFileSource,
    index: &LineIndex,
    node: &MSyntaxNode,
) {
    if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
        if source_type.is_handler() {
            if let Some((handler, events)) = handler_definition(func, index) {
                let mut events = events
                    .into_iter()
                    .map(AnyMDefinition::MHandlerEventDefinition)
                    .collect();
                model
                    .definitions
                    .push(AnyMDefinition::MHandlerDefinition(handler));
                model.definitions.append(&mut events);
            }
        } else if let Some(def) = function_definition(func, index) {
            model
                .definitions
                .push(AnyMDefinition::MFunctionDefinition(def));
        }
    }

    if let Some(class) = MClassDeclaration::cast(node.clone())
        && let Some((class, members)) = class_definition(class, index)
    {
        let mut members: Vec<AnyMDefinition> = members
            .into_iter()
            .map(AnyMDefinition::MClassMemberDefinition)
            .collect();
        model
            .definitions
            .push(AnyMDefinition::MClassDefinition(class));
        model.definitions.append(&mut members);
    }

    if let Some(report) = MReport::cast(node.clone())
        && let Some((report, sections)) = report_definition(report, index)
    {
        let mut sections = sections
            .into_iter()
            .map(AnyMDefinition::MReportSectionDefiniton)
            .collect();
        model
            .definitions
            .push(AnyMDefinition::MReportDefinition(report));
        model.definitions.append(&mut sections);
    }
}

fn function_definition(
    func: MFunctionDeclaration,
    index: &LineIndex,
) -> Option<MFunctionDefinition> {
    let func_token = func.function_token().ok()?;
    let func_id = func.id().ok()?;
    let func_id_range = index.line_col_range(func_id.range())?;
    let func_range = index.line_col_range(func.range())?;

    let description = format_description(
        func.syntax().first_leading_trivia(),
        func.doc_string().map(|s| s.text()),
    );

    let params = AnyMFunction::MFunctionDeclaration(func)
        .params()
        .map(Into::into)
        .unwrap_or_default();

    let func = MFunctionDefinition {
        keyword: func_token.text_trimmed().to_string(),
        id: DefinitionId {
            name: func_id.text(),
            range: func_id_range,
        },
        params,
        description,
        range: func_range,
    };
    Some(func)
}

fn handler_definition(
    func: MFunctionDeclaration,
    index: &LineIndex,
) -> Option<(Arc<MHandlerDefinition>, Vec<MHandlerEventDefinition>)> {
    let func_token = func.function_token().ok()?;
    let func_id = func.id().ok()?;
    let func_id_range = index.line_col_range(func_id.range())?;
    let func_range = index.line_col_range(func.range())?;

    let func = AnyMFunction::MFunctionDeclaration(func);

    let params = func.params().map(Into::into).unwrap_or_default();

    let hdlr = Arc::new(MHandlerDefinition {
        keyword: func_token.text_trimmed().to_string(),
        id: DefinitionId {
            name: func_id.text(),
            range: func_id_range,
        },
        params,
        range: func_range,
        events: vec![],
    });

    if let AnyMFunction::MFunctionDeclaration(func) = func
        && let Ok(body) = func.body()
    {
        let events = body
            .statements()
            .iter()
            .filter_map(|statement| {
                let switch_statement = statement.as_m_switch_statement()?;
                let events = switch_statement
                    .cases()
                    .iter()
                    .filter_map(|case| handler_event_definition(case, Arc::downgrade(&hdlr), index))
                    .collect::<Vec<_>>();
                Some(events)
            })
            .flatten()
            .collect::<Vec<_>>();

        return Some((hdlr, events));
    }

    Some((hdlr, vec![]))
}

fn handler_event_definition(
    clause: AnyMSwitchClause,
    handler: Weak<MHandlerDefinition>,
    index: &LineIndex,
) -> Option<MHandlerEventDefinition> {
    let event_range = index.line_col_range(clause.range())?;

    let event_id = match clause {
        AnyMSwitchClause::MCaseClause(m_case_clause) => {
            let test = m_case_clause.test().ok()?;
            let literal = test.as_any_m_literal_expression()?;
            match literal {
                AnyMLiteralExpression::MStringLiteralExpression(string) => string.value_token(),
                AnyMLiteralExpression::MLongStringLiteralExpression(string) => string.value_token(),
                _ => return None,
            }
        }
        AnyMSwitchClause::MDefaultClause(m_default_clause) => m_default_clause.else_token(),
    };

    let event_id = event_id.ok()?;
    let event_id_range = index.line_col_range(event_id.text_range())?;

    Some(MHandlerEventDefinition {
        id: DefinitionId {
            name: event_id.text_trimmed().to_string(),
            range: event_id_range,
        },
        handler,
        range: event_range,
    })
}

fn class_definition(
    class: MClassDeclaration,
    index: &LineIndex,
) -> Option<(Arc<MClassDefinition>, Vec<MClassMemberDefinition>)> {
    let class_token = class.class_token().ok()?;
    let class_id = class.id().ok()?;
    let class_id_range = index.line_col_range(class_id.range())?;
    let class_range = index.line_col_range(class.range())?;

    let class_def = Arc::new(MClassDefinition {
        keyword: class_token.text_trimmed().to_string(),
        id: DefinitionId {
            name: class_id.text(),
            range: class_id_range,
        },
        methods: vec![],
        description: format_description(
            class.syntax().first_leading_trivia(),
            class.doc_string().map(|s| s.text()),
        ),
        range: class_range,
        extends: class
            .extends_clause()
            .and_then(|ext| Some(ext.super_class().ok()?.text())),
    });

    let mut members = class
        .members()
        .iter()
        .filter_map(|member| class_member_definition(member, Arc::downgrade(&class_def), index))
        .collect::<Vec<_>>();

    add_variables_to_class_members(&mut members, class, index, &class_def);

    Some((class_def, members))
}

// select all class members that are not explicitly declared
// like
// class Test {
//    constructor() { this.a = 1; }
// }
// where 'a' is class variable
fn add_variables_to_class_members(
    members: &mut Vec<MClassMemberDefinition>,
    class: MClassDeclaration,
    index: &LineIndex,
    class_def: &Arc<MClassDefinition>,
) {
    let constructor = class
        .members()
        .iter()
        .find(|m| m.as_m_constructor_class_member().is_some());

    if let Some(constructor) = constructor
        && let Some(body) = constructor
            .as_m_constructor_class_member()
            .unwrap()
            .body()
            .ok()
        && let Some(constructor_def) =
            class_member_definition(constructor, Arc::downgrade(&class_def), index)
    {
        let static_member_names = body
            .statements()
            .iter()
            .filter_map(|s| {
                let expr_stmt = s.as_m_expression_statement()?;
                let expr = expr_stmt.expression().ok()?;
                let assign = expr.as_m_assignment_expression()?;
                let left = assign.left().ok()?;
                let static_member = left.as_m_static_member_assignment()?;
                static_member.member().ok().iter().next().cloned()
            })
            .map(|m| m.to_string().trim().to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<String>>();

        let mut variables = static_member_names
            .iter()
            .filter(|variable_name| {
                !members.iter().any(|m| {
                    m.id.name
                        .to_lowercase()
                        .eq(variable_name.to_lowercase().as_str())
                })
            })
            .map(|variable_name| class_property_definition(variable_name, &constructor_def))
            .collect();

        members.append(&mut variables);
    }
}

fn class_property_definition(
    variable_name: &String,
    constructor_def: &MClassMemberDefinition,
) -> MClassMemberDefinition {
    MClassMemberDefinition {
        keyword: None,
        id: DefinitionId {
            name: variable_name.to_string(),
            range: constructor_def.range,
        },
        class: constructor_def.class.clone(),
        params: MParameters {
            text: String::from(""),
            total_count: 0,
            optional_count: 0,
            has_rest: false,
        },
        description: None,
        range: constructor_def.range,
        m_type: MClassMethodType::Property,
    }
}

fn class_member_definition(
    member: AnyMClassMember,
    class: Weak<MClassDefinition>,
    index: &LineIndex,
) -> Option<MClassMemberDefinition> {
    let member_token = member.token().ok()?;
    let member_id = member.name().ok()??;
    let member_id_range = index.line_col_range(member_id.range())?;
    let member_range = index.line_col_range(member.range())?;

    Some(MClassMemberDefinition {
        keyword: member_token.map(|s| s.text_trimmed().to_string()),
        id: DefinitionId {
            name: member_id.text(),
            range: member_id_range,
        },
        class,
        params: member
            .params()
            .map(|params| params.map(Into::into))
            .unwrap_or_default()
            .unwrap_or_default(),
        description: format_description(
            member.leading_trivia(),
            member.doc_string().map(|s| s.text()),
        ),
        range: member_range,
        m_type: match member {
            AnyMClassMember::MConstructorClassMember(_) => MClassMethodType::Constructor,
            AnyMClassMember::MGetterClassMember(_) => MClassMethodType::Getter,
            AnyMClassMember::MSetterClassMember(_) => MClassMethodType::Setter,
            _ => MClassMethodType::Method,
        },
    })
}

fn report_definition(
    report: MReport,
    index: &LineIndex,
) -> Option<(Arc<MReportDefinition>, Vec<MReportSectionDefiniton>)> {
    let report_id = report.name().ok()?.m_name().ok()?;
    let report_id_range = index.line_col_range(report_id.range())?;
    let report_range = index.line_col_range(report.range())?;

    let report_def = Arc::new(MReportDefinition {
        id: DefinitionId {
            name: report_id.text(),
            range: report_id_range,
        },
        sections: vec![],
        range: report_range,
    });

    let sections = report
        .sections()
        .iter()
        .filter_map(|section| {
            report_section_definition(section, Arc::downgrade(&report_def), index)
        })
        .collect();

    Some((report_def, sections))
}

fn report_section_definition(
    section: MReportSection,
    report: Weak<MReportDefinition>,
    index: &LineIndex,
) -> Option<MReportSectionDefiniton> {
    let section_id = section.name().ok()?.m_name().ok()?;
    let section_id_range = index.line_col_range(section_id.range())?;
    let section_range = index.line_col_range(section.range())?;

    Some(MReportSectionDefiniton {
        id: DefinitionId {
            name: section_id.text(),
            range: section_id_range,
        },
        report,
        range: section_range,
    })
}

fn format_description(
    trivia: Option<SyntaxTrivia<MLanguage>>,
    doc_string: Option<String>,
) -> Option<String> {
    // All trivia before the first non-whitespace trivia
    let description = trivia
        .filter(|trivia| trivia.pieces().any(|piece| piece.kind().is_comment()))
        .map(|trivia| {
            let mut pieces = Vec::new();
            let mut newline_count = 0;

            for piece in trivia.pieces().rev() {
                match piece.kind() {
                    TriviaPieceKind::SingleLineComment => {
                        pieces.push(piece.text().to_string());
                        pieces.push(String::from("\n"));
                        newline_count = 0;
                    }
                    TriviaPieceKind::Newline => {
                        newline_count += 1;
                        if newline_count >= 2 {
                            break;
                        }
                    }
                    _ => continue,
                }
            }

            pieces.into_iter().rev().collect()
        });

    // doc string after function name
    let doc_string = doc_string.map(|s| s[1..s.len() - 1].to_string());

    if let Some(doc_string) = doc_string {
        return description
            .map(|s| format!("{s}\n{doc_string}"))
            .or(Some(doc_string));
    }

    description
}

#[cfg(test)]
mod tests {
    use line_index::LineCol;

    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    use super::*;
    use crate::semantics;

    #[inline]
    fn line_col_range(
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> LineColRange {
        LineColRange {
            start: LineCol {
                line: start_line,
                col: start_col,
            },
            end: LineCol {
                line: end_line,
                col: end_col,
            },
        }
    }

    #[test]
    fn test_convert_to_definitions() {
        let text = r#"
    # about module a

    # something else
    # about function a
    func a(x, y, z = 5, ...) {
        return b;
    }

    # about function b
    func b() {
        return 123;
    }

    class x extends z {
        constructor() {}

        # getter description
        get x() {
            return 1
        };
        calc() {
            return this.x * 2;
        }
    },
    "#;
        let file_source = MFileSource::module();
        let parsed = parse(text, file_source);

        let semantic_model = semantics(text, parsed.syntax(), file_source);
        let mut definitions = semantic_model.definitions();

        assert_ne!(definitions.len(), 0);

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                keyword: String::from("func"),
                id: DefinitionId {
                    name: String::from("a"),
                    range: line_col_range(5, 9, 5, 10)
                },
                params: MParameters {
                    text: String::from("( x, y, z = 5, ... )"),
                    total_count: 4,
                    optional_count: 1,
                    has_rest: true
                },
                description: Some(String::from("\n# something else\n# about function a")),
                range: line_col_range(5, 4, 7, 5),
            })
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                keyword: String::from("func"),
                id: DefinitionId {
                    name: String::from("b"),
                    range: line_col_range(10, 9, 10, 10)
                },

                params: MParameters {
                    text: String::from("()"),
                    total_count: 0,
                    optional_count: 0,
                    has_rest: false
                },
                description: Some(String::from("\n# about function b")),
                range: line_col_range(10, 4, 12, 5)
            })
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MClassDefinition(Arc::new(MClassDefinition {
                keyword: String::from("class"),
                id: DefinitionId {
                    name: String::from("x"),
                    range: line_col_range(14, 10, 14, 11)
                },

                description: None,
                range: line_col_range(14, 4, 24, 5),
                extends: Some("z".into()),
                methods: vec![]
            }))
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                keyword: None,
                id: DefinitionId {
                    name: String::from("constructor"),
                    range: line_col_range(15, 8, 15, 19)
                },
                class: Weak::new(),
                params: MParameters {
                    text: String::from("()"),
                    total_count: 0,
                    optional_count: 0,
                    has_rest: false
                },
                description: None,
                range: line_col_range(15, 8, 15, 24),
                m_type: MClassMethodType::Constructor
            }),
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                keyword: None,
                id: DefinitionId {
                    name: String::from("x"),
                    range: line_col_range(18, 12, 18, 13)
                },
                class: Weak::new(),
                params: MParameters {
                    text: String::from(""),
                    total_count: 0,
                    optional_count: 0,
                    has_rest: false
                },
                description: Some(String::from("\n# getter description")),
                range: line_col_range(18, 8, 20, 9),
                m_type: MClassMethodType::Getter
            }),
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                keyword: None,
                id: DefinitionId {
                    name: String::from("calc"),
                    range: line_col_range(21, 8, 21, 12)
                },
                class: Weak::new(),
                params: MParameters {
                    text: String::from("()"),
                    total_count: 0,
                    optional_count: 0,
                    has_rest: false
                },
                description: None,
                range: line_col_range(21, 8, 23, 9),
                m_type: MClassMethodType::Method
            })
        );
    }

    #[test]
    fn test_convert_report_to_definitions() {
        let text = r#"#
CommonReport
.CloseWindow = 1;
.Template = "tmp.xlsx";
.ReportFile = "rep.xlsx";
{
    var month = WorkMonth();
}
Function declaration
{
    func add( i )
    {
        return i++;
    }
}
print
{
    print("hey");
}
"#;
        let file_source = MFileSource::report();
        let parsed = parse(text, file_source);

        let semantic_model = semantics(text, parsed.syntax(), file_source);
        let mut definitions = semantic_model.definitions();

        assert_ne!(!definitions.len(), 0);

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MReportDefinition(Arc::new(MReportDefinition {
                id: DefinitionId {
                    name: String::from("CommonReport"),
                    range: line_col_range(1, 2, 1, 14)
                },
                range: line_col_range(1, 0, 18, 1),
                sections: vec![]
            }))
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MReportSectionDefiniton(MReportSectionDefiniton {
                id: DefinitionId {
                    name: String::from("Function declaration"),
                    range: line_col_range(8, 1, 8, 21)
                },
                report: Weak::new(),
                range: line_col_range(8, 0, 14, 1),
            }),
        );

        assert_eq!(
            *definitions.next().unwrap(),
            AnyMDefinition::MReportSectionDefiniton(MReportSectionDefiniton {
                id: DefinitionId {
                    name: String::from("print"),
                    range: line_col_range(15, 1, 15, 6)
                },
                report: Weak::new(),
                range: line_col_range(15, 0, 18, 1),
            }),
        );
    }

    #[test]
    fn test_called_parameters() {
        let params = MParameters {
            text: "(a, b = 1, ...)".into(),
            total_count: 3,
            optional_count: 1,
            has_rest: true,
        };
        assert!(params.can_be_called(1));
        assert!(params.can_be_called(2));
        assert!(params.can_be_called(3));
        assert!(params.can_be_called(99));

        let params = MParameters {
            text: "(a, b, c".into(),
            total_count: 3,
            optional_count: 0,
            has_rest: false,
        };
        assert!(params.can_be_called(3));
        assert!(!params.can_be_called(1));
        assert!(!params.can_be_called(99));
    }

    #[test]
    fn test_call_priority() {
        let mut functions = [
            MParameters {
                text: "(a, ...)".into(),
                total_count: 2,
                optional_count: 0,
                has_rest: true,
            },
            MParameters {
                text: "(a, b = 1)".into(),
                total_count: 2,
                optional_count: 1,
                has_rest: false,
            },
            MParameters {
                text: "(a, b = 1, c = 2)".into(),
                total_count: 3,
                optional_count: 2,
                has_rest: false,
            },
            MParameters {
                text: "(a, b = 1, c = 2, d = 3)".into(),
                total_count: 4,
                optional_count: 3,
                has_rest: false,
            },
        ];

        // 1 args
        functions.sort_by(|a, b| a.call_priority(b, 1));
        assert_eq!(functions[0].text, "(a, b = 1)");

        // 2 args
        functions.sort_by(|a, b| a.call_priority(b, 2));
        assert_eq!(functions[0].text, "(a, b = 1)");

        // 3 args
        functions.sort_by(|a, b| a.call_priority(b, 3));
        assert_eq!(functions[0].text, "(a, b = 1, c = 2)");

        // 4 args
        functions.sort_by(|a, b| a.call_priority(b, 4));
        assert_eq!(functions[0].text, "(a, b = 1, c = 2, d = 3)");

        // 5 args
        functions.sort_by(|a, b| a.call_priority(b, 5));
        assert_eq!(functions[0].text, "(a, ...)");
    }
}
