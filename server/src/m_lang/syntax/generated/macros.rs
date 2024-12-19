//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match crate::m_lang::syntax::MSyntaxNode::kind(&node) {
                crate::m_lang::syntax::MSyntaxKind::M_ARRAY_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MArrayExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_ARRAY_HOLE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MArrayHole::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MAssignmentExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_ASSIGNMENT_PATTERN => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MAssignmentPattern::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BIGINT_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MBigintLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BINARY_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBinaryExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BINDING_PATTERN => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBindingPattern::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BLOCK_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBlockStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MBooleanLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BREAK_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBreakStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CALL_ARGUMENTS => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCallArguments::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CALL_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCallExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CASE_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCaseClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CATCH_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCatchClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CATCH_DECLARATION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCatchDeclaration::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CLASS => {
                    let $pattern = unsafe { crate::m_lang::syntax::MClass::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CLASS_DECLARATION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MClassDeclaration::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MComputedMemberAssignment::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MComputedMemberExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_COMPUTED_MEMBER_NAME => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MComputedMemberName::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CONDITIONAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MConditionalExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MConstructorClassMember::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CONSTRUCTOR_PARAMETERS => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MConstructorParameters::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CONTINUE_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MContinueStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_DEBUG_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MDebugStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_DEFAULT_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MDefaultClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_DIRECTIVE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MDirective::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_ELSE_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MElseClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_EMPTY_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MEmptyClassMember::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_EMPTY_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MEmptyStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_EXPRESSION_SNIPPED => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MExpressionSnipped::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_EXPRESSION_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MExpressionStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_EXTENDS_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MExtendsClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FINALLY_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MFinallyClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FOR_ALL_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MForAllStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FOR_IN_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MForInStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FOR_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MForStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FOR_VARIABLE_DECLARATION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MForVariableDeclaration::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FORMAL_PARAMETER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MFormalParameter::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FUNCTION_BODY => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MFunctionBody::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MFunctionExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_GETTER_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MGetterClassMember::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IDENTIFIER_ASSIGNMENT => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MIdentifierAssignment::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IDENTIFIER_BINDING => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MIdentifierBinding::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IDENTIFIER_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MIdentifierExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IF_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MIfStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IN_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MInExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_IN_PROPERTY => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MInProperty::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MInitializerClause::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_LITERAL_MEMBER_NAME => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MLiteralMemberName::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_LOGICAL_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MLogicalExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_LONG_STRING_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MLongStringLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MMethodClassMember::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_NAME => {
                    let $pattern = unsafe { crate::m_lang::syntax::MName::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_NEW_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MNewExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MNullLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MNumberLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_OBJECT_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MObjectExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PARAMETERS => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MParameters::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PARENTHESIZED_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MParenthesizedExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_POST_UPDATE_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MPostUpdateExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PRE_UPDATE_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PRIVATE_CLASS_MEMBER_NAME => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MPrivateClassMemberName::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MPropertyObjectMember::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_REFERENCE_IDENTIFIER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_REST_PARAMETER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MRestParameter::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_RETURN_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MReturnStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SCRIPT => {
                    let $pattern = unsafe { crate::m_lang::syntax::MScript::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SEQUENCE_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MSequenceExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SETTER_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MSetterClassMember::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MShorthandPropertyObjectMember::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SPREAD => {
                    let $pattern = unsafe { crate::m_lang::syntax::MSpread::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MStaticMemberAssignment::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MStaticMemberExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_STRING_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MStringLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SUPER_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MSuperExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SWITCH_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MSwitchStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_TEMPLATE_ELEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MTemplateElement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_THIS_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MThisExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_THROW_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MThrowStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_TRY_FINALLY_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_TRY_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MTryStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_UNARY_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MUnaryExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MVariableDeclaration::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MVariableDeclarationClause::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_VARIABLE_DECLARATOR => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MVariableDeclarator::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_VARIABLE_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MVariableStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_WHILE_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MWhileStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS => {
                    let $pattern = unsafe { crate::m_lang::syntax::MBogus::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_ASSIGNMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusAssignment::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_BINDING => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusBinding::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_EXPRESSION => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusExpression::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_MEMBER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusMember::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_PARAMETER => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusParameter::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_BOGUS_STATEMENT => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MBogusStatement::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_ARRAY_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MArrayElementList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CALL_ARGUMENT_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MCallArgumentList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CLASS_MEMBER_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MClassMemberList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MConstructorParameterList::new_unchecked(node)
                    };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_DIRECTIVE_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MDirectiveList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_OBJECT_MEMBER_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MObjectMemberList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MParameterList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_STATEMENT_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MStatementList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_SWITCH_CASE_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MSwitchCaseList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_TEMPLATE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { crate::m_lang::syntax::MTemplateElementList::new_unchecked(node) };
                    $body
                }
                crate::m_lang::syntax::MSyntaxKind::M_VARIABLE_DECLARATOR_LIST => {
                    let $pattern = unsafe {
                        crate::m_lang::syntax::MVariableDeclaratorList::new_unchecked(node)
                    };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
