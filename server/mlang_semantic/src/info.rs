use biome_rowan::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize};
use mlang_lsp_definition::SemanticInfo;
use mlang_syntax::{
    AnyMAssignment, AnyMBinding, AnyMExpression, MAssignmentExpression, MCallExpression,
    MClassDeclaration, MExpressionStatement, MLanguage, MNewExpression, MSequenceExpression,
    MSyntaxKind, MVariableStatement,
};

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

pub fn identifier_for_completion(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<SemanticInfo> {
    let range = TextRange::new(offset, offset);
    if !root.text_range().contains_range(range) {
        return None;
    }
    let node = root.covering_element(range);
    if let Some(token) = node.as_token() {
        return identifier_for_token(token);
    }
    None
}

fn identifier_for_token(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if token.kind() == MSyntaxKind::IDENT {
        let ident = token.text_trimmed().trim().to_string();
        if let Some(node) = token.parent() {
            // try take Reference for identifier
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
                                    && let SemanticInfo::RefClass(class_name) = info
                                {
                                    class_id = Some(class_name);
                                };
                                return Some(SemanticInfo::MethodCall(ident, class_id));
                            }
                        }
                        return Some(SemanticInfo::MethodCall(ident, None));
                    }
                    MSyntaxKind::M_NEW_EXPRESSION => {
                        return Some(SemanticInfo::NewExpression(Some(ident)));
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
            let info = match token.kind() {
                MSyntaxKind::THIS_KW => SemanticInfo::RefClass(class_id),
                MSyntaxKind::SUPER_KW => {
                    SemanticInfo::SuperCall(token.text_trimmed().to_string(), class_id)
                }
                _ => unreachable!(),
            };
            return Some(info);
        }
    }

    if token.kind() == MSyntaxKind::NEW_KW {
        return Some(SemanticInfo::NewExpression(None));
    }
    None
}

fn find_identifier_by_reference(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    let ident = node.text_trimmed().to_string().to_lowercase();

    for parent in node.ancestors() {
        if let Some(right_side) = get_first_assignment_or_declaration(&parent, &ident) {
            return find_identifier_from_right_side(right_side);
        }
    }
    None
}

fn get_first_assignment_or_declaration(
    parent: &SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    parent
        .siblings(biome_rowan::Direction::Prev)
        .find_map(|n| match n.kind() {
            MSyntaxKind::M_EXPRESSION_STATEMENT => find_expression_statement(n, ident),
            MSyntaxKind::M_ASSIGNMENT_EXPRESSION => find_assignment_expression(n, ident),
            MSyntaxKind::M_VARIABLE_STATEMENT => find_variable_statement(n, ident),
            _ => None,
        })
}

fn find_expression_statement(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let expr = MExpressionStatement::cast(expression)?;
    match expr.expression().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    }
}

// find assignments from sequence expression
// example x = 1, y = 2;
fn find_sequence_expression(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let seq = MSequenceExpression::cast(expression)?;

    let left = match seq.left().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    };

    if left.is_some() {
        return left;
    }

    match seq.right().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    }
}

// return the right assignment expression
// example x = 1;
// where x == ident
fn find_assignment_expression(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let assignment = MAssignmentExpression::cast(expression)?;

    let left_ident = assignment.left().ok()?;
    let AnyMAssignment::MIdentifierAssignment(left_ident) = left_ident else {
        return None;
    };

    let right = assignment.right().ok()?;
    let left_name = left_ident.name_token().ok()?;

    if left_name.text_trimmed().to_string().to_lowercase() != ident {
        // find our ident in the right side of the assignment
        if matches!(right, AnyMExpression::MAssignmentExpression(_)) {
            return find_assignment_expression(right.into_syntax(), ident);
        }
        return None;
    }

    Some(right.into_syntax())
}

// return the right declaration expression
// example var x = 1;
// where x == ident
fn find_variable_statement(
    statement: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let var_statement = MVariableStatement::cast(statement)?;
    let var_declaration = var_statement.declaration().ok()?;

    var_declaration
        .declarators()
        .into_iter()
        .find_map(|declarator| {
            let declarator = declarator.ok()?;
            let binding = declarator.id().ok()?;
            let AnyMBinding::MIdentifierBinding(binding) = binding else {
                return None;
            };

            let binding_name = binding.name_token().ok()?;
            let expression = declarator.initializer()?.expression().ok()?;

            if binding_name.text_trimmed().to_string().to_lowercase() != ident {
                // find our ident in the right side of the assignment
                if matches!(expression, AnyMExpression::MAssignmentExpression(_)) {
                    return find_assignment_expression(expression.into_syntax(), ident);
                }
                return None;
            }

            Some(expression.into_syntax())
        })
}

fn find_identifier_from_right_side(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    let mut info_token = None;

    if let Some(assignment) = MAssignmentExpression::cast(node.clone()) {
        let mut right = assignment.right().ok()?;
        while let AnyMExpression::MAssignmentExpression(assignment) = right {
            right = assignment.right().ok()?;
        }
        return find_identifier_from_right_side(right.into_syntax());
    }

    if let Some(call_expression) = MCallExpression::cast(node.clone()) {
        let callee = call_expression.callee().ok()?;
        info_token = match callee {
            AnyMExpression::MIdentifierExpression(ident) => {
                let ident = ident.name().ok()?;
                Some(ident.value_token().ok()?)
            }
            AnyMExpression::MStaticMemberExpression(expr) => {
                let ident = expr.member().ok()?;
                Some(ident.value_token().ok()?)
            }
            _ => None,
        }
    }

    if let Some(new_expression) = MNewExpression::cast(node) {
        let class = new_expression.callee().ok()?;
        if let AnyMExpression::MIdentifierExpression(ident) = class {
            let ident = ident.name().ok()?;
            info_token = Some(ident.value_token().ok()?);
        }
    }

    let info_token = info_token?;
    let info = identifier_for_token(&info_token)?;

    match info {
        SemanticInfo::FunctionCall(ident) => Some(SemanticInfo::RefFunctionResult(ident)),
        SemanticInfo::NewExpression(Some(ident)) => Some(SemanticInfo::RefClass(ident)),
        SemanticInfo::MethodCall(ident, class) => Some(SemanticInfo::RefMethodResult(ident, class)),
        _ => None,
    }
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
            ("var x = new TodoClass()",15, SemanticInfo::NewExpression(Some("TodoClass".to_owned()))),
            ("var x = callFunction( z.callMethod() )", 30, SemanticInfo::MethodCall("callMethod".to_owned(), None)),
            ("var x = z.callMethod( callFunction() )", 30, SemanticInfo::FunctionCall("callFunction".to_owned())),
            ("var x = z.callMethod( new TodoClass() )",30, SemanticInfo::NewExpression(Some("TodoClass".to_owned()))),
            ("#comment line
              callaFterComment()",30, SemanticInfo::FunctionCall("callaFterComment".to_owned())),
            ("class B extends A {}", 17, SemanticInfo::ClassExtends("A".to_owned())),
            ("class B extends A { constructor() { super() } }", 40, SemanticInfo::SuperCall("super".to_owned(), "A".to_owned())),
            ("forall( iterator(arr, ind)) {}", 15, SemanticInfo::FunctionCall("iterator".to_owned())),
            ("new ", 2, SemanticInfo::NewExpression(None)),
        ];

        for (input, offset, info) in inputs {
            let parsed = parse(input, MFileSource::script());
            let semantic_info = identifier_for_offset(parsed.syntax(), TextSize::from(offset))
                .unwrap_or_else(|| panic!("failed for `{input}`"));
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_for_offset_in_class_delcaration() {
        let input = r#"
            class Test {
                constructor() { this.m2(); }
                m1() {}
                m2() { this.m1(); }
            }
        "#;
        let parsed = parse(input, MFileSource::script());

        #[rustfmt::skip]
        let offsets = [
            (65, SemanticInfo::MethodCall("m2".to_owned(), Some("Test".into()))),
            (125, SemanticInfo::MethodCall("m1".to_owned(), Some("Test".into()))),
            (62, SemanticInfo::RefClass("Test".into())),
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
            ("var x = callFunction(); X ", 25, SemanticInfo::RefFunctionResult("callFunction".to_owned())),
            ("var x = z.callMethod(); x ", 25, SemanticInfo::RefMethodResult("callMethod".to_owned(), None)),
            ("var x = callFunction(); y = x + 3 ", 29, SemanticInfo::RefFunctionResult("callFunction".to_owned())),
            ("var x = new Tst(); x.callMethod() ", 20, SemanticInfo::RefClass("Tst".to_owned())),
            ("var x = new Tst(); if (true) x.callMethod() ", 30, SemanticInfo::RefClass("Tst".to_owned())),
            ("var a = 3, x = new Tst(); x ", 27, SemanticInfo::RefClass("Tst".to_owned())),
            ("var x = 3; x = new Tst(); x ", 27, SemanticInfo::RefClass("Tst".to_owned())),
            ("var x = new Tst(); x.a = 3; x ", 29, SemanticInfo::RefClass("Tst".to_owned())),
            ("a = 3, x = new Tst(); x ", 23, SemanticInfo::RefClass("Tst".to_owned())),
            ("x = new Tst(), a = 3; x ", 23, SemanticInfo::RefClass("Tst".to_owned())),
            ("x = callFunction(); x ", 21, SemanticInfo::RefFunctionResult("callFunction".to_owned())),
            ("var y = z = x = new Tst(); x ", 28, SemanticInfo::RefClass("Tst".to_owned()))
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let semantic_info =
                identifier_for_token(&token).unwrap_or_else(|| panic!("failed for `{input}`"));
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
