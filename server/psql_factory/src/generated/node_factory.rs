//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use psql_syntax::{
    PsqlSyntaxElement as SyntaxElement, PsqlSyntaxNode as SyntaxNode,
    PsqlSyntaxToken as SyntaxToken, *,
};
pub fn psql_alias(value: PsqlName) -> PsqlAliasBuilder {
    PsqlAliasBuilder {
        value,
        as_token: None,
    }
}
pub struct PsqlAliasBuilder {
    value: PsqlName,
    as_token: Option<SyntaxToken>,
}
impl PsqlAliasBuilder {
    pub fn with_as_token(mut self, as_token: SyntaxToken) -> Self {
        self.as_token = Some(as_token);
        self
    }
    pub fn build(self) -> PsqlAlias {
        PsqlAlias::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_ALIAS,
            [
                self.as_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn psql_binary_expression(
    left: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    right: AnyPsqlExpression,
) -> PsqlBinaryExpression {
    PsqlBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn psql_boolean_literal_expression(value_token: SyntaxToken) -> PsqlBooleanLiteralExpression {
    PsqlBooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOOLEAN_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_col_reference(name: PsqlName) -> PsqlColReferenceBuilder {
    PsqlColReferenceBuilder { name, alias: None }
}
pub struct PsqlColReferenceBuilder {
    name: PsqlName,
    alias: Option<PsqlAlias>,
}
impl PsqlColReferenceBuilder {
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlColReference {
        PsqlColReference::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_COL_REFERENCE,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_data_base_name(name: PsqlName, dot_token: SyntaxToken) -> PsqlDataBaseName {
    PsqlDataBaseName::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_DATA_BASE_NAME,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
        ],
    ))
}
pub fn psql_from_clause(
    from_token: SyntaxToken,
    any_psql_from_expression: AnyPsqlFromExpression,
) -> PsqlFromClause {
    PsqlFromClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_FROM_CLAUSE,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(any_psql_from_expression.into_syntax())),
        ],
    ))
}
pub fn psql_function_binding(
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlArgumentList,
    r_paren_token: SyntaxToken,
) -> PsqlFunctionBindingBuilder {
    PsqlFunctionBindingBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        schema: None,
        alias: None,
    }
}
pub struct PsqlFunctionBindingBuilder {
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlArgumentList,
    r_paren_token: SyntaxToken,
    schema: Option<PsqlShemaName>,
    alias: Option<PsqlAlias>,
}
impl PsqlFunctionBindingBuilder {
    pub fn with_schema(mut self, schema: PsqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlFunctionBinding {
        PsqlFunctionBinding::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_FUNCTION_BINDING,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_group_by_clause(
    group_by_token: SyntaxToken,
    list: PsqlGroupByItemList,
) -> PsqlGroupByClause {
    PsqlGroupByClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_GROUP_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(group_by_token)),
            Some(SyntaxElement::Node(list.into_syntax())),
        ],
    ))
}
pub fn psql_having_clause(
    having_token: SyntaxToken,
    any_psql_expression: AnyPsqlExpression,
) -> PsqlHavingClause {
    PsqlHavingClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_HAVING_CLAUSE,
        [
            Some(SyntaxElement::Token(having_token)),
            Some(SyntaxElement::Node(any_psql_expression.into_syntax())),
        ],
    ))
}
pub fn psql_limit_clause(
    limit_token: SyntaxToken,
    limit_count: PsqlNumberLiteralExpression,
) -> PsqlLimitClause {
    PsqlLimitClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_LIMIT_CLAUSE,
        [
            Some(SyntaxElement::Token(limit_token)),
            Some(SyntaxElement::Node(limit_count.into_syntax())),
        ],
    ))
}
pub fn psql_logical_expression(
    left: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    right: AnyPsqlExpression,
) -> PsqlLogicalExpression {
    PsqlLogicalExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn psql_name(value_token: SyntaxToken) -> PsqlName {
    PsqlName::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_null_literal_expression(value_token: SyntaxToken) -> PsqlNullLiteralExpression {
    PsqlNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_number_literal_expression(value_token: SyntaxToken) -> PsqlNumberLiteralExpression {
    PsqlNumberLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NUMBER_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_offset_clause(
    offset_token: SyntaxToken,
    start: PsqlNumberLiteralExpression,
) -> PsqlOffsetClause {
    PsqlOffsetClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_OFFSET_CLAUSE,
        [
            Some(SyntaxElement::Token(offset_token)),
            Some(SyntaxElement::Node(start.into_syntax())),
        ],
    ))
}
pub fn psql_order_by_clause(
    order_by_token: SyntaxToken,
    list: PsqlOrderByExpressionList,
) -> PsqlOrderByClause {
    PsqlOrderByClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ORDER_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(order_by_token)),
            Some(SyntaxElement::Node(list.into_syntax())),
        ],
    ))
}
pub fn psql_order_by_expression(item: AnyPsqlExpression) -> PsqlOrderByExpressionBuilder {
    PsqlOrderByExpressionBuilder {
        item,
        order_token: None,
    }
}
pub struct PsqlOrderByExpressionBuilder {
    item: AnyPsqlExpression,
    order_token: Option<SyntaxToken>,
}
impl PsqlOrderByExpressionBuilder {
    pub fn with_order_token(mut self, order_token: SyntaxToken) -> Self {
        self.order_token = Some(order_token);
        self
    }
    pub fn build(self) -> PsqlOrderByExpression {
        PsqlOrderByExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.item.into_syntax())),
                self.order_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: AnyPsqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlParenthesizedExpression {
    PsqlParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_root(stmt: PsqlStmtList, eof_token: SyntaxToken) -> PsqlRoot {
    PsqlRoot::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ROOT,
        [
            Some(SyntaxElement::Node(stmt.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn psql_select_clause(select_token: SyntaxToken, list: PsqlSelectItemList) -> PsqlSelectClause {
    PsqlSelectClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SELECT_CLAUSE,
        [
            Some(SyntaxElement::Token(select_token)),
            Some(SyntaxElement::Node(list.into_syntax())),
        ],
    ))
}
pub fn psql_select_stmt(select_clause: PsqlSelectClause) -> PsqlSelectStmtBuilder {
    PsqlSelectStmtBuilder {
        select_clause,
        from_clause: None,
        where_clause: None,
        group_by_clause: None,
        having_clause: None,
        order_by_clause: None,
        limit_clause: None,
        offset_clause: None,
    }
}
pub struct PsqlSelectStmtBuilder {
    select_clause: PsqlSelectClause,
    from_clause: Option<PsqlFromClause>,
    where_clause: Option<PsqlWhereClause>,
    group_by_clause: Option<PsqlGroupByClause>,
    having_clause: Option<PsqlHavingClause>,
    order_by_clause: Option<PsqlOrderByClause>,
    limit_clause: Option<PsqlLimitClause>,
    offset_clause: Option<PsqlOffsetClause>,
}
impl PsqlSelectStmtBuilder {
    pub fn with_from_clause(mut self, from_clause: PsqlFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: PsqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_group_by_clause(mut self, group_by_clause: PsqlGroupByClause) -> Self {
        self.group_by_clause = Some(group_by_clause);
        self
    }
    pub fn with_having_clause(mut self, having_clause: PsqlHavingClause) -> Self {
        self.having_clause = Some(having_clause);
        self
    }
    pub fn with_order_by_clause(mut self, order_by_clause: PsqlOrderByClause) -> Self {
        self.order_by_clause = Some(order_by_clause);
        self
    }
    pub fn with_limit_clause(mut self, limit_clause: PsqlLimitClause) -> Self {
        self.limit_clause = Some(limit_clause);
        self
    }
    pub fn with_offset_clause(mut self, offset_clause: PsqlOffsetClause) -> Self {
        self.offset_clause = Some(offset_clause);
        self
    }
    pub fn build(self) -> PsqlSelectStmt {
        PsqlSelectStmt::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_SELECT_STMT,
            [
                Some(SyntaxElement::Node(self.select_clause.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.group_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.having_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.order_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.limit_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.offset_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_shema_name(name: PsqlName, dot_token: SyntaxToken) -> PsqlShemaNameBuilder {
    PsqlShemaNameBuilder {
        name,
        dot_token,
        base: None,
    }
}
pub struct PsqlShemaNameBuilder {
    name: PsqlName,
    dot_token: SyntaxToken,
    base: Option<PsqlDataBaseName>,
}
impl PsqlShemaNameBuilder {
    pub fn with_base(mut self, base: PsqlDataBaseName) -> Self {
        self.base = Some(base);
        self
    }
    pub fn build(self) -> PsqlShemaName {
        PsqlShemaName::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_SHEMA_NAME,
            [
                self.base
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.dot_token)),
            ],
        ))
    }
}
pub fn psql_star(value_token: SyntaxToken) -> PsqlStar {
    PsqlStar::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_stmt(psql_select_stmt: PsqlSelectStmt) -> PsqlStmt {
    PsqlStmt::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STMT,
        [Some(SyntaxElement::Node(psql_select_stmt.into_syntax()))],
    ))
}
pub fn psql_string_literal_expression(value_token: SyntaxToken) -> PsqlStringLiteralExpression {
    PsqlStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_sub_query(
    l_paren_token: SyntaxToken,
    psql_select_stmt: PsqlSelectStmt,
    r_paren_token: SyntaxToken,
    alias: PsqlAlias,
) -> PsqlSubQuery {
    PsqlSubQuery::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SUB_QUERY,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(psql_select_stmt.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(alias.into_syntax())),
        ],
    ))
}
pub fn psql_table_binding(table: PsqlTableName) -> PsqlTableBindingBuilder {
    PsqlTableBindingBuilder { table, alias: None }
}
pub struct PsqlTableBindingBuilder {
    table: PsqlTableName,
    alias: Option<PsqlAlias>,
}
impl PsqlTableBindingBuilder {
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlTableBinding {
        PsqlTableBinding::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_TABLE_BINDING,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_table_col_reference(
    table: PsqlTableName,
    dot_token: SyntaxToken,
    name: PsqlName,
) -> PsqlTableColReferenceBuilder {
    PsqlTableColReferenceBuilder {
        table,
        dot_token,
        name,
        alias: None,
    }
}
pub struct PsqlTableColReferenceBuilder {
    table: PsqlTableName,
    dot_token: SyntaxToken,
    name: PsqlName,
    alias: Option<PsqlAlias>,
}
impl PsqlTableColReferenceBuilder {
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlTableColReference {
        PsqlTableColReference::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_TABLE_COL_REFERENCE,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                Some(SyntaxElement::Token(self.dot_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_table_name(name: PsqlName) -> PsqlTableNameBuilder {
    PsqlTableNameBuilder { name, schema: None }
}
pub struct PsqlTableNameBuilder {
    name: PsqlName,
    schema: Option<PsqlShemaName>,
}
impl PsqlTableNameBuilder {
    pub fn with_schema(mut self, schema: PsqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn build(self) -> PsqlTableName {
        PsqlTableName::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_TABLE_NAME,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn psql_where_clause(
    where_token: SyntaxToken,
    any_psql_expression: AnyPsqlExpression,
) -> PsqlWhereClause {
    PsqlWhereClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_WHERE_CLAUSE,
        [
            Some(SyntaxElement::Token(where_token)),
            Some(SyntaxElement::Node(any_psql_expression.into_syntax())),
        ],
    ))
}
pub fn psql_argument_list<I, S>(items: I, separators: S) -> PsqlArgumentList
where
    I: IntoIterator<Item = AnyPsqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlArgumentList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_group_by_item_list<I, S>(items: I, separators: S) -> PsqlGroupByItemList
where
    I: IntoIterator<Item = AnyPsqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlGroupByItemList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_GROUP_BY_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_order_by_expression_list<I, S>(items: I, separators: S) -> PsqlOrderByExpressionList
where
    I: IntoIterator<Item = PsqlOrderByExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlOrderByExpressionList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_select_item_list<I, S>(items: I, separators: S) -> PsqlSelectItemList
where
    I: IntoIterator<Item = AnyPsqlSelectItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlSelectItemList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SELECT_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_stmt_list<I>(items: I) -> PsqlStmtList
where
    I: IntoIterator<Item = PsqlStmt>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlStmtList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STMT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_bogus<I>(slots: I) -> PsqlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogus::unwrap_cast(SyntaxNode::new_detached(PsqlSyntaxKind::PSQL_BOGUS, slots))
}
pub fn psql_bogus_assignment<I>(slots: I) -> PsqlBogusAssignment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusAssignment::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_ASSIGNMENT,
        slots,
    ))
}
pub fn psql_bogus_binding<I>(slots: I) -> PsqlBogusBinding
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusBinding::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_BINDING,
        slots,
    ))
}
pub fn psql_bogus_expression<I>(slots: I) -> PsqlBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn psql_bogus_member<I>(slots: I) -> PsqlBogusMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusMember::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_MEMBER,
        slots,
    ))
}
pub fn psql_bogus_parameter<I>(slots: I) -> PsqlBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn psql_bogus_statement<I>(slots: I) -> PsqlBogusStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusStatement::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_STATEMENT,
        slots,
    ))
}
