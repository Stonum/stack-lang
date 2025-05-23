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
            node => match $crate::syntax::MSyntaxNode::kind(&node) {
                $crate::syntax::MSyntaxKind::M_ANNOTATION_ATTRIBUTE => {
                    let $pattern =
                        unsafe { $crate::syntax::MAnnotationAttribute::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_BINDING => {
                    let $pattern =
                        unsafe { $crate::syntax::MAnnotationBinding::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MAnnotationElement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_GROUP => {
                    let $pattern = unsafe { $crate::syntax::MAnnotationGroup::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ARRAY_HOLE => {
                    let $pattern = unsafe { $crate::syntax::MArrayHole::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MAssignmentExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BINARY_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BLOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MBlockStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BREAK_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MBreakStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CALL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::syntax::MCallArguments::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CASE_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MCaseClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CATCH_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MCatchClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CATCH_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::syntax::MCatchDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CLASS_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::syntax::MClassDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MComputedMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MComputedMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_COMPUTED_MEMBER_NAME => {
                    let $pattern =
                        unsafe { $crate::syntax::MComputedMemberName::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONDITIONAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MConditionalExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONSTANT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MConstantExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MConstructorClassMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONSTRUCTOR_PARAMETERS => {
                    let $pattern =
                        unsafe { $crate::syntax::MConstructorParameters::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONTINUE_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MContinueStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_DATE_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MDateLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_DEBUG_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MDebugStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_DIRECTIVE => {
                    let $pattern = unsafe { $crate::syntax::MDirective::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EXTENDED_BINDING => {
                    let $pattern = unsafe { $crate::syntax::MExtendedBinding::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MElseClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EMPTY_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MEmptyClassMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EXPRESSION_SNIPPED => {
                    let $pattern =
                        unsafe { $crate::syntax::MExpressionSnipped::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EXPRESSION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MExpressionStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FINALLY_CLAUSE => {
                    let $pattern = unsafe { $crate::syntax::MFinallyClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FOR_ALL_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MForAllStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FOR_ITERATOR_FACTORY => {
                    let $pattern =
                        unsafe { $crate::syntax::MForIteratorFactory::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FOR_ALL_IN_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MForAllInStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MForStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FOR_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::syntax::MForVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FORMAL_PARAMETER => {
                    let $pattern = unsafe { $crate::syntax::MFormalParameter::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FUNCTION_BODY => {
                    let $pattern = unsafe { $crate::syntax::MFunctionBody::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::syntax::MFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_GETTER_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MGetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_HASH_MAP_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MHashMapExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_HASH_SET_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MHashSetExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_IDENTIFIER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MIdentifierAssignment::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_IDENTIFIER_BINDING => {
                    let $pattern =
                        unsafe { $crate::syntax::MIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_IDENTIFIER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MIdentifierExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MInExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::syntax::MInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_LITERAL_MEMBER_NAME => {
                    let $pattern =
                        unsafe { $crate::syntax::MLiteralMemberName::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_LOGICAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_LONG_STRING_LITERAL_EXPRESSION => {
                    let $pattern = unsafe {
                        $crate::syntax::MLongStringLiteralExpression::new_unchecked(node)
                    };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MMethodClassMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_MODULE => {
                    let $pattern = unsafe { $crate::syntax::MModule::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_NAME => {
                    let $pattern = unsafe { $crate::syntax::MName::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_NEW_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MNewExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_NULL_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_OBJECT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MObjectExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PARENTHESIZED_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MParenthesizedAssignment::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PARAMETERS => {
                    let $pattern = unsafe { $crate::syntax::MParameters::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_POST_UPDATE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MPostUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PRE_UPDATE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PROPERTY_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REFERENCE_IDENTIFIER => {
                    let $pattern =
                        unsafe { $crate::syntax::MReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT => {
                    let $pattern = unsafe { $crate::syntax::MReport::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_FILE => {
                    let $pattern = unsafe { $crate::syntax::MReportFile::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_NAME => {
                    let $pattern = unsafe { $crate::syntax::MReportName::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_SECTION => {
                    let $pattern = unsafe { $crate::syntax::MReportSection::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_SECTION_NAME => {
                    let $pattern =
                        unsafe { $crate::syntax::MReportSectionName::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REST_PARAMETER => {
                    let $pattern = unsafe { $crate::syntax::MRestParameter::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_RETURN_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MReturnStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SCRIPT => {
                    let $pattern = unsafe { $crate::syntax::MScript::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SEQUENCE_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MSequenceExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SETTER_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::syntax::MSetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe {
                        $crate::syntax::MShorthandPropertyObjectMember::new_unchecked(node)
                    };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SPREAD => {
                    let $pattern = unsafe { $crate::syntax::MSpread::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MStaticMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MStaticMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SUPER_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MSuperExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SWITCH_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MSwitchStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_THIS_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MThisExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_THROW_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MThrowStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_TIME_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::syntax::MTimeLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_TRY_FINALLY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_TRY_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MTryStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::syntax::MVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::syntax::MVariableDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_VARIABLE_DECLARATOR => {
                    let $pattern =
                        unsafe { $crate::syntax::MVariableDeclarator::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_VARIABLE_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::syntax::MVariableStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS => {
                    let $pattern = unsafe { $crate::syntax::MBogus::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::syntax::MBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::syntax::MBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::syntax::MBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::syntax::MBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::syntax::MBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::syntax::MBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_ATTRIBUTE_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MAnnotationAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_GROUP_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MAnnotationGroupList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ANNOTATION_LIST => {
                    let $pattern = unsafe { $crate::syntax::MAnnotationList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_ARRAY_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CALL_ARGUMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MCallArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CLASS_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::syntax::MClassMemberList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MConstructorParameterList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::syntax::MDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_HASH_MAP_MEMBER_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MHashMapMemberList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_HASH_SET_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MHashSetElementList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_MODULE_ITEM_LIST => {
                    let $pattern = unsafe { $crate::syntax::MModuleItemList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_OBJECT_MEMBER_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MObjectMemberList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::syntax::MParameterList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_INIT_LIST => {
                    let $pattern = unsafe { $crate::syntax::MReportInitList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_LIST => {
                    let $pattern = unsafe { $crate::syntax::MReportList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_REPORT_SECTION_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MReportSectionList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::syntax::MStatementList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_SWITCH_CASE_LIST => {
                    let $pattern = unsafe { $crate::syntax::MSwitchCaseList::new_unchecked(node) };
                    $body
                }
                $crate::syntax::MSyntaxKind::M_VARIABLE_DECLARATOR_LIST => {
                    let $pattern =
                        unsafe { $crate::syntax::MVariableDeclaratorList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
