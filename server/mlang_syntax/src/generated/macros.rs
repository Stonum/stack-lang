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
            node => match $crate::MSyntaxNode::kind(&node) {
                $crate::MSyntaxKind::M_ANNOTATION_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::MAnnotationAttribute::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_BINDING => {
                    let $pattern = unsafe { $crate::MAnnotationBinding::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_ELEMENT => {
                    let $pattern = unsafe { $crate::MAnnotationElement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_GROUP => {
                    let $pattern = unsafe { $crate::MAnnotationGroup::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::MArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ARRAY_HOLE => {
                    let $pattern = unsafe { $crate::MArrayHole::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
                    let $pattern = unsafe { $crate::MAssignmentExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::MBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BLOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::MBlockStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::MBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BREAK_STATEMENT => {
                    let $pattern = unsafe { $crate::MBreakStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CALL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::MCallArguments::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CASE_CLAUSE => {
                    let $pattern = unsafe { $crate::MCaseClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CATCH_CLAUSE => {
                    let $pattern = unsafe { $crate::MCatchClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CATCH_DECLARATION => {
                    let $pattern = unsafe { $crate::MCatchDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CLASS_DECLARATION => {
                    let $pattern = unsafe { $crate::MClassDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CLASS_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::MClassMemberName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::MComputedMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::MComputedMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_COMPUTED_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::MComputedMemberName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONDITIONAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MConditionalExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONSTANT_EXPRESSION => {
                    let $pattern = unsafe { $crate::MConstantExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::MConstructorClassMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONSTRUCTOR_PARAMETERS => {
                    let $pattern = unsafe { $crate::MConstructorParameters::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONTINUE_STATEMENT => {
                    let $pattern = unsafe { $crate::MContinueStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_DATE_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MDateLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_DEBUG_STATEMENT => {
                    let $pattern = unsafe { $crate::MDebugStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { $crate::MDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_DIRECTIVE => {
                    let $pattern = unsafe { $crate::MDirective::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_EXTENDED_BINDING => {
                    let $pattern = unsafe { $crate::MExtendedBinding::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::MElseClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::MEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_EXPRESSION_SNIPPED => {
                    let $pattern = unsafe { $crate::MExpressionSnipped::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_EXPRESSION_STATEMENT => {
                    let $pattern = unsafe { $crate::MExpressionStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::MExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FINALLY_CLAUSE => {
                    let $pattern = unsafe { $crate::MFinallyClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FOR_ALL_STATEMENT => {
                    let $pattern = unsafe { $crate::MForAllStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FOR_ITERATOR_FACTORY => {
                    let $pattern = unsafe { $crate::MForIteratorFactory::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FOR_ALL_IN_STATEMENT => {
                    let $pattern = unsafe { $crate::MForAllInStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::MForStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FOR_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { $crate::MForVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FORMAL_PARAMETER => {
                    let $pattern = unsafe { $crate::MFormalParameter::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FUNCTION_BODY => {
                    let $pattern = unsafe { $crate::MFunctionBody::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FUNCTION_DECLARATION => {
                    let $pattern = unsafe { $crate::MFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_FUNCTION_EXPRESSION => {
                    let $pattern = unsafe { $crate::MFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_GETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::MGetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_HASH_MAP_EXPRESSION => {
                    let $pattern = unsafe { $crate::MHashMapExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_HASH_SET_EXPRESSION => {
                    let $pattern = unsafe { $crate::MHashSetExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_IDENTIFIER_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::MIdentifierAssignment::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { $crate::MIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_IDENTIFIER_EXPRESSION => {
                    let $pattern = unsafe { $crate::MIdentifierExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::MIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::MInExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_INITIALIZER_CLAUSE => {
                    let $pattern = unsafe { $crate::MInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_INSTANCEOF_EXPRESSION => {
                    let $pattern = unsafe { $crate::MInstanceofExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_LITERAL_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::MLiteralMemberName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_LONG_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::MLongStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::MMethodClassMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_MODULE => {
                    let $pattern = unsafe { $crate::MModule::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_NAME => {
                    let $pattern = unsafe { $crate::MName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_NEW_EXPRESSION => {
                    let $pattern = unsafe { $crate::MNewExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_OBJECT_EXPRESSION => {
                    let $pattern = unsafe { $crate::MObjectExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PARENTHESIZED_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::MParenthesizedAssignment::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PARAMETERS => {
                    let $pattern = unsafe { $crate::MParameters::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PARENTHESIZED_EXPRESSION => {
                    let $pattern = unsafe { $crate::MParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_POST_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::MPostUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PRE_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::MPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::MPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REFERENCE_IDENTIFIER => {
                    let $pattern = unsafe { $crate::MReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT => {
                    let $pattern = unsafe { $crate::MReport::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_FILE => {
                    let $pattern = unsafe { $crate::MReportFile::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_NAME => {
                    let $pattern = unsafe { $crate::MReportName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_SECTION => {
                    let $pattern = unsafe { $crate::MReportSection::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_SECTION_NAME => {
                    let $pattern = unsafe { $crate::MReportSectionName::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REST_PARAMETER => {
                    let $pattern = unsafe { $crate::MRestParameter::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_RETURN_STATEMENT => {
                    let $pattern = unsafe { $crate::MReturnStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SCRIPT => {
                    let $pattern = unsafe { $crate::MScript::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SEQUENCE_EXPRESSION => {
                    let $pattern = unsafe { $crate::MSequenceExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::MSetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::MShorthandPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SPREAD => {
                    let $pattern = unsafe { $crate::MSpread::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::MStaticMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                    let $pattern = unsafe { $crate::MStaticMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_STRING_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SUPER_EXPRESSION => {
                    let $pattern = unsafe { $crate::MSuperExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SWITCH_STATEMENT => {
                    let $pattern = unsafe { $crate::MSwitchStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_THIS_EXPRESSION => {
                    let $pattern = unsafe { $crate::MThisExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_THROW_STATEMENT => {
                    let $pattern = unsafe { $crate::MThrowStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_TIME_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::MTimeLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_TRY_FINALLY_STATEMENT => {
                    let $pattern = unsafe { $crate::MTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_TRY_STATEMENT => {
                    let $pattern = unsafe { $crate::MTryStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::MUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { $crate::MVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::MVariableDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_VARIABLE_DECLARATOR => {
                    let $pattern = unsafe { $crate::MVariableDeclarator::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_VARIABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::MVariableStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::MWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS => {
                    let $pattern = unsafe { $crate::MBogus::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::MBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::MBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::MBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::MBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::MBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::MBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::MAnnotationAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_GROUP_LIST => {
                    let $pattern = unsafe { $crate::MAnnotationGroupList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ANNOTATION_LIST => {
                    let $pattern = unsafe { $crate::MAnnotationList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::MArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CALL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::MCallArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CLASS_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::MClassMemberList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::MConstructorParameterList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::MDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_HASH_MAP_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::MHashMapMemberList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_HASH_SET_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::MHashSetElementList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_MODULE_ITEM_LIST => {
                    let $pattern = unsafe { $crate::MModuleItemList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_OBJECT_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::MObjectMemberList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::MParameterList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_INIT_LIST => {
                    let $pattern = unsafe { $crate::MReportInitList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_LIST => {
                    let $pattern = unsafe { $crate::MReportList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_REPORT_SECTION_LIST => {
                    let $pattern = unsafe { $crate::MReportSectionList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::MStatementList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_SWITCH_CASE_LIST => {
                    let $pattern = unsafe { $crate::MSwitchCaseList::new_unchecked(node) };
                    $body
                }
                $crate::MSyntaxKind::M_VARIABLE_DECLARATOR_LIST => {
                    let $pattern = unsafe { $crate::MVariableDeclaratorList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
