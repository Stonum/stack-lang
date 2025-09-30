use std::sync::{Arc, Weak};

use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{AstNode, AstNodeList, SyntaxNode, TriviaPieceKind, WalkEvent};
use line_index::{LineColRange, LineIndex};

use mlang_lsp_definition::{
    CodeSymbolDefinition, CodeSymbolInformation, LocationDefinition, MarkupDefinition, SymbolKind,
};
use mlang_syntax::{
    AnyMClassMember, AnyMExpression, AnyMLiteralExpression, AnyMStatement, AnyMSwitchClause,
    MClassDeclaration, MFileSource, MFunctionDeclaration, MLanguage, MReport, MReportSection,
    MSyntaxNode,
};

pub fn semantics(
    text: &str,
    root: SyntaxNode<MLanguage>,
    source_type: MFileSource,
) -> Vec<AnyMDefinition> {
    let mut collector = SemanticModel {
        definitions: vec![],
    };

    let line_index = LineIndex::new(text);

    for event in root.preorder() {
        if let WalkEvent::Enter(node) = event {
            collector.visit_node(source_type, &line_index, node);
        }
    }

    collector.definitions
}

#[derive(Debug, Default)]
pub struct SemanticModel {
    definitions: Vec<AnyMDefinition>,
}

impl SemanticModel {
    pub fn definitions<'a>(&'a self) -> Vec<&'a AnyMDefinition> {
        self.definitions.iter().collect()
    }
}

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
        matches!(self, AnyMDefinition::MClassMemberDefinition(_))
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
            AnyMDefinition::MClassMemberDefinition(_) => SymbolKind::FUNCTION,
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
                "```{} {}{}```  \n{}",
                function.type_keyword(),
                function.id.name,
                function.params,
                function.description.as_deref().unwrap_or_default()
            ),
            AnyMDefinition::MClassDefinition(class) => format!(
                "```{} {}```  \n{}",
                class.type_keyword(),
                class.id.name,
                class.description.as_deref().unwrap_or_default()
            ),
            AnyMDefinition::MClassMemberDefinition(member) => match member.m_type {
                MClassMethodType::Method if member.class.upgrade().is_some() => {
                    let class = member.class.upgrade().unwrap();

                    format!(
                        "```{} {}```  \n```{}{}```  \n{}",
                        class.type_keyword(),
                        class.id.name,
                        member.id.name,
                        member.params,
                        member.description.as_deref().unwrap_or_default()
                    )
                }
                _ => format!(
                    "```{} {}{}```  \n{}",
                    member.type_keyword(),
                    member.id.name,
                    member.params,
                    member.description.as_deref().unwrap_or_default()
                ),
            },
            AnyMDefinition::MReportDefinition(report) => {
                format!("```{} {}```", report.type_keyword(), report.id.name,)
            }
            AnyMDefinition::MReportSectionDefiniton(section) => {
                format!("```{} {}```", section.type_keyword(), section.id.name,)
            }
            AnyMDefinition::MHandlerDefinition(handler) => {
                format!("```{} {}```", handler.type_keyword(), handler.id.name,)
            }
            AnyMDefinition::MHandlerEventDefinition(event) => {
                format!("```{} {}```", event.type_keyword(), event.id.name,)
            }
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MFunctionDefinition {
    id: DefinitionId,
    params: String,
    description: Option<String>,
    range: LineColRange,
}
impl MFunctionDefinition {
    fn type_keyword(&self) -> &'static str {
        "function"
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassDefinition {
    id: DefinitionId,
    methods: Vec<Arc<MClassMemberDefinition>>,
    description: Option<String>,
    range: LineColRange,
    extends: Option<String>,
}
impl MClassDefinition {
    fn type_keyword(&self) -> &'static str {
        "class"
    }
}

#[derive(Debug, Default)]
pub struct MClassMemberDefinition {
    id: DefinitionId,
    class: Weak<MClassDefinition>,
    params: String,
    description: Option<String>,
    range: LineColRange,
    m_type: MClassMethodType,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
enum MClassMethodType {
    Constructor,
    Getter,
    Setter,
    #[default]
    Method,
}

impl MClassMemberDefinition {
    fn type_keyword(&self) -> &'static str {
        match self.m_type {
            MClassMethodType::Getter => "get",
            MClassMethodType::Setter => "set",
            _ => "",
        }
    }
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

impl MReportDefinition {
    fn type_keyword(&self) -> &'static str {
        "report"
    }
}

#[derive(Debug, Default)]
pub struct MReportSectionDefiniton {
    id: DefinitionId,
    report: Weak<MReportDefinition>,
    range: LineColRange,
}

impl MReportSectionDefiniton {
    fn type_keyword(&self) -> &'static str {
        "section"
    }
}

impl PartialEq for MReportSectionDefiniton {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.range == other.range
    }
}
impl Eq for MReportSectionDefiniton {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MHandlerDefinition {
    id: DefinitionId,
    params: String,
    events: Vec<Arc<MHandlerEventDefinition>>,
    range: LineColRange,
}
impl MHandlerDefinition {
    fn type_keyword(&self) -> &'static str {
        "function"
    }
}

#[derive(Debug, Default)]
pub struct MHandlerEventDefinition {
    id: DefinitionId,
    handler: Weak<MHandlerDefinition>,
    range: LineColRange,
}
impl MHandlerEventDefinition {
    fn type_keyword(&self) -> &'static str {
        ""
    }
}

impl PartialEq for MHandlerEventDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.range == other.range
    }
}
impl Eq for MHandlerEventDefinition {}

impl SemanticModel {
    fn visit_node(&mut self, source_type: MFileSource, index: &LineIndex, node: MSyntaxNode) {
        if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
            if source_type.is_handler() {
                if let Some((handler, events)) = handler_definition(func, index) {
                    let mut events = events
                        .into_iter()
                        .map(|m| AnyMDefinition::MHandlerEventDefinition(m))
                        .collect();
                    self.definitions
                        .push(AnyMDefinition::MHandlerDefinition(handler));
                    self.definitions.append(&mut events);
                }
            } else {
                if let Some(def) = function_definition(func, index) {
                    self.definitions
                        .push(AnyMDefinition::MFunctionDefinition(def));
                }
            }
        }

        if let Some(class) = MClassDeclaration::cast(node.clone()) {
            if let Some((class, metohds)) = class_definition(class, index) {
                let mut metohds = metohds
                    .into_iter()
                    .map(|m| AnyMDefinition::MClassMemberDefinition(m))
                    .collect();
                self.definitions
                    .push(AnyMDefinition::MClassDefinition(class));
                self.definitions.append(&mut metohds);
            }
        }

        if let Some(report) = MReport::cast(node.clone()) {
            if let Some((report, sections)) = report_definition(report, index) {
                let mut sections = sections
                    .into_iter()
                    .map(|m| AnyMDefinition::MReportSectionDefiniton(m))
                    .collect();
                self.definitions
                    .push(AnyMDefinition::MReportDefinition(report));
                self.definitions.append(&mut sections);
            }
        }
    }
}

fn function_definition(
    func: MFunctionDeclaration,
    index: &LineIndex,
) -> Option<MFunctionDefinition> {
    let func_id = func.id().ok()?;
    let func_id_range = index.line_col_range(func_id.range())?;
    let func_range = index.line_col_range(func.range())?;

    let func = MFunctionDefinition {
        id: DefinitionId {
            name: func_id.text(),
            range: func_id_range,
        },
        params: func
            .parameters()
            .map(|params| params.to_string().trim().to_string())
            .unwrap_or_default(),
        description: format_description(
            func.syntax().first_leading_trivia(),
            func.doc_string().map(|s| s.text()),
        ),
        range: func_range,
    };
    Some(func)
}

fn handler_definition(
    func: MFunctionDeclaration,
    index: &LineIndex,
) -> Option<(Arc<MHandlerDefinition>, Vec<MHandlerEventDefinition>)> {
    let func_id = func.id().ok()?;
    let func_id_range = index.line_col_range(func_id.range())?;
    let func_range = index.line_col_range(func.range())?;

    let hdlr = Arc::new(MHandlerDefinition {
        id: DefinitionId {
            name: func_id.text(),
            range: func_id_range,
        },
        params: func
            .parameters()
            .map(|params| params.to_string().trim().to_string())
            .unwrap_or_default(),
        range: func_range,
        events: vec![],
    });

    if let Ok(body) = func.body() {
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
            name: event_id.text().to_string(),
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
    let class_id = class.id().ok()?;
    let class_id_range = index.line_col_range(class_id.range())?;
    let class_range = index.line_col_range(class.range())?;

    let class_def = Arc::new(MClassDefinition {
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
            .map_or(None, |ext| Some(ext.super_class().ok()?.text())),
    });

    let methods = class
        .members()
        .iter()
        .filter_map(|member| class_member_definition(member, Arc::downgrade(&class_def), index))
        .collect::<Vec<_>>();

    Some((class_def, methods))
}

fn class_member_definition(
    member: AnyMClassMember,
    class: Weak<MClassDefinition>,
    index: &LineIndex,
) -> Option<MClassMemberDefinition> {
    let member_id = member.name().ok()??;
    let member_id_range = index.line_col_range(member_id.range())?;
    let member_range = index.line_col_range(member.range())?;

    Some(MClassMemberDefinition {
        id: DefinitionId {
            name: member_id.text(),
            range: member_id_range,
        },
        class,
        params: member
            .params()
            .map(|params| params.map(|p| p.to_string()).unwrap_or(String::from("()")))
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
                        pieces.push(piece.text().replace("#", "\\#"));
                        pieces.push(String::from("  \n"));
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
    let doc_string = doc_string.map(|s| s[1..s.len() - 1].replace("\r\n", "  \n"));

    if let Some(doc_string) = doc_string {
        return description
            .map(|s| format!("{s}  \n{doc_string}"))
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
        let parsed = parse(&text, file_source);

        let definitions = semantics(text, parsed.syntax(), file_source);

        assert!(!definitions.is_empty());

        assert_eq!(
            definitions[0],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: DefinitionId {
                    name: String::from("a"),
                    range: line_col_range(5, 9, 5, 10)
                },

                params: String::from("(x, y, z = 5, ...)"),
                description: Some(String::from(
                    "  \n\\# something else  \n\\# about function a"
                )),
                range: line_col_range(5, 4, 7, 5),
            })
        );

        assert_eq!(
            definitions[1],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: DefinitionId {
                    name: String::from("b"),
                    range: line_col_range(10, 9, 10, 10)
                },

                params: String::from("()"),
                description: Some(String::from("  \n\\# about function b")),
                range: line_col_range(10, 4, 12, 5)
            })
        );

        assert_eq!(
            definitions[2],
            AnyMDefinition::MClassDefinition(Arc::new(MClassDefinition {
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
            definitions[3],
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                id: DefinitionId {
                    name: String::from("constructor"),
                    range: line_col_range(15, 8, 15, 19)
                },
                class: Weak::new(),
                params: String::from("()"),
                description: None,
                range: line_col_range(15, 8, 15, 24),
                m_type: MClassMethodType::Constructor
            }),
        );

        assert_eq!(
            definitions[4],
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                id: DefinitionId {
                    name: String::from("x"),
                    range: line_col_range(18, 12, 18, 13)
                },
                class: Weak::new(),
                params: String::from("()"),
                description: Some(String::from("  \n\\# getter description")),
                range: line_col_range(18, 8, 20, 9),
                m_type: MClassMethodType::Getter
            }),
        );

        assert_eq!(
            definitions[5],
            AnyMDefinition::MClassMemberDefinition(MClassMemberDefinition {
                id: DefinitionId {
                    name: String::from("calc"),
                    range: line_col_range(21, 8, 21, 12)
                },
                class: Weak::new(),
                params: String::from("()"),
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
        let parsed = parse(&text, file_source);

        let definitions = semantics(text, parsed.syntax(), file_source);

        assert!(!definitions.is_empty());

        assert_eq!(
            definitions[0],
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
            definitions[1],
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
            definitions[2],
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
}
