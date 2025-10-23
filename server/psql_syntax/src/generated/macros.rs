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
            node => match $crate::PsqlSyntaxNode::kind(&node) {
                $crate::PsqlSyntaxKind::PSQL_ALIAS => {
                    let $pattern = unsafe { $crate::PsqlAlias::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::PsqlColReference::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DATA_BASE_NAME => {
                    let $pattern = unsafe { $crate::PsqlDataBaseName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlFromClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FUNCTION_BINDING => {
                    let $pattern = unsafe { $crate::PsqlFunctionBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GROUP_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlGroupByClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_HAVING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlHavingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LIMIT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlLimitClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NAME => {
                    let $pattern = unsafe { $crate::PsqlName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NULL_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_OFFSET_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOffsetClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOrderByClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlOrderByExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ROOT => {
                    let $pattern = unsafe { $crate::PsqlRoot::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSelectClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_STMT => {
                    let $pattern = unsafe { $crate::PsqlSelectStmt::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SHEMA_NAME => {
                    let $pattern = unsafe { $crate::PsqlShemaName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STAR => {
                    let $pattern = unsafe { $crate::PsqlStar::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STMT => {
                    let $pattern = unsafe { $crate::PsqlStmt::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUB_QUERY => {
                    let $pattern = unsafe { $crate::PsqlSubQuery::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_BINDING => {
                    let $pattern = unsafe { $crate::PsqlTableBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::PsqlTableColReference::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_NAME => {
                    let $pattern = unsafe { $crate::PsqlTableName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WHERE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlWhereClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS => {
                    let $pattern = unsafe { $crate::PsqlBogus::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::PsqlBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::PsqlBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::PsqlBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::PsqlBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GROUP_BY_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlGroupByItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlOrderByExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlSelectItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STMT_LIST => {
                    let $pattern = unsafe { $crate::PsqlStmtList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
