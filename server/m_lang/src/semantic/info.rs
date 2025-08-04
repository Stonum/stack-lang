use crate::syntax::{MClassDeclaration, MLanguage, MSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, TextRange, TextSize};

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

pub fn identifier_for_offset(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<SemanticInfo> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [
        offset,
        offset.checked_add(1.into()).unwrap_or(TextSize::default()),
        offset.checked_sub(1.into()).unwrap_or(TextSize::default()),
    ];

    for offset in offsets {
        let range = TextRange::new(offset, offset);
        if !root.text_range().contains_range(range) {
            continue;
        }

        let node = root.covering_element(range);

        let token = node.as_token();

        if token.is_none() {
            continue;
        }

        let token = token.unwrap();
        if token.kind() == MSyntaxKind::IDENT {
            let ident = token.text_trimmed().trim().to_string();

            if let Some(node) = node.parent() {
                // take nearest parents
                for n in node.ancestors().take(3) {
                    match n.kind() {
                        MSyntaxKind::M_FUNCTION_DECLARATION => {
                            return Some(SemanticInfo::FunctionCall(ident));
                        }
                        MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                            if let Some(child) = n.first_child() {
                                // try find class name
                                if child.kind() == MSyntaxKind::M_THIS_EXPRESSION {
                                    let class_id = node
                                        .ancestors()
                                        .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                                        .map_or(None, |class_node| {
                                            let class = MClassDeclaration::cast(class_node)?;
                                            let id = class.id().ok()?.text();
                                            Some(id)
                                        });
                                    return Some(SemanticInfo::MethodCall(ident, class_id));
                                }
                            }
                            return Some(SemanticInfo::MethodCall(ident, None));
                        }
                        MSyntaxKind::M_NEW_EXPRESSION => {
                            return Some(SemanticInfo::NewExpression(ident));
                        }
                        MSyntaxKind::M_CALL_EXPRESSION => {
                            return Some(SemanticInfo::FunctionCall(ident));
                        }
                        MSyntaxKind::M_EXTENDS_CLAUSE => {
                            return Some(SemanticInfo::ClassExtends(ident));
                        }
                        _ => (),
                    };
                }
            }
        }

        if token.kind() == MSyntaxKind::SUPER_KW {
            let super_class_id = node
                .ancestors()
                .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                .map_or(None, |class_node| {
                    let class = MClassDeclaration::cast(class_node)?;
                    let id = class.extends_clause()?.super_class().ok()?.text();
                    Some(id)
                });
            if let Some(super_class) = super_class_id {
                let info =
                    SemanticInfo::SuperCall(token.text_trimmed().trim().to_string(), super_class);
                return Some(info);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse, syntax::MFileSource};

    use super::*;

    #[test]
    fn test_identifier_for_offset() {
        #[rustfmt::skip]
        let inputs = [
            ("var x = callFunction()", 15, SemanticInfo::FunctionCall("callFunction".to_owned())),
            ("var x = z.callMethod()", 15, SemanticInfo::MethodCall("callMethod".to_owned(), None)),
            ("var x = new TodoClass()",15, SemanticInfo::NewExpression("TodoClass".to_owned())),
            ("var x = callFunction( z.callMethod() )", 30, SemanticInfo::MethodCall("callMethod".to_owned(), None)),
            ("var x = z.callMethod( callFunction() )", 30, SemanticInfo::FunctionCall("callFunction".to_owned())),
            ("var x = z.callMethod( new TodoClass() )",30, SemanticInfo::NewExpression("TodoClass".to_owned())),
            ("#comment line
              callaFterComment()",30, SemanticInfo::FunctionCall("callaFterComment".to_owned())),
            ("class B extends A {}", 17, SemanticInfo::ClassExtends("A".to_owned())),
            ("class B extends A { constructor() { super() } }", 40, SemanticInfo::SuperCall("super".to_owned(), "A".to_owned()))
        ];

        for (input, offset, info) in inputs {
            let parsed = parse(input, MFileSource::script());
            let semantic_info =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_for_offset2() {
        let input = r#"
            class Test {
                constructor() { this.m2(); }
                m1() {}
                m2() { this.m1(); }
            }
        "#;
        let parsed = parse(input, MFileSource::script());

        let offsets = [
            (
                65,
                SemanticInfo::MethodCall("m2".to_owned(), Some("Test".into())),
            ),
            (
                125,
                SemanticInfo::MethodCall("m1".to_owned(), Some("Test".into())),
            ),
        ];

        for (offset, info) in offsets {
            let semantic_info =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(info, semantic_info);
        }
    }
}
