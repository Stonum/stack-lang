use biome_rowan::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize};
use mlang_lsp_definition::SemanticInfo;
use mlang_syntax::{MClassDeclaration, MLanguage, MSyntaxKind};

pub fn identifier_for_offset(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<SemanticInfo> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [
        offset,
        offset.checked_add(1.into()).unwrap_or_default(),
        offset.checked_sub(1.into()).unwrap_or_default(),
    ];

    for offset in offsets {
        let range = TextRange::new(offset, offset);
        if !root.text_range().contains_range(range) {
            continue;
        }
        let node = root.covering_element(range);
        let token = node.as_token();
        if token.is_none() {
            return None;
        }
        let token = token.unwrap();
        match identifier_for_token(token) {
            Some(info) => {
                return Some(info);
            }
            None => {}
        }
    }
    None
}

fn identifier_for_token(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if token.kind() == MSyntaxKind::IDENT {
        let ident = token.text_trimmed().trim().to_string();
        if let Some(node) = token.parent() {
            if node.kind() == MSyntaxKind::M_REFERENCE_IDENTIFIER {
                match find_identifier_by_referense(node) {
                    Some(info) => {
                        return Some(info);
                    }
                    None => {}
                }
            }
            // take nearest parents
            for n in token.ancestors().take(3) {
                match n.kind() {
                    MSyntaxKind::M_FUNCTION_DECLARATION => {
                        return Some(SemanticInfo::FunctionDeclaration(ident));
                    }
                    MSyntaxKind::M_CLASS_DECLARATION => {
                        return Some(SemanticInfo::ClassDeclaration(ident));
                    }
                    MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                        let class_member_list_node = n.parent()?;
                        let class_node = class_member_list_node.parent()?;

                        let class = MClassDeclaration::cast(class_node)?;
                        let class_id = class.id().ok()?.text();

                        return Some(SemanticInfo::MethodDeclaration(ident, class_id));
                    }
                    MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                        if let Some(child) = n.first_child() {
                            // try find class name
                            if child.kind() == MSyntaxKind::M_THIS_EXPRESSION
                                || child.kind() == MSyntaxKind::M_SUPER_EXPRESSION
                            {
                                let class_id = token
                                    .ancestors()
                                    .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                                    .and_then(|class_node| {
                                        let class = MClassDeclaration::cast(class_node)?;
                                        let id = match child.kind()
                                            == MSyntaxKind::M_THIS_EXPRESSION
                                        {
                                            true => class.id().ok()?.text(),
                                            false => {
                                                class.extends_clause()?.super_class().ok()?.text()
                                            }
                                        };
                                        Some(id)
                                    });
                                return Some(SemanticInfo::MethodCall(ident, class_id));
                            }
                            if child.kind() == MSyntaxKind::M_IDENTIFIER_EXPRESSION {
                                let mut class_id: Option<String> = None;
                                match find_identifier_by_referense(child) {
                                    Some(info) => match info {
                                        SemanticInfo::Referense(base_info) => {
                                            match base_info.as_ref() {
                                                SemanticInfo::NewExpression(class_name) => {
                                                    class_id = Some(class_name.to_string());
                                                }
                                                _ => (),
                                            }
                                        }
                                        _ => (),
                                    },
                                    None => (),
                                };
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
                    MSyntaxKind::M_FOR_ITERATOR_FACTORY => {
                        return Some(SemanticInfo::FunctionCall(ident));
                    }
                    _ => (),
                };
            }
        }
    }

    if token.kind() == MSyntaxKind::SUPER_KW || token.kind() == MSyntaxKind::THIS_KW {
        let class_id = token
            .ancestors()
            .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
            .and_then(|class_node| {
                let class = MClassDeclaration::cast(class_node)?;
                let id = match token.kind() == MSyntaxKind::THIS_KW {
                    true => class.id().ok()?.text(),
                    false => class.extends_clause()?.super_class().ok()?.text(),
                };
                Some(id)
            });
        if let Some(class_id) = class_id {
            let info = match token.kind() == MSyntaxKind::THIS_KW {
                true => SemanticInfo::ThisCall(token.text_trimmed().trim().to_string(), class_id),
                false => SemanticInfo::SuperCall(token.text_trimmed().trim().to_string(), class_id),
            };
            return Some(info);
        }
    }
    None
}

fn find_identifier_by_referense(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    let ident = node.text_trimmed().to_string().trim().to_lowercase();

    if let Some(node) = node.parent() {
        let mut parent: SyntaxNode<MLanguage> = node;
        while let Some(node) = parent.parent() {
            parent = node;
            let assignment = parent
                .siblings_with_tokens(biome_rowan::Direction::Prev)
                .skip(1)
                .filter_map(|e| e.into_node())
                .filter_map(|n| {
                    if n.kind() == MSyntaxKind::M_EXPRESSION_STATEMENT {
                        let mut first_assignment = n.first_child().unwrap();
                        if first_assignment.kind() == MSyntaxKind::M_SEQUENCE_EXPRESSION {
                            first_assignment = first_assignment
                                .first_child()
                                .unwrap()
                                .siblings(biome_rowan::Direction::Next)
                                .next()
                                .unwrap();
                        }
                        let assignments = first_assignment.siblings(biome_rowan::Direction::Next);

                        return assignments
                            .filter(|n| n.kind() == MSyntaxKind::M_ASSIGNMENT_EXPRESSION)
                            .filter(|n| {
                                n.first_token()
                                    .unwrap()
                                    .text_trimmed()
                                    .trim()
                                    .to_lowercase()
                                    == ident
                            })
                            .next();
                    }

                    if n.kind() == MSyntaxKind::M_VARIABLE_STATEMENT {
                        let assignments = n
                            .first_child()
                            .unwrap()
                            .siblings(biome_rowan::Direction::Next)
                            .filter(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATION)
                            .next()
                            .unwrap()
                            .first_child()
                            .unwrap()
                            .siblings(biome_rowan::Direction::Next)
                            .filter(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATOR_LIST)
                            .flat_map(|n| {
                                n.first_child()
                                    .unwrap()
                                    .siblings(biome_rowan::Direction::Next)
                                    .filter(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATOR)
                            });

                        return assignments
                            .filter(|n| {
                                n.first_token()
                                    .unwrap()
                                    .text_trimmed()
                                    .trim()
                                    .to_lowercase()
                                    == ident
                            })
                            .next();
                    }
                    None
                })
                .next();
            match assignment {
                Some(n) => {
                    let right_side = n
                        .first_child()
                        .unwrap()
                        .siblings_with_tokens(biome_rowan::Direction::Next)
                        .filter_map(|e| e.into_node())
                        .skip(1)
                        .next()
                        .unwrap()
                        .siblings_with_tokens(biome_rowan::Direction::Next)
                        .filter_map(|e| e.into_node())
                        .next();

                    match right_side {
                        Some(n) => {
                            let mut node = n.first_child().unwrap();
                            // get method name
                            if node.kind() == MSyntaxKind::M_CALL_EXPRESSION {
                                let method_name = n
                                    .first_child()
                                    .unwrap()
                                    .first_child()
                                    .unwrap()
                                    .first_child()
                                    .unwrap()
                                    .siblings(biome_rowan::Direction::Next)
                                    .filter(|n| n.kind() == MSyntaxKind::M_NAME)
                                    .next();
                                match method_name {
                                    Some(name) => node = name,
                                    None => {}
                                }
                            }
                            // skeep initialize
                            if node.kind() == MSyntaxKind::M_INITIALIZER_CLAUSE {
                                node = node.first_child().unwrap();
                            }
                            // skeep new
                            let mut ft = node.first_token().unwrap();
                            if ft.kind() == MSyntaxKind::NEW_KW {
                                ft = ft.next_token().unwrap();
                            }
                            let identifier = identifier_for_token(&ft);
                            match identifier {
                                Some(i) => {
                                    return Some(SemanticInfo::Referense(Box::new(i)));
                                }
                                None => {}
                            }
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

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
            ("class B extends A { constructor() { super() } }", 40, SemanticInfo::SuperCall("super".to_owned(), "A".to_owned())),
            ("forall( iterator(arr, ind)) {}", 15, SemanticInfo::FunctionCall("iterator".to_owned()))
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
