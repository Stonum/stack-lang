//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
use crate::syntax::{
    MSyntaxElement as SyntaxElement, MSyntaxNode as SyntaxNode, MSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn m_annotation_attribute(
    name: AnyMBinding,
    eq_token: SyntaxToken,
    value: AnyMLiteralExpression,
) -> MAnnotationAttribute {
    MAnnotationAttribute::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_ATTRIBUTE,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn m_annotation_binding(name: AnyMBinding) -> MAnnotationBinding {
    MAnnotationBinding::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_BINDING,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn m_annotation_element(
    name: AnyMBinding,
    l_paren_token: SyntaxToken,
    attributes: MAnnotationAttributeList,
    r_paren_token: SyntaxToken,
) -> MAnnotationElement {
    MAnnotationElement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_ELEMENT,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(attributes.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_annotation_group(
    colon_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    elements: MAnnotationList,
    r_brack_token: SyntaxToken,
) -> MAnnotationGroup {
    MAnnotationGroup::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_GROUP,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn m_array_expression(
    at_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    elements: MArrayElementList,
    r_brack_token: SyntaxToken,
) -> MArrayExpression {
    MArrayExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ARRAY_EXPRESSION,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn m_array_hole() -> MArrayHole {
    MArrayHole::unwrap_cast(SyntaxNode::new_detached(MSyntaxKind::M_ARRAY_HOLE, []))
}
pub fn m_assignment_expression(
    left: AnyMAssignment,
    operator_token_token: SyntaxToken,
    right: AnyMExpression,
) -> MAssignmentExpression {
    MAssignmentExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ASSIGNMENT_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn m_binary_expression(
    left: AnyMExpression,
    operator_token_token: SyntaxToken,
    right: AnyMExpression,
) -> MBinaryExpression {
    MBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn m_block_statement(
    l_curly_token: SyntaxToken,
    statements: MStatementList,
    r_curly_token: SyntaxToken,
) -> MBlockStatement {
    MBlockStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BLOCK_STATEMENT,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn m_boolean_literal_expression(value_token_token: SyntaxToken) -> MBooleanLiteralExpression {
    MBooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOOLEAN_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn m_break_statement(break_token: SyntaxToken) -> MBreakStatementBuilder {
    MBreakStatementBuilder {
        break_token,
        semicolon_token: None,
    }
}
pub struct MBreakStatementBuilder {
    break_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl MBreakStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MBreakStatement {
        MBreakStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_BREAK_STATEMENT,
            [
                Some(SyntaxElement::Token(self.break_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_call_arguments(
    l_paren_token: SyntaxToken,
    args: MCallArgumentList,
    r_paren_token: SyntaxToken,
) -> MCallArguments {
    MCallArguments::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CALL_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(args.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_call_expression(callee: AnyMExpression, arguments: MCallArguments) -> MCallExpression {
    MCallExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CALL_EXPRESSION,
        [
            Some(SyntaxElement::Node(callee.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn m_case_clause(
    case_token: SyntaxToken,
    test: AnyMExpression,
    colon_token: SyntaxToken,
    consequent: MStatementList,
) -> MCaseClause {
    MCaseClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CASE_CLAUSE,
        [
            Some(SyntaxElement::Token(case_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
        ],
    ))
}
pub fn m_catch_clause(catch_token: SyntaxToken, body: MBlockStatement) -> MCatchClauseBuilder {
    MCatchClauseBuilder {
        catch_token,
        body,
        declaration: None,
    }
}
pub struct MCatchClauseBuilder {
    catch_token: SyntaxToken,
    body: MBlockStatement,
    declaration: Option<MCatchDeclaration>,
}
impl MCatchClauseBuilder {
    pub fn with_declaration(mut self, declaration: MCatchDeclaration) -> Self {
        self.declaration = Some(declaration);
        self
    }
    pub fn build(self) -> MCatchClause {
        MCatchClause::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_CATCH_CLAUSE,
            [
                Some(SyntaxElement::Token(self.catch_token)),
                self.declaration
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_catch_declaration(
    l_paren_token: SyntaxToken,
    binding: AnyMBinding,
    r_paren_token: SyntaxToken,
) -> MCatchDeclaration {
    MCatchDeclaration::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CATCH_DECLARATION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(binding.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_class_declaration(
    annotation: MAnnotationGroupList,
    class_token: SyntaxToken,
    id: AnyMBinding,
    l_curly_token: SyntaxToken,
    members: MClassMemberList,
    r_curly_token: SyntaxToken,
) -> MClassDeclarationBuilder {
    MClassDeclarationBuilder {
        annotation,
        class_token,
        id,
        l_curly_token,
        members,
        r_curly_token,
        extends_clause: None,
        doc_string: None,
    }
}
pub struct MClassDeclarationBuilder {
    annotation: MAnnotationGroupList,
    class_token: SyntaxToken,
    id: AnyMBinding,
    l_curly_token: SyntaxToken,
    members: MClassMemberList,
    r_curly_token: SyntaxToken,
    extends_clause: Option<MExtendsClause>,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MClassDeclarationBuilder {
    pub fn with_extends_clause(mut self, extends_clause: MExtendsClause) -> Self {
        self.extends_clause = Some(extends_clause);
        self
    }
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MClassDeclaration {
        MClassDeclaration::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_CLASS_DECLARATION,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Token(self.class_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.extends_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_curly_token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn m_computed_member_assignment(
    l_brack_token: SyntaxToken,
    member: AnyMExpression,
    r_brack_token: SyntaxToken,
) -> MComputedMemberAssignmentBuilder {
    MComputedMemberAssignmentBuilder {
        l_brack_token,
        member,
        r_brack_token,
        object: None,
    }
}
pub struct MComputedMemberAssignmentBuilder {
    l_brack_token: SyntaxToken,
    member: AnyMExpression,
    r_brack_token: SyntaxToken,
    object: Option<AnyMExpression>,
}
impl MComputedMemberAssignmentBuilder {
    pub fn with_object(mut self, object: AnyMExpression) -> Self {
        self.object = Some(object);
        self
    }
    pub fn build(self) -> MComputedMemberAssignment {
        MComputedMemberAssignment::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT,
            [
                self.object
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.member.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn m_computed_member_expression(
    l_brack_token: SyntaxToken,
    member: AnyMExpression,
    r_brack_token: SyntaxToken,
) -> MComputedMemberExpressionBuilder {
    MComputedMemberExpressionBuilder {
        l_brack_token,
        member,
        r_brack_token,
        object: None,
    }
}
pub struct MComputedMemberExpressionBuilder {
    l_brack_token: SyntaxToken,
    member: AnyMExpression,
    r_brack_token: SyntaxToken,
    object: Option<AnyMExpression>,
}
impl MComputedMemberExpressionBuilder {
    pub fn with_object(mut self, object: AnyMExpression) -> Self {
        self.object = Some(object);
        self
    }
    pub fn build(self) -> MComputedMemberExpression {
        MComputedMemberExpression::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION,
            [
                self.object
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.member.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn m_computed_member_name(
    l_brack_token: SyntaxToken,
    expression: AnyMExpression,
    r_brack_token: SyntaxToken,
) -> MComputedMemberName {
    MComputedMemberName::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_COMPUTED_MEMBER_NAME,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn m_conditional_expression(
    test: AnyMExpression,
    question_mark_token: SyntaxToken,
    consequent: AnyMExpression,
    colon_token: SyntaxToken,
    alternate: AnyMExpression,
) -> MConditionalExpression {
    MConditionalExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CONDITIONAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(question_mark_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(alternate.into_syntax())),
        ],
    ))
}
pub fn m_constant_expression(
    token: SyntaxToken,
    constant: AnyMStringLiteralExpression,
) -> MConstantExpression {
    MConstantExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CONSTANT_EXPRESSION,
        [
            Some(SyntaxElement::Token(token)),
            Some(SyntaxElement::Node(constant.into_syntax())),
        ],
    ))
}
pub fn m_constructor_class_member(
    annotation: MAnnotationGroupList,
    name: MLiteralMemberName,
    parameters: MConstructorParameters,
    body: MFunctionBody,
) -> MConstructorClassMemberBuilder {
    MConstructorClassMemberBuilder {
        annotation,
        name,
        parameters,
        body,
        doc_string: None,
    }
}
pub struct MConstructorClassMemberBuilder {
    annotation: MAnnotationGroupList,
    name: MLiteralMemberName,
    parameters: MConstructorParameters,
    body: MFunctionBody,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MConstructorClassMemberBuilder {
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MConstructorClassMember {
        MConstructorClassMember::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_CONSTRUCTOR_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_constructor_parameters(
    l_paren_token: SyntaxToken,
    parameters: MConstructorParameterList,
    r_paren_token: SyntaxToken,
) -> MConstructorParameters {
    MConstructorParameters::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CONSTRUCTOR_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_continue_statement(continue_token: SyntaxToken) -> MContinueStatementBuilder {
    MContinueStatementBuilder {
        continue_token,
        semicolon_token: None,
    }
}
pub struct MContinueStatementBuilder {
    continue_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl MContinueStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MContinueStatement {
        MContinueStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_CONTINUE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.continue_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_date_literal_expression(value_token: SyntaxToken) -> MDateLiteralExpression {
    MDateLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_DATE_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_debug_statement(debug_token: SyntaxToken) -> MDebugStatementBuilder {
    MDebugStatementBuilder {
        debug_token,
        semicolon_token: None,
    }
}
pub struct MDebugStatementBuilder {
    debug_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl MDebugStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MDebugStatement {
        MDebugStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_DEBUG_STATEMENT,
            [
                Some(SyntaxElement::Token(self.debug_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_default_clause(else_token: SyntaxToken, consequent: MStatementList) -> MDefaultClause {
    MDefaultClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_DEFAULT_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(consequent.into_syntax())),
        ],
    ))
}
pub fn m_directive(version_token: SyntaxToken, value_token: SyntaxToken) -> MDirective {
    MDirective::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_DIRECTIVE,
        [
            Some(SyntaxElement::Token(version_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn m_extended_binding(
    object: AnyMFunctionBinding,
    operator_token: SyntaxToken,
    member: MName,
) -> MExtendedBinding {
    MExtendedBinding::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_EXTENDED_BINDING,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub struct MDirectiveBuilder {
    value_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl MDirectiveBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MDirective {
        MDirective::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.value_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_else_clause(else_token: SyntaxToken, alternate: AnyMStatement) -> MElseClause {
    MElseClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(alternate.into_syntax())),
        ],
    ))
}
pub fn m_empty_statement(semicolon_token: SyntaxToken) -> MEmptyStatement {
    MEmptyStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_EMPTY_STATEMENT,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn m_expression_snipped(
    expression: AnyMExpression,
    eof_token: SyntaxToken,
) -> MExpressionSnipped {
    MExpressionSnipped::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_EXPRESSION_SNIPPED,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn m_expression_statement(expression: AnyMExpression) -> MExpressionStatementBuilder {
    MExpressionStatementBuilder {
        expression,
        semicolon_token: None,
    }
}
pub struct MExpressionStatementBuilder {
    expression: AnyMExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl MExpressionStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MExpressionStatement {
        MExpressionStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_EXPRESSION_STATEMENT,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_extends_clause(extends_token: SyntaxToken, super_class: AnyMExpression) -> MExtendsClause {
    MExtendsClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_EXTENDS_CLAUSE,
        [
            Some(SyntaxElement::Token(extends_token)),
            Some(SyntaxElement::Node(super_class.into_syntax())),
        ],
    ))
}
pub fn m_finally_clause(finally_token: SyntaxToken, body: MBlockStatement) -> MFinallyClause {
    MFinallyClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FINALLY_CLAUSE,
        [
            Some(SyntaxElement::Token(finally_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_for_all_in_statement(
    forall_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    initializer: AnyMForInInitializer,
    in_token: SyntaxToken,
    expression: AnyMExpression,
    r_paren_token: SyntaxToken,
    body: AnyMStatement,
) -> MForAllInStatement {
    MForAllInStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FOR_ALL_IN_STATEMENT,
        [
            Some(SyntaxElement::Token(forall_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(initializer.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_for_all_statement(
    forall_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    iter: MForIteratorFactory,
    r_paren_token: SyntaxToken,
    body: AnyMStatement,
) -> MForAllStatement {
    MForAllStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FOR_ALL_STATEMENT,
        [
            Some(SyntaxElement::Token(forall_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(iter.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_for_iterator_factory(
    name: MIdentifierExpression,
    l_paren_token: SyntaxToken,
    expression: AnyMExpression,
    comma_token: SyntaxToken,
    initializer: MVariableDeclarator,
    r_paren_token: SyntaxToken,
) -> MForIteratorFactory {
    MForIteratorFactory::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FOR_ITERATOR_FACTORY,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(initializer.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_for_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    first_semi_token: SyntaxToken,
    second_semi_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: AnyMStatement,
) -> MForStatementBuilder {
    MForStatementBuilder {
        for_token,
        l_paren_token,
        first_semi_token,
        second_semi_token,
        r_paren_token,
        body,
        initializer: None,
        test: None,
        update: None,
    }
}
pub struct MForStatementBuilder {
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    first_semi_token: SyntaxToken,
    second_semi_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: AnyMStatement,
    initializer: Option<AnyMForInitializer>,
    test: Option<AnyMExpression>,
    update: Option<AnyMExpression>,
}
impl MForStatementBuilder {
    pub fn with_initializer(mut self, initializer: AnyMForInitializer) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn with_test(mut self, test: AnyMExpression) -> Self {
        self.test = Some(test);
        self
    }
    pub fn with_update(mut self, update: AnyMExpression) -> Self {
        self.update = Some(update);
        self
    }
    pub fn build(self) -> MForStatement {
        MForStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_FOR_STATEMENT,
            [
                Some(SyntaxElement::Token(self.for_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.first_semi_token)),
                self.test
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.second_semi_token)),
                self.update
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_for_variable_declaration(
    declarator: MVariableDeclarator,
) -> MForVariableDeclarationBuilder {
    MForVariableDeclarationBuilder {
        declarator,
        var_token: None,
    }
}
pub struct MForVariableDeclarationBuilder {
    declarator: MVariableDeclarator,
    var_token: Option<SyntaxToken>,
}
impl MForVariableDeclarationBuilder {
    pub fn with_var_token(mut self, var_token: SyntaxToken) -> Self {
        self.var_token = Some(var_token);
        self
    }
    pub fn build(self) -> MForVariableDeclaration {
        MForVariableDeclaration::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_FOR_VARIABLE_DECLARATION,
            [
                self.var_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.declarator.into_syntax())),
            ],
        ))
    }
}
pub fn m_formal_parameter(binding: AnyMBinding) -> MFormalParameterBuilder {
    MFormalParameterBuilder {
        binding,
        initializer: None,
    }
}
pub struct MFormalParameterBuilder {
    binding: AnyMBinding,
    initializer: Option<MInitializerClause>,
}
impl MFormalParameterBuilder {
    pub fn with_initializer(mut self, initializer: MInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> MFormalParameter {
        MFormalParameter::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_FORMAL_PARAMETER,
            [
                Some(SyntaxElement::Node(self.binding.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn m_function_body(
    l_curly_token: SyntaxToken,
    directives: MDirectiveList,
    statements: MStatementList,
    r_curly_token: SyntaxToken,
) -> MFunctionBody {
    MFunctionBody::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FUNCTION_BODY,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(directives.into_syntax())),
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn m_function_declaration(
    annotation: MAnnotationGroupList,
    function_token: SyntaxToken,
    id: AnyMFunctionBinding,
    parameters: MParameters,
    body: MFunctionBody,
) -> MFunctionDeclarationBuilder {
    MFunctionDeclarationBuilder {
        annotation,
        function_token,
        id,
        parameters,
        body,
        doc_string: None,
    }
}
pub struct MFunctionDeclarationBuilder {
    annotation: MAnnotationGroupList,
    function_token: SyntaxToken,
    id: AnyMFunctionBinding,
    parameters: MParameters,
    body: MFunctionBody,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MFunctionDeclarationBuilder {
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MFunctionDeclaration {
        MFunctionDeclaration::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_FUNCTION_DECLARATION,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Token(self.function_token)),
                Some(SyntaxElement::Node(self.id.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_function_expression(
    function_token: SyntaxToken,
    parameters: MParameters,
    body: MFunctionBody,
) -> MFunctionExpression {
    MFunctionExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_FUNCTION_EXPRESSION,
        [
            Some(SyntaxElement::Token(function_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_getter_class_member(
    annotation: MAnnotationGroupList,
    get_token: SyntaxToken,
    name: AnyMClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: MFunctionBody,
) -> MGetterClassMemberBuilder {
    MGetterClassMemberBuilder {
        annotation,
        get_token,
        name,
        l_paren_token,
        r_paren_token,
        body,
        doc_string: None,
    }
}
pub struct MGetterClassMemberBuilder {
    annotation: MAnnotationGroupList,
    get_token: SyntaxToken,
    name: AnyMClassMemberName,
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    body: MFunctionBody,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MGetterClassMemberBuilder {
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MGetterClassMember {
        MGetterClassMember::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_GETTER_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Token(self.get_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_hash_map_expression(
    at_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    members: MHashMapMemberList,
    r_paren_token: SyntaxToken,
) -> MHashMapExpression {
    MHashMapExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_HASH_MAP_EXPRESSION,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_hash_set_expression(
    set_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    members: MHashSetElementList,
    r_paren_token: SyntaxToken,
) -> MHashSetExpression {
    MHashSetExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_HASH_SET_EXPRESSION,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_identifier_assignment(name_token: SyntaxToken) -> MIdentifierAssignment {
    MIdentifierAssignment::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_IDENTIFIER_ASSIGNMENT,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn m_identifier_binding(name_token: SyntaxToken) -> MIdentifierBinding {
    MIdentifierBinding::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_IDENTIFIER_BINDING,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn m_identifier_expression(name: MReferenceIdentifier) -> MIdentifierExpression {
    MIdentifierExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_IDENTIFIER_EXPRESSION,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn m_if_statement(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyMExpression,
    r_paren_token: SyntaxToken,
    consequent: AnyMStatement,
) -> MIfStatementBuilder {
    MIfStatementBuilder {
        if_token,
        l_paren_token,
        test,
        r_paren_token,
        consequent,
        else_clause: None,
    }
}
pub struct MIfStatementBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyMExpression,
    r_paren_token: SyntaxToken,
    consequent: AnyMStatement,
    else_clause: Option<MElseClause>,
}
impl MIfStatementBuilder {
    pub fn with_else_clause(mut self, else_clause: MElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> MIfStatement {
        MIfStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_IF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.test.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.consequent.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn m_in_expression(
    property: AnyMExpression,
    in_token: SyntaxToken,
    object: AnyMExpression,
) -> MInExpression {
    MInExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_IN_EXPRESSION,
        [
            Some(SyntaxElement::Node(property.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(object.into_syntax())),
        ],
    ))
}
pub fn m_initializer_clause(
    eq_token: SyntaxToken,
    expression: AnyMExpression,
) -> MInitializerClause {
    MInitializerClause::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_INITIALIZER_CLAUSE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn m_literal_member_name(value_token: SyntaxToken) -> MLiteralMemberName {
    MLiteralMemberName::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_LITERAL_MEMBER_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_logical_expression(
    left: AnyMExpression,
    operator_token_token: SyntaxToken,
    right: AnyMExpression,
) -> MLogicalExpression {
    MLogicalExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_LOGICAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn m_long_string_literal_expression(value_token: SyntaxToken) -> MLongStringLiteralExpression {
    MLongStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_LONG_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_method_class_member(
    annotation: MAnnotationGroupList,
    name: AnyMClassMemberName,
    parameters: MParameters,
    body: MFunctionBody,
) -> MMethodClassMemberBuilder {
    MMethodClassMemberBuilder {
        annotation,
        name,
        parameters,
        body,
        doc_string: None,
    }
}
pub struct MMethodClassMemberBuilder {
    annotation: MAnnotationGroupList,
    name: AnyMClassMemberName,
    parameters: MParameters,
    body: MFunctionBody,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MMethodClassMemberBuilder {
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MMethodClassMember {
        MMethodClassMember::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_METHOD_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_module(
    directives: MDirectiveList,
    items: MModuleItemList,
    eof_token: SyntaxToken,
) -> MModule {
    MModule::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_MODULE,
        [
            Some(SyntaxElement::Node(directives.into_syntax())),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn m_name(value_token: SyntaxToken) -> MName {
    MName::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_new_expression(new_token: SyntaxToken, callee: AnyMExpression) -> MNewExpressionBuilder {
    MNewExpressionBuilder {
        new_token,
        callee,
        arguments: None,
    }
}
pub struct MNewExpressionBuilder {
    new_token: SyntaxToken,
    callee: AnyMExpression,
    arguments: Option<MCallArguments>,
}
impl MNewExpressionBuilder {
    pub fn with_arguments(mut self, arguments: MCallArguments) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn build(self) -> MNewExpression {
        MNewExpression::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_NEW_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.new_token)),
                Some(SyntaxElement::Node(self.callee.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn m_null_literal_expression(value_token: SyntaxToken) -> MNullLiteralExpression {
    MNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_number_literal_expression(value_token: SyntaxToken) -> MNumberLiteralExpression {
    MNumberLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_NUMBER_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_object_expression(
    at_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    members: MObjectMemberList,
    r_curly_token: SyntaxToken,
) -> MObjectExpression {
    MObjectExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_OBJECT_EXPRESSION,
        [
            Some(SyntaxElement::Token(at_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn m_parameters(
    l_paren_token: SyntaxToken,
    items: MParameterList,
    r_paren_token: SyntaxToken,
) -> MParameters {
    MParameters::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_parenthesized_assignment(
    l_paren_token: SyntaxToken,
    assignment: AnyMAssignment,
    r_paren_token: SyntaxToken,
) -> MParenthesizedAssignment {
    MParenthesizedAssignment::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PARENTHESIZED_ASSIGNMENT,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(assignment.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: AnyMExpression,
    r_paren_token: SyntaxToken,
) -> MParenthesizedExpression {
    MParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn m_post_update_expression(
    operand: AnyMAssignment,
    operator_token_token: SyntaxToken,
) -> MPostUpdateExpression {
    MPostUpdateExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_POST_UPDATE_EXPRESSION,
        [
            Some(SyntaxElement::Node(operand.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
        ],
    ))
}
pub fn m_pre_update_expression(
    operator_token_token: SyntaxToken,
    operand: AnyMAssignment,
) -> MPreUpdateExpression {
    MPreUpdateExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PRE_UPDATE_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(operand.into_syntax())),
        ],
    ))
}
pub fn m_property_object_member(
    name: AnyMObjectMemberName,
    colon_token: SyntaxToken,
    value: AnyMExpression,
) -> MPropertyObjectMember {
    MPropertyObjectMember::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PROPERTY_OBJECT_MEMBER,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn m_reference_identifier(value_token: SyntaxToken) -> MReferenceIdentifier {
    MReferenceIdentifier::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REFERENCE_IDENTIFIER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_report(
    name: MReportName,
    init: MReportInitList,
    sections: MReportSectionList,
) -> MReportBuilder {
    MReportBuilder {
        name,
        init,
        sections,
        default: None,
    }
}
pub struct MReportBuilder {
    name: MReportName,
    init: MReportInitList,
    sections: MReportSectionList,
    default: Option<MBlockStatement>,
}
impl MReportBuilder {
    pub fn with_default(mut self, default: MBlockStatement) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> MReport {
        MReport::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_REPORT,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.init.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.sections.into_syntax())),
            ],
        ))
    }
}
pub fn m_report_file(reports: MReportList, eof_token: SyntaxToken) -> MReportFile {
    MReportFile::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_FILE,
        [
            Some(SyntaxElement::Node(reports.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn m_report_name(ff2_token: SyntaxToken, m_name: MName) -> MReportName {
    MReportName::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_NAME,
        [
            Some(SyntaxElement::Token(ff2_token)),
            Some(SyntaxElement::Node(m_name.into_syntax())),
        ],
    ))
}
pub fn m_report_section(name: MReportSectionName, body: MBlockStatement) -> MReportSection {
    MReportSection::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_SECTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_report_section_name(ff_token: SyntaxToken, m_name: MName) -> MReportSectionName {
    MReportSectionName::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_SECTION_NAME,
        [
            Some(SyntaxElement::Token(ff_token)),
            Some(SyntaxElement::Node(m_name.into_syntax())),
        ],
    ))
}
pub fn m_rest_parameter(dotdotdot_token: SyntaxToken) -> MRestParameterBuilder {
    MRestParameterBuilder {
        dotdotdot_token,
        binding: None,
    }
}
pub struct MRestParameterBuilder {
    dotdotdot_token: SyntaxToken,
    binding: Option<AnyMBinding>,
}
impl MRestParameterBuilder {
    pub fn with_binding(mut self, binding: AnyMBinding) -> Self {
        self.binding = Some(binding);
        self
    }
    pub fn build(self) -> MRestParameter {
        MRestParameter::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_REST_PARAMETER,
            [
                Some(SyntaxElement::Token(self.dotdotdot_token)),
                self.binding
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn m_return_statement(return_token: SyntaxToken) -> MReturnStatementBuilder {
    MReturnStatementBuilder {
        return_token,
        argument: None,
        semicolon_token: None,
    }
}
pub struct MReturnStatementBuilder {
    return_token: SyntaxToken,
    argument: Option<AnyMExpression>,
    semicolon_token: Option<SyntaxToken>,
}
impl MReturnStatementBuilder {
    pub fn with_argument(mut self, argument: AnyMExpression) -> Self {
        self.argument = Some(argument);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MReturnStatement {
        MReturnStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_RETURN_STATEMENT,
            [
                Some(SyntaxElement::Token(self.return_token)),
                self.argument
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_script(statements: MStatementList, eof_token: SyntaxToken) -> MScript {
    MScript::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SCRIPT,
        [
            Some(SyntaxElement::Node(statements.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn m_sequence_expression(
    left: AnyMExpression,
    comma_token: SyntaxToken,
    right: AnyMExpression,
) -> MSequenceExpression {
    MSequenceExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SEQUENCE_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn m_setter_class_member(
    annotation: MAnnotationGroupList,
    set_token: SyntaxToken,
    name: AnyMClassMemberName,
    l_paren_token: SyntaxToken,
    parameter: AnyMFormalParameter,
    r_paren_token: SyntaxToken,
    body: MFunctionBody,
) -> MSetterClassMemberBuilder {
    MSetterClassMemberBuilder {
        annotation,
        set_token,
        name,
        l_paren_token,
        parameter,
        r_paren_token,
        body,
        comma_token: None,
        doc_string: None,
    }
}
pub struct MSetterClassMemberBuilder {
    annotation: MAnnotationGroupList,
    set_token: SyntaxToken,
    name: AnyMClassMemberName,
    l_paren_token: SyntaxToken,
    parameter: AnyMFormalParameter,
    r_paren_token: SyntaxToken,
    body: MFunctionBody,
    comma_token: Option<SyntaxToken>,
    doc_string: Option<AnyMStringLiteralExpression>,
}
impl MSetterClassMemberBuilder {
    pub fn with_comma_token(mut self, comma_token: SyntaxToken) -> Self {
        self.comma_token = Some(comma_token);
        self
    }
    pub fn with_doc_string(mut self, doc_string: AnyMStringLiteralExpression) -> Self {
        self.doc_string = Some(doc_string);
        self
    }
    pub fn build(self) -> MSetterClassMember {
        MSetterClassMember::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_SETTER_CLASS_MEMBER,
            [
                Some(SyntaxElement::Node(self.annotation.into_syntax())),
                Some(SyntaxElement::Token(self.set_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.parameter.into_syntax())),
                self.comma_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.doc_string
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
            ],
        ))
    }
}
pub fn m_shorthand_property_object_member(
    name: MReferenceIdentifier,
) -> MShorthandPropertyObjectMember {
    MShorthandPropertyObjectMember::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SHORTHAND_PROPERTY_OBJECT_MEMBER,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn m_spread(dotdotdot_token: SyntaxToken, argument: AnyMExpression) -> MSpread {
    MSpread::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SPREAD,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn m_static_member_assignment(
    object: AnyMExpression,
    dot_token: SyntaxToken,
    member: MName,
) -> MStaticMemberAssignment {
    MStaticMemberAssignment::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn m_static_member_expression(
    object: AnyMExpression,
    operator_token: SyntaxToken,
    member: MName,
) -> MExtendedBinding {
    MExtendedBinding::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_STATIC_MEMBER_EXPRESSION,
        [
            Some(SyntaxElement::Node(object.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(member.into_syntax())),
        ],
    ))
}
pub fn m_string_literal_expression(value_token: SyntaxToken) -> MStringLiteralExpression {
    MStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_super_expression(super_token: SyntaxToken) -> MSuperExpression {
    MSuperExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SUPER_EXPRESSION,
        [Some(SyntaxElement::Token(super_token))],
    ))
}
pub fn m_switch_statement(
    switch_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    discriminant: AnyMExpression,
    r_paren_token: SyntaxToken,
    l_curly_token: SyntaxToken,
    cases: MSwitchCaseList,
    r_curly_token: SyntaxToken,
) -> MSwitchStatement {
    MSwitchStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SWITCH_STATEMENT,
        [
            Some(SyntaxElement::Token(switch_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(discriminant.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(cases.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn m_this_expression(this_token: SyntaxToken) -> MThisExpression {
    MThisExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_THIS_EXPRESSION,
        [Some(SyntaxElement::Token(this_token))],
    ))
}
pub fn m_throw_statement(
    throw_token: SyntaxToken,
    argument: AnyMExpression,
) -> MThrowStatementBuilder {
    MThrowStatementBuilder {
        throw_token,
        argument,
        semicolon_token: None,
    }
}
pub struct MThrowStatementBuilder {
    throw_token: SyntaxToken,
    argument: AnyMExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl MThrowStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MThrowStatement {
        MThrowStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_THROW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.throw_token)),
                Some(SyntaxElement::Node(self.argument.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_time_literal_expression(value_token: SyntaxToken) -> MTimeLiteralExpression {
    MTimeLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_TIME_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn m_try_finally_statement(
    try_token: SyntaxToken,
    body: MBlockStatement,
    finally_clause: MFinallyClause,
) -> MTryFinallyStatementBuilder {
    MTryFinallyStatementBuilder {
        try_token,
        body,
        finally_clause,
        catch_clause: None,
    }
}
pub struct MTryFinallyStatementBuilder {
    try_token: SyntaxToken,
    body: MBlockStatement,
    finally_clause: MFinallyClause,
    catch_clause: Option<MCatchClause>,
}
impl MTryFinallyStatementBuilder {
    pub fn with_catch_clause(mut self, catch_clause: MCatchClause) -> Self {
        self.catch_clause = Some(catch_clause);
        self
    }
    pub fn build(self) -> MTryFinallyStatement {
        MTryFinallyStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_TRY_FINALLY_STATEMENT,
            [
                Some(SyntaxElement::Token(self.try_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                self.catch_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.finally_clause.into_syntax())),
            ],
        ))
    }
}
pub fn m_try_statement(
    try_token: SyntaxToken,
    body: MBlockStatement,
    catch_clause: MCatchClause,
) -> MTryStatement {
    MTryStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_TRY_STATEMENT,
        [
            Some(SyntaxElement::Token(try_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Node(catch_clause.into_syntax())),
        ],
    ))
}
pub fn m_unary_expression(
    operator_token_token: SyntaxToken,
    argument: AnyMExpression,
) -> MUnaryExpression {
    MUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn m_variable_declaration(
    kind_token: SyntaxToken,
    declarators: MVariableDeclaratorList,
) -> MVariableDeclaration {
    MVariableDeclaration::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_VARIABLE_DECLARATION,
        [
            Some(SyntaxElement::Token(kind_token)),
            Some(SyntaxElement::Node(declarators.into_syntax())),
        ],
    ))
}
pub fn m_variable_declaration_clause(
    declaration: MVariableDeclaration,
) -> MVariableDeclarationClauseBuilder {
    MVariableDeclarationClauseBuilder {
        declaration,
        semicolon_token: None,
    }
}
pub struct MVariableDeclarationClauseBuilder {
    declaration: MVariableDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl MVariableDeclarationClauseBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MVariableDeclarationClause {
        MVariableDeclarationClause::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_VARIABLE_DECLARATION_CLAUSE,
            [
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_variable_declarator(id: AnyMBinding) -> MVariableDeclaratorBuilder {
    MVariableDeclaratorBuilder {
        id,
        initializer: None,
    }
}
pub struct MVariableDeclaratorBuilder {
    id: AnyMBinding,
    initializer: Option<MInitializerClause>,
}
impl MVariableDeclaratorBuilder {
    pub fn with_initializer(mut self, initializer: MInitializerClause) -> Self {
        self.initializer = Some(initializer);
        self
    }
    pub fn build(self) -> MVariableDeclarator {
        MVariableDeclarator::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_VARIABLE_DECLARATOR,
            [
                Some(SyntaxElement::Node(self.id.into_syntax())),
                self.initializer
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn m_variable_statement(declaration: MVariableDeclaration) -> MVariableStatementBuilder {
    MVariableStatementBuilder {
        declaration,
        semicolon_token: None,
    }
}
pub struct MVariableStatementBuilder {
    declaration: MVariableDeclaration,
    semicolon_token: Option<SyntaxToken>,
}
impl MVariableStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> MVariableStatement {
        MVariableStatement::unwrap_cast(SyntaxNode::new_detached(
            MSyntaxKind::M_VARIABLE_STATEMENT,
            [
                Some(SyntaxElement::Node(self.declaration.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn m_while_statement(
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    test: AnyMExpression,
    r_paren_token: SyntaxToken,
    body: AnyMStatement,
) -> MWhileStatement {
    MWhileStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_WHILE_STATEMENT,
        [
            Some(SyntaxElement::Token(while_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(test.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn m_annotation_attribute_list<I, S>(items: I, separators: S) -> MAnnotationAttributeList
where
    I: IntoIterator<Item = MAnnotationAttribute>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MAnnotationAttributeList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_ATTRIBUTE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_annotation_group_list<I>(items: I) -> MAnnotationGroupList
where
    I: IntoIterator<Item = MAnnotationGroup>,
    I::IntoIter: ExactSizeIterator,
{
    MAnnotationGroupList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_GROUP_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_annotation_list<I, S>(items: I, separators: S) -> MAnnotationList
where
    I: IntoIterator<Item = AnyMAnnotationElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MAnnotationList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ANNOTATION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_array_element_list<I, S>(items: I, separators: S) -> MArrayElementList
where
    I: IntoIterator<Item = AnyMArrayElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_ARRAY_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_call_argument_list<I, S>(items: I, separators: S) -> MCallArgumentList
where
    I: IntoIterator<Item = AnyMCallArgument>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MCallArgumentList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CALL_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_class_member_list<I>(items: I) -> MClassMemberList
where
    I: IntoIterator<Item = AnyMClassMember>,
    I::IntoIter: ExactSizeIterator,
{
    MClassMemberList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CLASS_MEMBER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_constructor_parameter_list<I, S>(items: I, separators: S) -> MConstructorParameterList
where
    I: IntoIterator<Item = AnyMConstructorParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MConstructorParameterList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_CONSTRUCTOR_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_directive_list<I>(items: I) -> MDirectiveList
where
    I: IntoIterator<Item = MDirective>,
    I::IntoIter: ExactSizeIterator,
{
    MDirectiveList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_DIRECTIVE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_hash_map_member_list<I, S>(items: I, separators: S) -> MHashMapMemberList
where
    I: IntoIterator<Item = AnyMObjectMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MHashMapMemberList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_HASH_MAP_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_hash_set_element_list<I, S>(items: I, separators: S) -> MHashSetElementList
where
    I: IntoIterator<Item = AnyMArrayElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MHashSetElementList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_HASH_SET_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_module_item_list<I>(items: I) -> MModuleItemList
where
    I: IntoIterator<Item = AnyMStatement>,
    I::IntoIter: ExactSizeIterator,
{
    MModuleItemList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_MODULE_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_object_member_list<I, S>(items: I, separators: S) -> MObjectMemberList
where
    I: IntoIterator<Item = AnyMObjectMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MObjectMemberList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_OBJECT_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_parameter_list<I, S>(items: I, separators: S) -> MParameterList
where
    I: IntoIterator<Item = AnyMParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MParameterList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_report_init_list<I>(items: I) -> MReportInitList
where
    I: IntoIterator<Item = MExpressionStatement>,
    I::IntoIter: ExactSizeIterator,
{
    MReportInitList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_INIT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_report_list<I>(items: I) -> MReportList
where
    I: IntoIterator<Item = MReport>,
    I::IntoIter: ExactSizeIterator,
{
    MReportList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_report_section_list<I>(items: I) -> MReportSectionList
where
    I: IntoIterator<Item = MReportSection>,
    I::IntoIter: ExactSizeIterator,
{
    MReportSectionList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_REPORT_SECTION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_statement_list<I>(items: I) -> MStatementList
where
    I: IntoIterator<Item = AnyMStatement>,
    I::IntoIter: ExactSizeIterator,
{
    MStatementList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_switch_case_list<I>(items: I) -> MSwitchCaseList
where
    I: IntoIterator<Item = AnyMSwitchClause>,
    I::IntoIter: ExactSizeIterator,
{
    MSwitchCaseList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_SWITCH_CASE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn m_variable_declarator_list<I, S>(items: I, separators: S) -> MVariableDeclaratorList
where
    I: IntoIterator<Item = MVariableDeclarator>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MVariableDeclaratorList::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_VARIABLE_DECLARATOR_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn m_bogus<I>(slots: I) -> MBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogus::unwrap_cast(SyntaxNode::new_detached(MSyntaxKind::M_BOGUS, slots))
}
pub fn m_bogus_assignment<I>(slots: I) -> MBogusAssignment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusAssignment::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOGUS_ASSIGNMENT,
        slots,
    ))
}
pub fn m_bogus_binding<I>(slots: I) -> MBogusBinding
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusBinding::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOGUS_BINDING,
        slots,
    ))
}
pub fn m_bogus_expression<I>(slots: I) -> MBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn m_bogus_member<I>(slots: I) -> MBogusMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusMember::unwrap_cast(SyntaxNode::new_detached(MSyntaxKind::M_BOGUS_MEMBER, slots))
}
pub fn m_bogus_parameter<I>(slots: I) -> MBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn m_bogus_statement<I>(slots: I) -> MBogusStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MBogusStatement::unwrap_cast(SyntaxNode::new_detached(
        MSyntaxKind::M_BOGUS_STATEMENT,
        slots,
    ))
}
