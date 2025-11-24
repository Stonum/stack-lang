//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{PsqlSyntaxToken as SyntaxToken, generated::nodes::*};
use biome_rowan::AstNode;
use std::iter::once;
impl PsqlAlias {
    pub fn with_as_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_value(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlBinaryExpression {
    pub fn with_left(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_right(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlBooleanLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlColReference {
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlDataBaseName {
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlDeleteStatement {
    pub fn with_delete_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: PsqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_using(self, element: Option<PsqlDeleteUsingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<PsqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlDeleteUsingClause {
    pub fn with_using_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_any_psql_from_expression(self, element: AnyPsqlFromExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlFromClause {
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_clause(self, element: AnyPsqlFromExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlFunctionBinding {
    pub fn with_schema(self, element: Option<PsqlShemaName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_arguments(self, element: PsqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
    pub fn with_alias(self, element: Option<PsqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlGroupByClause {
    pub fn with_group_by_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlGroupByItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlHavingClause {
    pub fn with_having_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlInsertColumns {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlInsertColumnList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl PsqlInsertStatement {
    pub fn with_insert_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_into_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: PsqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_columns(self, element: Option<PsqlInsertColumns>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_source(self, element: AnyPsqlInsertSource) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlInsertValues {
    pub fn with_values_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlLimitClause {
    pub fn with_limit_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_limit_count(self, element: PsqlNumberLiteralExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlLogicalExpression {
    pub fn with_left(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_right(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlName {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlNullLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlNumberLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlOffsetClause {
    pub fn with_offset_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_start(self, element: PsqlNumberLiteralExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlOrderByClause {
    pub fn with_order_by_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlOrderByExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlOrderByExpression {
    pub fn with_item(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_order_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlParenthesizedExpression {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl PsqlRoot {
    pub fn with_stmt(self, element: PsqlStatementList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eof_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlSelectClause {
    pub fn with_select_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_list(self, element: PsqlSelectItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlSelectExpression {
    pub fn with_expr(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_alias(self, element: Option<PsqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlSelectStatement {
    pub fn with_select_clause(self, element: PsqlSelectClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_from_clause(self, element: Option<PsqlFromClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<PsqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_group_by_clause(self, element: Option<PsqlGroupByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_having_clause(self, element: Option<PsqlHavingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_order_by_clause(self, element: Option<PsqlOrderByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_limit_clause(self, element: Option<PsqlLimitClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_offset_clause(self, element: Option<PsqlOffsetClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            7usize..=7usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlSetClause {
    pub fn with_set_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlSetItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlSetItem {
    pub fn with_column(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eq_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_expr(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlShemaName {
    pub fn with_base(self, element: Option<PsqlDataBaseName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl PsqlStar {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlStringLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlTableBinding {
    pub fn with_table(self, element: PsqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_alias(self, element: Option<PsqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlTableColReference {
    pub fn with_table(self, element: PsqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlTableName {
    pub fn with_schema(self, element: Option<PsqlShemaName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: PsqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlUpdateStatement {
    pub fn with_update_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: PsqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_set_clause(self, element: PsqlSetClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_where_clause(self, element: Option<PsqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlWhereClause {
    pub fn with_where_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnyPsqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
