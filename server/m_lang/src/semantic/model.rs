use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{AstNode, AstNodeList, SyntaxNode, TextRange, TriviaPieceKind, WalkEvent};

use crate::syntax::{
    AnyMClassMember, MClassDeclaration, MFileSource, MFunctionDeclaration, MLanguage, MReport,
    MSyntaxNode, ModuleKind,
};

pub fn semantics(root: SyntaxNode<MLanguage>, source: MFileSource) -> SemanticModel {
    let mut collector = SemanticModel {
        definitions: vec![],
        kind: source.module_kind(),
    };

    for event in root.preorder() {
        if let WalkEvent::Enter(node) = event {
            collector.visit_node(node);
        }
    }

    collector
}

#[derive(Debug, Default)]
pub struct SemanticModel {
    pub definitions: Vec<AnyMDefinition>,
    pub kind: ModuleKind,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMDefinition {
    MFunctionDefinition(MFunctionDefinition),
    MClassDefinition(MClassDefinition),
    MReportDefinition(MReportDefinition),
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
    fn range(&self) -> TextRange;
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
    fn range(&self) -> TextRange {
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
    id: String,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
}
impl Definition for MFunctionDefinition {
    fn range(&self) -> TextRange {
        self.range
    }
    fn type_keyword(&self) -> &'static str {
        "function"
    }
    fn id(&self) -> String {
        self.id.clone()
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
    id: String,
    methods: Vec<MClassMethodDefinition>,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
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
    fn range(&self) -> TextRange {
        self.range
    }
    fn type_keyword(&self) -> &'static str {
        "class"
    }
    fn id(&self) -> String {
        self.id.clone()
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
    id: String,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
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

impl Definition for MClassMethodDefinition {
    fn range(&self) -> TextRange {
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
        self.id.clone()
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
    id: String,
    sections: Vec<MReportSectionDefiniton>,
    range: TextRange,
}

impl MReportDefinition {
    pub fn sections(&self) -> &Vec<MReportSectionDefiniton> {
        &self.sections
    }
}

impl Definition for MReportDefinition {
    fn range(&self) -> TextRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "report"
    }

    fn id(&self) -> String {
        self.id.clone()
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
    id: String,
    range: TextRange,
}

impl Definition for MReportSectionDefiniton {
    fn range(&self) -> TextRange {
        self.range
    }

    fn type_keyword(&self) -> &'static str {
        "section"
    }

    fn id(&self) -> String {
        self.id.clone()
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
    fn visit_node(&mut self, node: MSyntaxNode) {
        // Обработка Function Declaration
        if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
            if let Ok(name) = func.id() {
                self.definitions
                    .push(AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                        id: name.text(),
                        params: func
                            .parameters()
                            .map(|params| params.to_string().trim().to_string())
                            .unwrap_or_default(),
                        description: Self::trivia_to_string(func.syntax().first_leading_trivia()),
                        doc_string: func.doc_string().map(|s| s.text()),
                        range: name.range(),
                    }));
            }
        }

        // Обработка Class Declaration
        if let Some(class) = MClassDeclaration::cast(node.clone()) {
            let members = class.members();

            let id = if let Ok(id) = class.id() {
                id.text()
            } else {
                String::from("<unnamed class>")
            };

            let mut class = MClassDefinition {
                id,
                methods: Vec::with_capacity(members.len()),
                description: Self::trivia_to_string(class.syntax().first_leading_trivia()),
                doc_string: class.doc_string().map(|s| s.text()),
                range: class.id().map(|id| id.range()).unwrap_or(class.range()),
                extends: class
                    .extends_clause()
                    .map_or(None, |ext| Some(ext.super_class().ok()?.text())),
            };

            for member in members {
                if let Ok(Some(name)) = member.name() {
                    let method_definition = MClassMethodDefinition {
                        id: name.text(),
                        params: member
                            .params()
                            .map(|params| {
                                params.map(|p| p.to_string()).unwrap_or(String::from("()"))
                            })
                            .unwrap_or_default(),
                        description: Self::trivia_to_string(member.leading_trivia()),
                        doc_string: member.doc_string().map(|s| s.text()),
                        range: name.range(),
                        m_type: match member {
                            AnyMClassMember::MConstructorClassMember(_) => {
                                MClassMethodType::Constructor
                            }
                            AnyMClassMember::MGetterClassMember(_) => MClassMethodType::Getter,
                            AnyMClassMember::MSetterClassMember(_) => MClassMethodType::Setter,
                            _ => MClassMethodType::Method,
                        },
                    };
                    class.methods.push(method_definition);
                }
            }

            self.definitions
                .push(AnyMDefinition::MClassDefinition(class));
        }

        // Обработка Report file
        if let Some(report) = MReport::cast(node.clone()) {
            let report_name = report.name().and_then(|n| n.m_name());

            let (id, range) = report_name
                .map(|n| (n.text(), n.range()))
                .unwrap_or_else(|_| (String::from("<unnamed report>"), report.range()));

            let sections = report
                .sections()
                .iter()
                .filter_map(|section| {
                    let name = section.name().ok()?;
                    let name = name.m_name().ok()?;

                    Some(MReportSectionDefiniton {
                        id: name.text(),
                        range: name.range(),
                    })
                })
                .collect();

            let report = MReportDefinition {
                id,
                sections,
                range,
            };

            self.definitions
                .push(AnyMDefinition::MReportDefinition(report));
        }
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
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse, syntax::MFileSource};

    use super::*;

    #[test]
    fn test_convert_to_definitions() {
        let parsed = parse(
            r#"
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
"#,
            MFileSource::module(),
        );

        let collector = semantics(parsed.syntax(), MFileSource::module());

        assert!(!collector.definitions.is_empty());

        assert_eq!(
            collector.definitions[0],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: String::from("a"),
                params: String::from("(x, y, z = 5, ...)"),
                description: Some(String::from("\r\n# something else\r\n# about function a")),
                doc_string: None,
                range: TextRange::new(60.into(), 61.into()),
            })
        );

        assert_eq!(
            collector.definitions[1],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: String::from("b"),
                params: String::from("()"),
                description: Some(String::from("\r\n# about function b")),
                doc_string: None,
                range: TextRange::new(125.into(), 126.into()),
            })
        );

        assert_eq!(
            collector.definitions[2],
            AnyMDefinition::MClassDefinition(MClassDefinition {
                id: String::from("x"),
                description: None,
                doc_string: None,
                range: TextRange::new(158.into(), 159.into()),
                extends: Some("z".into()),
                methods: vec![
                    MClassMethodDefinition {
                        id: String::from("constructor"),
                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: TextRange::new(176.into(), 187.into()),
                        m_type: MClassMethodType::Constructor
                    },
                    MClassMethodDefinition {
                        id: String::from("x"),
                        params: String::from("()"),
                        description: Some(String::from("\r\n# getter description")),
                        doc_string: None,
                        range: TextRange::new(227.into(), 228.into()),
                        m_type: MClassMethodType::Getter
                    },
                    MClassMethodDefinition {
                        id: String::from("calc"),
                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: TextRange::new(263.into(), 267.into()),
                        m_type: MClassMethodType::Method
                    }
                ]
            })
        );
    }

    #[test]
    fn test_convert_report_to_definitions() {
        let parsed = parse(
            r#"#
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
"#,
            MFileSource::report(),
        );

        let collector = semantics(parsed.syntax(), MFileSource::report());

        assert!(!collector.definitions.is_empty());

        assert_eq!(
            collector.definitions[0],
            AnyMDefinition::MReportDefinition(MReportDefinition {
                id: String::from("CommonReport"),
                range: TextRange::new(4.into(), 16.into()),
                sections: vec![
                    MReportSectionDefiniton {
                        id: String::from("Function declaration"),
                        range: TextRange::new(119.into(), 139.into()),
                    },
                    MReportSectionDefiniton {
                        id: String::from("print"),
                        range: TextRange::new(195.into(), 200.into()),
                    },
                ]
            })
        );
    }
}
