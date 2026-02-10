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
        token?;
        let token = token.unwrap();
        if let Some(info) = identifier_for_token(token) {
            return Some(info);
        }
    }
    None
}

fn identifier_for_token(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if token.kind() == MSyntaxKind::IDENT {
        let ident = token.text_trimmed().trim().to_string();
        if let Some(node) = token.parent() {
            // take Reference
            if node.kind() == MSyntaxKind::M_REFERENCE_IDENTIFIER
                && let Some(info) = find_identifier_by_reference(node)
            {
                return Some(info);
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
                                if let Some(info) = find_identifier_by_reference(child)
                                    && let SemanticInfo::Reference(base_info) = info
                                    && let SemanticInfo::NewExpression(class_name) =
                                        base_info.as_ref()
                                {
                                    class_id = Some(class_name.to_string());
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
                true => SemanticInfo::ClassInstance(class_id),
                false => SemanticInfo::SuperCall(token.text_trimmed().trim().to_string(), class_id),
            };
            return Some(info);
        }
    }
    None
}

fn find_identifier_by_reference(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    let ident = node.text_trimmed().to_string().trim().to_lowercase();

    let mut parent: SyntaxNode<MLanguage> = node;
    while let Some(node) = parent.parent() {
        parent = node;
        if let Some(assignment_or_declaration) = get_firs_assignment_or_declaration(&parent, &ident)
            && let Some(right_side) =
                get_right_side_of_assignment_or_declaration(assignment_or_declaration)
        {
            return find_identifier_from_right_side(right_side);
        }
    }
    None
}

fn get_firs_assignment_or_declaration(
    parent: &SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    parent
        .siblings(biome_rowan::Direction::Prev)
        .skip(1)
        .filter_map(|n| match n.kind() {
            MSyntaxKind::M_EXPRESSION_STATEMENT => find_assignment_expression(n, ident),
            MSyntaxKind::M_VARIABLE_STATEMENT => find_variable_statement(n, ident),
            _ => None,
        })
        .next()
}

fn find_assignment_expression(
    epxpression_statement: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let first_assignment = epxpression_statement.first_child();
    first_assignment.as_ref()?;
    let mut first_assignment = first_assignment.unwrap();
    if first_assignment.kind() == MSyntaxKind::M_SEQUENCE_EXPRESSION {
        match first_assignment.first_child() {
            Some(child) => {
                first_assignment = child;
            }
            None => {
                return None;
            }
        }
    }

    let assignments = first_assignment.siblings(biome_rowan::Direction::Next);

    assignments
        .filter(|n| n.kind() == MSyntaxKind::M_ASSIGNMENT_EXPRESSION)
        .find(|n| {
            if let Some(ft) = n.first_token()
                && let Some(nt) = ft.next_token()
            {
                return ft.text_trimmed().to_string().trim().to_lowercase() == ident
                    && nt.kind() == MSyntaxKind::EQ;
            }
            false
        })
}

fn find_variable_statement(
    variable_statement: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let mut assignments = variable_statement
        .first_child()?
        .siblings(biome_rowan::Direction::Next)
        .find(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATION)?
        .first_child()?
        .siblings(biome_rowan::Direction::Next)
        .filter(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATOR_LIST)
        .flat_map(|n| {
            n.first_child()
                .map(|first_child| first_child.siblings(biome_rowan::Direction::Next))
                .into_iter()
                .flatten()
                .filter(|n| n.kind() == MSyntaxKind::M_VARIABLE_DECLARATOR)
        });

    assignments.find(|n| {
        n.first_token()
            .is_some_and(|ft| ft.text_trimmed().to_string().trim().to_lowercase() == ident)
    })
}

fn get_right_side_of_assignment_or_declaration(
    node: SyntaxNode<MLanguage>,
) -> Option<SyntaxNode<MLanguage>> {
    let right_side = node
        .first_child()?
        .siblings(biome_rowan::Direction::Next)
        .nth(1);

    if let Some(n) = right_side {
        // skeep initialize (only from declaration)
        return match n.kind() {
            MSyntaxKind::M_INITIALIZER_CLAUSE => n.first_child(),
            _ => Some(n),
        };
    }
    None
}

fn find_identifier_from_right_side(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    let info_token = match node.kind() {
        MSyntaxKind::M_CALL_EXPRESSION => {
            let method_name = node
                .first_child()? // M_STATIC_MEMBER_EXPRESSION
                .first_child()? // M_IDENTIFIER_EXPRESSION
                .siblings(biome_rowan::Direction::Next)
                .find(|n| n.kind() == MSyntaxKind::M_NAME);
            if let Some(n) = method_name {
                n.first_token()
            } else {
                node.first_token()
            }
        }
        MSyntaxKind::M_NEW_EXPRESSION => node.first_token()?.next_token(),
        _ => None,
    };

    if info_token.is_some()
        && let Some(identifier) = identifier_for_token(&info_token?)
    {
        return Some(SemanticInfo::Reference(Box::new(identifier)));
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
            (62, SemanticInfo::ClassInstance("Test".into())),
        ];

        for (offset, info) in offsets {
            let semantic_info =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(info, semantic_info);
        }
    }

    #[test]
    fn test_identifier_by_reference() {
        #[rustfmt::skip]
        let inputs = [
            ("var x = callFunction(); X ", 25, SemanticInfo::Reference(Box::new(SemanticInfo::FunctionCall("callFunction".to_owned())))),
            ("var x = z.callMethod(); x ", 25, SemanticInfo::Reference(Box::new(SemanticInfo::MethodCall("callMethod".to_owned(), None)))),
            ("var x = callFunction(); y = x + 3 ", 29, SemanticInfo::Reference(Box::new(SemanticInfo::FunctionCall("callFunction".to_owned())))),
            ("var x = new Tst(); x.callMethod() ", 20, SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))),
            ("var x = new Tst(); if (true) x.callMethod() ", 30, SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))),
            ("var a = 3, x = new Tst(); x ", 27, SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))),
            ("var x = 3; x = new Tst(); x ", 27, SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))),
            (
                "var x = new Tst(); x.a = 3; x ",
                29,
                SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))
            ),
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let semantic_info = identifier_for_token(&token).unwrap();
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_by_reference2() {
        #[rustfmt::skip]
        let inputs = [
            ("a = 3, x = new Tst(); x ", 23, SemanticInfo::Reference(Box::new(SemanticInfo::NewExpression("Tst".to_owned())))),
            ("x = callFunction(); x ", 21, SemanticInfo::Reference(Box::new(SemanticInfo::FunctionCall("callFunction".to_owned())))),
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let semantic_info = identifier_for_token(&token).unwrap();
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    fn get_token_from_offset(input: &str, offset: u32) -> SyntaxToken<MLanguage> {
        let parsed = parse(input, MFileSource::script());
        let syntax = parsed.syntax();
        let text_size_offset = TextSize::from(offset);
        let range = TextRange::new(text_size_offset, text_size_offset);
        let element = syntax.covering_element(range);
        let token = element.as_token().unwrap();
        token.clone()
    }
}
