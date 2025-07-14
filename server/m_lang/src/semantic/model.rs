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
    pub definitions: Vec<AnyMDefinition>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMDefinition {
    MFunctionDefinition(MFunctionDefinition),
    MClassDefinition(MClassDefinition),
    MReportDefinition(MReportDefinition),
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

    pub fn methods(&self) -> Option<&Vec<MClassMethodDefinition>> {
        match self {
            AnyMDefinition::MFunctionDefinition(_) => None,
            AnyMDefinition::MClassDefinition(class) => Some(class.methods()),
            AnyMDefinition::MReportDefinition(_) => None,
        }
    }

    pub fn extends(&self) -> Option<&String> {
        match self {
            AnyMDefinition::MFunctionDefinition(_) => None,
            AnyMDefinition::MClassDefinition(class) => class.extends(),
            AnyMDefinition::MReportDefinition(_) => None,
        }
    }
}

pub trait Definition {
    fn range(&self) -> LineColRange;
    fn type_keyword(&self) -> &'static str;
    fn id(&self) -> String;
    fn params(&self) -> String;
    fn description(&self) -> Option<String>;
    fn doc_string(&self) -> Option<String>;

    fn to_markdown(&self) -> String {
        format!(
            "```{} {}{}```\n{}\n{}",
            self.type_keyword(),
            self.id(),
            self.params(),
            self.description()
                .unwrap_or_default()
                .replace("#", "\\#")
                .replace("\r\n", "  \n"),
            self.doc_string()
                .map(|s| s[1..s.len() - 1].replace("\r\n", "  \n"))
                .unwrap_or_default()
        )
    }
}

impl Definition for AnyMDefinition {
    fn range(&self) -> LineColRange {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.range(),
            AnyMDefinition::MClassDefinition(def) => def.range(),
            AnyMDefinition::MReportDefinition(def) => def.range(),
        }
    }
    fn type_keyword(&self) -> &'static str {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.type_keyword(),
            AnyMDefinition::MClassDefinition(def) => def.type_keyword(),
            AnyMDefinition::MReportDefinition(def) => def.type_keyword(),
        }
    }
    fn id(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.id(),
            AnyMDefinition::MClassDefinition(def) => def.id(),
            AnyMDefinition::MReportDefinition(def) => def.id(),
        }
    }
    fn params(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.params(),
            AnyMDefinition::MClassDefinition(def) => def.params(),
            AnyMDefinition::MReportDefinition(def) => def.params(),
        }
    }
    fn description(&self) -> Option<String> {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.description(),
            AnyMDefinition::MClassDefinition(def) => def.description(),
            AnyMDefinition::MReportDefinition(def) => def.description(),
        }
    }
    fn doc_string(&self) -> Option<String> {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.doc_string(),
            AnyMDefinition::MClassDefinition(def) => def.doc_string(),
            AnyMDefinition::MReportDefinition(def) => def.doc_string(),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MFunctionDefinition {
    id: DefinitionId,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
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
    fn doc_string(&self) -> Option<String> {
        self.doc_string.clone()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassDefinition {
    id: DefinitionId,
    methods: Vec<MClassMethodDefinition>,
    description: Option<String>,
    doc_string: Option<String>,
    range: LineColRange,
    extends: Option<String>,
}
impl MClassDefinition {
    pub fn methods(&self) -> &Vec<MClassMethodDefinition> {
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
    fn doc_string(&self) -> Option<String> {
        self.doc_string.clone()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassMethodDefinition {
    id: DefinitionId,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
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

impl MClassMethodDefinition {
    pub fn is_constructor(&self) -> bool {
        self.m_type == MClassMethodType::Constructor
    }
}

impl Definition for MClassMethodDefinition {
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
    fn doc_string(&self) -> Option<String> {
        self.doc_string.clone()
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MReportDefinition {
    id: DefinitionId,
    sections: Vec<MReportSectionDefiniton>,
    range: LineColRange,
}

impl MReportDefinition {
    pub fn sections(&self) -> &Vec<MReportSectionDefiniton> {
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

    fn doc_string(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MReportSectionDefiniton {
    id: DefinitionId,
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

    fn doc_string(&self) -> Option<String> {
        None
    }
}

impl SemanticModel {
    fn visit_node(&mut self, index: &LineIndex, node: MSyntaxNode) {
        if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
            if let Some(def) = function_definition(func, index) {
                self.definitions
                    .push(AnyMDefinition::MFunctionDefinition(def));
            }
        }

        if let Some(class) = MClassDeclaration::cast(node.clone()) {
            if let Some(def) = class_definition(class, index) {
                self.definitions.push(AnyMDefinition::MClassDefinition(def));
            }
        }

        if let Some(report) = MReport::cast(node.clone()) {
            if let Some(def) = report_definition(report, index) {
                self.definitions
                    .push(AnyMDefinition::MReportDefinition(def));
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
        description: trivia_to_string(func.syntax().first_leading_trivia()),
        doc_string: func.doc_string().map(|s| s.text()),
        range: func_range,
    };
    Some(func)
}

fn class_definition(class: MClassDeclaration, index: &LineIndex) -> Option<MClassDefinition> {
    let class_id = class.id().ok()?;
    let class_id_range = index.line_col_range(class_id.range())?;
    let class_range = index.line_col_range(class.range())?;

    let methods = class
        .members()
        .iter()
        .filter_map(|member| class_member_definition(member, index))
        .collect();

    let class = MClassDefinition {
        id: DefinitionId {
            name: class_id.text(),
            range: class_id_range,
        },
        methods,
        description: trivia_to_string(class.syntax().first_leading_trivia()),
        doc_string: class.doc_string().map(|s| s.text()),
        range: class_range,
        extends: class
            .extends_clause()
            .map_or(None, |ext| Some(ext.super_class().ok()?.text())),
    };

    Some(class)
}

fn class_member_definition(
    member: AnyMClassMember,
    index: &LineIndex,
) -> Option<MClassMethodDefinition> {
    let member_id = member.name().ok()??;
    let member_id_range = index.line_col_range(member_id.range())?;
    let member_range = index.line_col_range(member.range())?;

    Some(MClassMethodDefinition {
        id: DefinitionId {
            name: member_id.text(),
            range: member_id_range,
        },
        params: member
            .params()
            .map(|params| params.map(|p| p.to_string()).unwrap_or(String::from("()")))
            .unwrap_or_default(),
        description: trivia_to_string(member.leading_trivia()),
        doc_string: member.doc_string().map(|s| s.text()),
        range: member_range,
        m_type: match member {
            AnyMClassMember::MConstructorClassMember(_) => MClassMethodType::Constructor,
            AnyMClassMember::MGetterClassMember(_) => MClassMethodType::Getter,
            AnyMClassMember::MSetterClassMember(_) => MClassMethodType::Setter,
            _ => MClassMethodType::Method,
        },
    })
}

fn report_definition(report: MReport, index: &LineIndex) -> Option<MReportDefinition> {
    let report_id = report.name().ok()?.m_name().ok()?;
    let report_id_range = index.line_col_range(report_id.range())?;
    let report_range = index.line_col_range(report.range())?;

    let sections = report
        .sections()
        .iter()
        .filter_map(|section| report_section_definition(section, index))
        .collect();

    let report = MReportDefinition {
        id: DefinitionId {
            name: report_id.text(),
            range: report_id_range,
        },
        sections,
        range: report_range,
    };

    Some(report)
}

fn report_section_definition(
    section: MReportSection,
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
        range: section_range,
    })
}

/// All trivia before the first non-whitespace trivia
fn trivia_to_string(trivia: Option<SyntaxTrivia<MLanguage>>) -> Option<String> {
    trivia
        .filter(|trivia| trivia.pieces().any(|piece| piece.kind().is_comment()))
        .map(|trivia| {
            let mut pieces = Vec::new();
            let mut newline_count = 0;

            for piece in trivia.pieces().rev() {
                match piece.kind() {
                    TriviaPieceKind::SingleLineComment => {
                        pieces.push(piece.text().to_string());
                        pieces.push(String::from("\r\n"));
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
        })
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
                description: Some(String::from("\r\n# something else\r\n# about function a")),
                doc_string: None,
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
                description: Some(String::from("\r\n# about function b")),
                doc_string: None,
                range: line_col_range(10, 4, 12, 5)
            })
        );

        assert_eq!(
            collector.definitions[2],
            AnyMDefinition::MClassDefinition(MClassDefinition {
                id: DefinitionId {
                    name: String::from("x"),
                    range: line_col_range(14, 10, 14, 11)
                },

                description: None,
                doc_string: None,
                range: line_col_range(14, 4, 24, 5),
                extends: Some("z".into()),
                methods: vec![
                    MClassMethodDefinition {
                        id: DefinitionId {
                            name: String::from("constructor"),
                            range: line_col_range(15, 8, 15, 19)
                        },

                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: line_col_range(15, 8, 15, 24),
                        m_type: MClassMethodType::Constructor
                    },
                    MClassMethodDefinition {
                        id: DefinitionId {
                            name: String::from("x"),
                            range: line_col_range(18, 12, 18, 13)
                        },

                        params: String::from("()"),
                        description: Some(String::from("\r\n# getter description")),
                        doc_string: None,
                        range: line_col_range(18, 8, 20, 9),
                        m_type: MClassMethodType::Getter
                    },
                    MClassMethodDefinition {
                        id: DefinitionId {
                            name: String::from("calc"),
                            range: line_col_range(21, 8, 21, 12)
                        },

                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: line_col_range(21, 8, 23, 9),
                        m_type: MClassMethodType::Method
                    }
                ]
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
            AnyMDefinition::MReportDefinition(MReportDefinition {
                id: DefinitionId {
                    name: String::from("CommonReport"),
                    range: line_col_range(1, 2, 1, 14)
                },
                range: line_col_range(1, 0, 18, 1),
                sections: vec![
                    MReportSectionDefiniton {
                        id: DefinitionId {
                            name: String::from("Function declaration"),
                            range: line_col_range(8, 1, 8, 21)
                        },
                        range: line_col_range(8, 0, 14, 1),
                    },
                    MReportSectionDefiniton {
                        id: DefinitionId {
                            name: String::from("print"),
                            range: line_col_range(15, 1, 15, 6)
                        },
                        range: line_col_range(15, 0, 18, 1),
                    },
                ]
            })
        );
    }
}
