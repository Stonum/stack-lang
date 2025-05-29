use crate::syntax::{MClassDeclaration, MLanguage, MSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, TextRange, TextSize};

#[derive(Debug, Eq, PartialEq)]
pub enum SemanticInfo {
    // like zzzzz();
    FunctionCall,

    // like z.cmethod() or this.method()
    MethodCall(Option<String>),

    // like new MyClass();
    NewExpression,
}

pub fn identifier_for_offset(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<(String, SemanticInfo)> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [
        offset,
        offset.checked_add(1.into()).unwrap_or(TextSize::default()),
        offset.checked_sub(1.into()).unwrap_or(TextSize::default()),
    ];

    for offset in offsets {
        let range = TextRange::new(offset, offset);
        let node = root.covering_element(range);

        let token = node.as_token();

        if token.is_none() {
            continue;
        }

        let token = token.unwrap();
        if token.kind() == MSyntaxKind::IDENT {
            let mut info = SemanticInfo::FunctionCall;

            if let Some(node) = node.parent() {
                // take nearest parents
                for n in node.ancestors().take(3) {
                    match n.kind() {
                        MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                            info = SemanticInfo::MethodCall(None);

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
                                    info = SemanticInfo::MethodCall(class_id);
                                }
                            }
                            break;
                        }
                        MSyntaxKind::M_NEW_EXPRESSION => {
                            info = SemanticInfo::NewExpression;
                            break;
                        }
                        MSyntaxKind::M_CALL_EXPRESSION => {
                            info = SemanticInfo::FunctionCall;
                            break;
                        }
                        _ => (),
                    };
                }
            }

            return Some((token.text_trimmed().to_string(), info));
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
            ("var x = callFunction()", 15, "callFunction", SemanticInfo::FunctionCall),
            ("var x = z.callMethod()", 15, "callMethod", SemanticInfo::MethodCall(None)),
            ("var x = new TodoClass()",15, "TodoClass", SemanticInfo::NewExpression),
            ("var x = callFunction( z.callMethod() )", 30, "callMethod", SemanticInfo::MethodCall(None)),
            ("var x = z.callMethod( callFunction() )", 30, "callFunction", SemanticInfo::FunctionCall),
            ("var x = z.callMethod( new TodoClass() )",30, "TodoClass", SemanticInfo::NewExpression),
            ("#comment line
              callaFterComment()",30, "callaFterComment", SemanticInfo::FunctionCall),
        ];

        for (input, offset, ident, info) in inputs {
            let parsed = parse(input, MFileSource::script());
            let (identifier, semantic_info) =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(
                (ident, info),
                (identifier.as_ref(), semantic_info),
                "{input}"
            );
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
            (65, "m2", SemanticInfo::MethodCall(Some("Test".into()))),
            (125, "m1", SemanticInfo::MethodCall(Some("Test".into()))),
        ];

        for (offset, ident, info) in offsets {
            let (identifier, semantic_info) =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!((ident, info), (identifier.as_ref(), semantic_info));
        }
    }
}
