//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use psql_syntax::{PsqlSyntaxKind, PsqlSyntaxKind::*, T, *};
#[derive(Debug)]
pub struct PsqlSyntaxFactory;
impl SyntaxFactory for PsqlSyntaxFactory {
    type Kind = PsqlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            PSQL_BOGUS
            | PSQL_BOGUS_ASSIGNMENT
            | PSQL_BOGUS_BINDING
            | PSQL_BOGUS_EXPRESSION
            | PSQL_BOGUS_MEMBER
            | PSQL_BOGUS_PARAMETER
            | PSQL_BOGUS_STATEMENT => RawSyntaxNode::new(kind, children.into_iter().map(Some)),
            PSQL_ALIAS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![as] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ALIAS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ALIAS, children)
            }
            PSQL_BINARY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if matches!(
                        element.kind(),
                        T ! [<]
                            | T ! [>]
                            | T ! [<=]
                            | T ! [>=]
                            | T ! [=]
                            | T ! [!=]
                            | T ! [+]
                            | T ! [-]
                            | T ! [*]
                            | T ! [/]
                            | T ! [%]
                            | T ! [&]
                            | T ! [|]
                    ) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_BINARY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_BINARY_EXPRESSION, children)
            }
            PSQL_BOOLEAN_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if matches!(element.kind(), T![true] | T![false]) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_BOOLEAN_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_BOOLEAN_LITERAL_EXPRESSION, children)
            }
            PSQL_COL_REFERENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlAlias::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_COL_REFERENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_COL_REFERENCE, children)
            }
            PSQL_DATA_BASE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [.] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DATA_BASE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DATA_BASE_NAME, children)
            }
            PSQL_FROM_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![from] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlFromExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_FROM_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_FROM_CLAUSE, children)
            }
            PSQL_FUNCTION_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlShemaName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T!['('] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlArgumentList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![')'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlAlias::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_FUNCTION_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_FUNCTION_BINDING, children)
            }
            PSQL_GROUP_BY_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![group_by] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlGroupByItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_GROUP_BY_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_GROUP_BY_CLAUSE, children)
            }
            PSQL_HAVING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![having] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_HAVING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_HAVING_CLAUSE, children)
            }
            PSQL_LIMIT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![limit] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlNumberLiteralExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_LIMIT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_LIMIT_CLAUSE, children)
            }
            PSQL_LOGICAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if matches!(element.kind(), T![and] | T![or]) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_LOGICAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_LOGICAL_EXPRESSION, children)
            }
            PSQL_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == IDENT {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_NAME, children)
            }
            PSQL_NULL_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![null] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_NULL_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_NULL_LITERAL_EXPRESSION, children)
            }
            PSQL_NUMBER_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == PSQL_NUMBER_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_NUMBER_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_NUMBER_LITERAL_EXPRESSION, children)
            }
            PSQL_OFFSET_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![offset] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlNumberLiteralExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_OFFSET_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_OFFSET_CLAUSE, children)
            }
            PSQL_ORDER_BY_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![order_by] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlOrderByExpressionList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ORDER_BY_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ORDER_BY_CLAUSE, children)
            }
            PSQL_ORDER_BY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if matches!(element.kind(), T![asc] | T![desc]) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ORDER_BY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ORDER_BY_EXPRESSION, children)
            }
            PSQL_PARENTHESIZED_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['('] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![')'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_PARENTHESIZED_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_PARENTHESIZED_EXPRESSION, children)
            }
            PSQL_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlStmtList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![EOF] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ROOT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ROOT, children)
            }
            PSQL_SELECT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![select] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlSelectItemList::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SELECT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SELECT_CLAUSE, children)
            }
            PSQL_SELECT_STMT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlSelectClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlFromClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlWhereClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlGroupByClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlHavingClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlOrderByClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlLimitClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlOffsetClause::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SELECT_STMT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SELECT_STMT, children)
            }
            PSQL_SHEMA_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlDataBaseName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [.] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SHEMA_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SHEMA_NAME, children)
            }
            PSQL_STAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [*] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_STAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_STAR, children)
            }
            PSQL_STMT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlSelectStmt::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_STMT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_STMT, children)
            }
            PSQL_STRING_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == PSQL_STRING_LITERAL {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_STRING_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_STRING_LITERAL_EXPRESSION, children)
            }
            PSQL_SUB_QUERY => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T!['('] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlSelectStmt::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T![')'] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlAlias::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SUB_QUERY.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SUB_QUERY, children)
            }
            PSQL_TABLE_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlTableName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlAlias::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TABLE_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TABLE_BINDING, children)
            }
            PSQL_TABLE_COL_REFERENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlTableName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if element.kind() == T ! [.] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlAlias::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TABLE_COL_REFERENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TABLE_COL_REFERENCE, children)
            }
            PSQL_TABLE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if PsqlShemaName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if PsqlName::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TABLE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TABLE_NAME, children)
            }
            PSQL_WHERE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == T![where] {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyPsqlExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_WHERE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_WHERE_CLAUSE, children)
            }
            PSQL_ARGUMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyPsqlExpression::can_cast,
                T ! [,],
                false,
            ),
            PSQL_GROUP_BY_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyPsqlExpression::can_cast,
                T ! [,],
                true,
            ),
            PSQL_ORDER_BY_EXPRESSION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                PsqlOrderByExpression::can_cast,
                T ! [,],
                false,
            ),
            PSQL_SELECT_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnyPsqlSelectItem::can_cast,
                T ! [,],
                true,
            ),
            PSQL_STMT_LIST => Self::make_node_list_syntax(kind, children, PsqlStmt::can_cast),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
