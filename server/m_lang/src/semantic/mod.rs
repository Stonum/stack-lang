use std::fmt::Display;

use biome_rowan::syntax::SyntaxTrivia;
use biome_rowan::{
    AstNode, SyntaxNode, SyntaxResult, TextRange, TextSize, TriviaPieceKind, WalkEvent,
};

use crate::syntax::{MClassDeclaration, MFunctionDeclaration, MLanguage, MSyntaxKind, MSyntaxNode};

pub fn semantics(root: SyntaxNode<MLanguage>) -> SemanticModel {
    let mut collector = SemanticModel::default();

    for event in root.preorder() {
        if let WalkEvent::Enter(node) = event {
            collector.visit_node(node);
        }
    }

    collector
}

pub fn identifier_for_offset(root: SyntaxNode<MLanguage>, offset: u32) -> Option<String> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [offset, offset.saturating_add(1), offset.saturating_sub(1)];

    for offset in offsets {
        let token = root
            .covering_element(TextRange::new(
                TextSize::from(offset),
                TextSize::from(offset),
            ))
            .into_token();

        if token.is_none() {
            continue;
        }

        let token = token.unwrap();
        if token.kind() == MSyntaxKind::IDENT {
            return Some(token.text().to_string());
        }
    }

    None
}

#[derive(Debug, Default)]
pub struct SemanticModel {
    pub module_definitions: Vec<AnyMDefinition>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AnyMDefinition {
    MFunctionDefinition(MFunctionDefinition),
    MClassDefinition(MClassDefinition),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MFunctionDefinition {
    id: String,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassDefinition {
    id: String,
    methods: Vec<MClassMethodDefinition>,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MClassMethodDefinition {
    id: String,
    params: String,
    description: Option<String>,
    doc_string: Option<String>,
    range: TextRange,
}

pub trait Definition {
    fn range(&self) -> TextRange;
    fn id(&self) -> String;
    fn params(&self) -> String;
    fn description(&self) -> Option<String>;
    fn doc_string(&self) -> Option<String>;

    fn to_markdown(&self) -> String {
        format!(
            "**{}**{}\n{}\n{}",
            self.id(),
            self.params(),
            self.description().unwrap_or_default().replace("#", "\\#"),
            self.doc_string()
                .map(|s| s[1..s.len() - 1].to_string())
                .unwrap_or_default()
        )
    }
}

impl Definition for AnyMDefinition {
    fn range(&self) -> TextRange {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.range(),
            AnyMDefinition::MClassDefinition(def) => def.range(),
        }
    }
    fn id(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.id(),
            AnyMDefinition::MClassDefinition(def) => def.id(),
        }
    }
    fn params(&self) -> String {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.params(),
            AnyMDefinition::MClassDefinition(def) => def.params(),
        }
    }
    fn description(&self) -> Option<String> {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.description(),
            AnyMDefinition::MClassDefinition(def) => def.description(),
        }
    }
    fn doc_string(&self) -> Option<String> {
        match self {
            AnyMDefinition::MFunctionDefinition(def) => def.doc_string(),
            AnyMDefinition::MClassDefinition(def) => def.doc_string(),
        }
    }
}

impl Definition for MFunctionDefinition {
    fn range(&self) -> TextRange {
        self.range
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

impl MClassDefinition {
    pub fn methods(&self) -> &Vec<MClassMethodDefinition> {
        &self.methods
    }
}

impl Definition for MClassDefinition {
    fn range(&self) -> TextRange {
        self.range
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

impl Definition for MClassMethodDefinition {
    fn range(&self) -> TextRange {
        self.range
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

impl SemanticModel {
    fn visit_node(&mut self, node: MSyntaxNode) {
        // Обработка Function Declaration
        if let Some(func) = MFunctionDeclaration::cast(node.clone()) {
            if let Ok(name) = func.id() {
                self.module_definitions
                    .push(AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                        id: name.text(),
                        params: Self::parameters_to_string(func.parameters()),
                        description: Self::trivia_to_string(func.syntax().first_leading_trivia()),
                        doc_string: func.doc_string().map(|s| s.text()),
                        range: func.range(),
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
                methods: Vec::new(),
                description: Self::trivia_to_string(class.syntax().first_leading_trivia()),
                doc_string: class.doc_string().map(|s| s.text()),
                range: class.range(),
            };

            for member in members {
                use crate::syntax::AnyMClassMember::*;
                match member {
                    MConstructorClassMember(constructor) if constructor.name().is_ok() => {
                        class.methods.push(MClassMethodDefinition {
                            id: constructor.name().unwrap().text(),
                            params: Self::parameters_to_string(constructor.parameters()),
                            description: Self::trivia_to_string(
                                constructor.syntax().first_leading_trivia(),
                            ),
                            doc_string: constructor.doc_string().map(|s| s.text()),
                            range: constructor.range(),
                        });
                    }
                    MMethodClassMember(method) if method.name().is_ok() => {
                        class.methods.push(MClassMethodDefinition {
                            id: method.name().unwrap().text(),
                            params: Self::parameters_to_string(method.parameters()),
                            description: Self::trivia_to_string(
                                method.syntax().first_leading_trivia(),
                            ),
                            doc_string: method.doc_string().map(|s| s.text()),
                            range: method.range(),
                        });
                    }
                    MGetterClassMember(getter) if getter.name().is_ok() => {
                        class.methods.push(MClassMethodDefinition {
                            id: getter.name().unwrap().text(),
                            params: String::from("()"),
                            description: Self::trivia_to_string(
                                getter.syntax().first_leading_trivia(),
                            ),
                            doc_string: getter.doc_string().map(|s| s.text()),
                            range: getter.range(),
                        });
                    }
                    MSetterClassMember(setter) if setter.name().is_ok() => {
                        class.methods.push(MClassMethodDefinition {
                            id: setter.name().unwrap().text(),
                            params: Self::parameters_to_string(setter.parameter()),
                            description: Self::trivia_to_string(
                                setter.syntax().first_leading_trivia(),
                            ),
                            doc_string: setter.doc_string().map(|s| s.text()),
                            range: setter.range(),
                        });
                    }
                    _ => continue,
                }
            }

            self.module_definitions
                .push(AnyMDefinition::MClassDefinition(class));
        }
    }

    #[inline]
    fn parameters_to_string<T>(parameters: SyntaxResult<T>) -> String
    where
        T: AstNode + Display,
    {
        if let Ok(params) = parameters {
            params.to_string().trim().to_string()
        } else {
            String::from("()")
        }
    }

    /// All trivia before the first non-whitespace trivia
    #[inline]
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
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse, syntax::MFileSource};

    use super::*;

    #[test]
    fn test() {
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

class x {
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

        let collector = semantics(parsed.syntax());

        assert!(!collector.module_definitions.is_empty());

        assert_eq!(
            collector.module_definitions[0],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: String::from("a"),
                params: String::from("(x, y, z = 5, ...)"),
                description: Some(String::from("\n# something else\n# about function a")),
                doc_string: None,
                range: TextRange::new(55.into(), 99.into()),
            })
        );

        assert_eq!(
            collector.module_definitions[1],
            AnyMDefinition::MFunctionDefinition(MFunctionDefinition {
                id: String::from("b"),
                params: String::from("()"),
                description: Some(String::from("\n# about function b")),
                doc_string: None,
                range: TextRange::new(120.into(), 150.into()),
            })
        );

        assert_eq!(
            collector.module_definitions[2],
            AnyMDefinition::MClassDefinition(MClassDefinition {
                id: String::from("x"),
                description: None,
                doc_string: None,
                range: TextRange::new(152.into(), 299.into()),
                methods: vec![
                    MClassMethodDefinition {
                        id: String::from("constructor"),
                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: TextRange::new(166.into(), 182.into()),
                    },
                    MClassMethodDefinition {
                        id: String::from("x"),
                        params: String::from("()"),
                        description: Some(String::from("\n# getter description")),
                        doc_string: None,
                        range: TextRange::new(213.into(), 247.into()),
                    },
                    MClassMethodDefinition {
                        id: String::from("calc"),
                        params: String::from("()"),
                        description: None,
                        doc_string: None,
                        range: TextRange::new(253.into(), 297.into()),
                    }
                ]
            })
        );
    }
}
