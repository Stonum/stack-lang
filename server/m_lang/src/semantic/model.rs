use std::sync::{Arc, Weak};

use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{AstNode, AstNodeList, SyntaxNode, TriviaPieceKind, WalkEvent};
use line_index::{LineColRange, LineIndex};

use crate::syntax::{
    AnyMClassMember, MClassDeclaration, MFunctionDeclaration, MLanguage, MReport, MReportSection,
    MSyntaxNode,
};

pub fn semantics(text: &str, root: SyntaxNode<MLanguage>) -> SemanticModel {
    let mut collector = SemanticModel {
        definitions: vec![],
    };

    let line_index = LineIndex::new(text);

    for event in root.preorder() {
        if let WalkEvent::Enter(node) = event {
            collector.visit_node(&line_index, node);
        }
    }

    collector
}

#[derive(Debug, Default)]
pub struct SemanticModel {
    definitions: Vec<AnyMDefinition>,
}

impl SemanticModel {
    pub fn definitions(&self) -> &Vec<AnyMDefinition> {
        self.definitions.as_ref()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMDefinition {
    MFunctionDefinition(MFunctionDefinition),
    MClassDefinition(Arc<MClassDefinition>),
    MClassMemberDefinition(MClassMemberDefinition),
    MReportDefinition(Arc<MReportDefinition>),
    MReportSectionDefiniton(MReportSectionDefiniton),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct DefinitionId {
    name: String,
    range: LineColRange,
}

impl AnyMDefinition {
    pub fn is_class(&self) -> bool {
        match self {
            AnyMDefinition::MClassDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            AnyMDefinition::MFunctionDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_report(&self) -> bool {
        match self {
            AnyMDefinition::MReportDefinition(_) => true,
            _ => false,
        }
    }

    pub fn is_constructor(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(method) => method.is_constructor(),
            _ => false,
        }
    }

    pub fn is_method(&self) -> bool {
        match self {
            AnyMDefinition::MClassMemberDefinition(method) => method.is_method(),
            _ => false,
        }
    }

    pub fn container(&self) -> Option<AnyMDefinition> {
        match self {
            AnyMDefinition::MClassMemberDefinition(method) => {
                method.class().map(AnyMDefinition::MClassDefinition)
            }
            AnyMDefinition::MReportSectionDefiniton(section) => {
                section.report().map(AnyMDefinition::MReportDefinition)
            }
            _ => None,
        }
    }

    pub fn parent(&self) -> Option<&String> {
        match self {
            AnyMDefinition::MClassDefinition(class) => class.extends(),
            _ => None,
        }
    }
}

pub trait Definition {
    fn range(&self) -> LineColRange;
    fn type_keyword(&self) -> &'static str;
    fn id(&self) -> String;
    fn id_range(&self) -> LineColRange;
    fn params(&self) -> String;
    fn description(&self) -> Option<String>;

    fn to_markdown(&self) -> String {
        format!(
            "```{} {}{}```  \n{}",
            self.type_keyword(),
            self.id(),
            self.params(),
            self.description().unwrap_or_default()
        )
    }
}

impl Definition for AnyMDefinition {
    fn range(&self) -> LineColRange {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.range(),
            AnyMDefinition::MClassDefinition(def) => def.range(),
            AnyMDefinition::MClassMemberDefinition(def) => def.range(),
            AnyMDefinition::MReportDefinition(def) => def.range(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.range(),
        }
    }
    fn type_keyword(&self) -> &'static str {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.type_keyword(),
            AnyMDefinition::MClassDefinition(def) => def.type_keyword(),
            AnyMDefinition::MClassMemberDefinition(def) => def.type_keyword(),
            AnyMDefinition::MReportDefinition(def) => def.type_keyword(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.type_keyword(),
        }
    }
    fn id(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.id(),
            AnyMDefinition::MClassDefinition(def) => def.id(),
            AnyMDefinition::MClassMemberDefinition(def) => def.id(),
            AnyMDefinition::MReportDefinition(def) => def.id(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.id(),
        }
    }
    fn id_range(&self) -> LineColRange {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.id_range(),
            AnyMDefinition::MClassDefinition(def) => def.id_range(),
            AnyMDefinition::MClassMemberDefinition(def) => def.id_range(),
            AnyMDefinition::MReportDefinition(def) => def.id_range(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.id_range(),
        }
    }
    fn params(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.params(),
            AnyMDefinition::MClassDefinition(def) => def.params(),
            AnyMDefinition::MClassMemberDefinition(def) => def.params(),
            AnyMDefinition::MReportDefinition(def) => def.params(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.params(),
        }
    }
    fn description(&self) -> Option<String> {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.description(),
            AnyMDefinition::MClassDefinition(def) => def.description(),
            AnyMDefinition::MClassMemberDefinition(def) => def.description(),
            AnyMDefinition::MReportDefinition(def) => def.description(),
            AnyMDefinition::MReportSectionDefiniton(def) => def.description(),
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
impl Definition for MFunctionDefinition {
    fn range(&self) -> LineColRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "function"
    }

    fn id(&self) -> String {
        self.id.name.clone()
    }

    fn params(&self) -> String {
        self.params.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn id_range(&self) -> LineColRange {
        self.id.range
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
    pub fn methods(&self) -> &Vec<Arc<MClassMemberDefinition>> {
        &self.methods
    }

    pub fn extends(&self) -> Option<&String> {
        self.extends.as_ref()
    }
}

impl Definition for MClassDefinition {
    fn range(&self) -> LineColRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "class"
    }

    fn id(&self) -> String {
        self.id.name.clone()
    }

    fn params(&self) -> String {
        String::from("")
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn id_range(&self) -> LineColRange {
        self.id.range
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
    pub fn is_constructor(&self) -> bool {
        self.m_type == MClassMethodType::Constructor
    }

    pub fn is_method(&self) -> bool {
        self.m_type == MClassMethodType::Method
    }

    pub fn class(&self) -> Option<Arc<MClassDefinition>> {
        self.class.upgrade()
    }
}

impl Definition for MClassMemberDefinition {
    fn range(&self) -> LineColRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        match self.m_type {
            MClassMethodType::Getter => "get",
            MClassMethodType::Setter => "set",
            _ => "",
        }
    }

    fn id(&self) -> String {
        self.id.name.clone()
    }

    fn params(&self) -> String {
        self.params.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn id_range(&self) -> LineColRange {
        self.id.range
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
    pub fn sections(&self) -> &Vec<Arc<MReportSectionDefiniton>> {
        &self.sections
    }
}

impl Definition for MReportDefinition {
    fn range(&self) -> LineColRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "report"
    }

    fn id(&self) -> String {
        self.id.name.clone()
    }

    fn params(&self) -> String {
        String::from("")
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn id_range(&self) -> LineColRange {
        self.id.range
    }
}

#[derive(Debug, Default)]
pub struct MReportSectionDefiniton {
    id: DefinitionId,
    report: Weak<MReportDefinition>,
    range: LineColRange,
}

impl Definition for MReportSectionDefiniton {
    fn range(&self) -> LineColRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "section"
    }

    fn id(&self) -> String {
        self.id.name.clone()
    }

    fn params(&self) -> String {
        String::from("")
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn id_range(&self) -> LineColRange {
        self.id.range
    }
}

impl MReportSectionDefiniton {
    pub fn report(&self) -> Option<Arc<MReportDefinition>> {
        self.report.upgrade()
    }
}

impl PartialEq for MReportSectionDefiniton {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.range == other.range
    }
}
impl Eq for MReportSectionDefiniton {}

impl SemanticModel {
    fn visit_node(&mut self, index: &LineIndex, node: MSyntaxNode) {
        if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
            if let Some(def) = function_definition(func, index) {
                self.definitions
                    .push(AnyMDefinition::MFunctionDefinition(def));
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

    use crate::{parser::parse, syntax::MFileSource};

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
        let parsed = parse(&text, MFileSource::module());

        let collector = semantics(text, parsed.syntax());

        assert!(!collector.definitions.is_empty());

        assert_eq!(
            collector.definitions[0],
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
            collector.definitions[1],
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
            collector.definitions[2],
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
            collector.definitions[3],
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
            collector.definitions[4],
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
            collector.definitions[5],
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
        let parsed = parse(&text, MFileSource::report());

        let collector = semantics(text, parsed.syntax());

        assert!(!collector.definitions.is_empty());

        assert_eq!(
            collector.definitions[0],
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
            collector.definitions[1],
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
            collector.definitions[2],
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
