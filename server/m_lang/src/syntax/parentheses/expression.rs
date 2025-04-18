use std::borrow::Cow;

use biome_rowan::{match_ast, AstNode};

use crate::syntax::{
    binary_like_expression::{
        should_flatten, AnyMBinaryLikeExpression, AnyMBinaryLikeLeftExpression,
    },
    expression_left_side::AnyMExpressionLeftSide,
    AnyMComputedMember, AnyMExpression, AnyMForInitializer, AnyMLiteralExpression,
    AnyMMemberAssignment, AnyMStatement, MArrayExpression, MAssignmentExpression,
    MBinaryExpression, MBooleanLiteralExpression, MCallExpression, MComputedMemberAssignment,
    MComputedMemberExpression, MComputedMemberName, MConditionalExpression, MConstantExpression,
    MDateLiteralExpression, MExpressionStatement, MForStatement, MFunctionExpression,
    MHashMapExpression, MHashSetExpression, MIdentifierExpression, MInExpression,
    MLogicalExpression, MLongStringLiteralExpression, MNewExpression, MNullLiteralExpression,
    MNumberLiteralExpression, MObjectExpression, MParenthesizedExpression, MPostUpdateExpression,
    MPreUpdateExpression, MPreUpdateOperator, MSequenceExpression, MStaticMemberExpression,
    MStringLiteralExpression, MSuperExpression, MSyntaxKind, MSyntaxNode, MThisExpression,
    MTimeLiteralExpression, MUnaryExpression, MUnaryOperator,
};

use super::NeedsParentheses;

impl NeedsParentheses for AnyMExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyMLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MArrayExpression(expr) => expr.needs_parentheses(),
            Self::MAssignmentExpression(expr) => expr.needs_parentheses(),
            Self::MBinaryExpression(expr) => expr.needs_parentheses(),
            Self::MCallExpression(expr) => expr.needs_parentheses(),
            Self::MComputedMemberExpression(expr) => expr.needs_parentheses(),
            Self::MConditionalExpression(expr) => expr.needs_parentheses(),
            Self::MFunctionExpression(expr) => expr.needs_parentheses(),
            Self::MIdentifierExpression(expr) => expr.needs_parentheses(),
            Self::MInExpression(expr) => expr.needs_parentheses(),
            Self::MLogicalExpression(expr) => expr.needs_parentheses(),
            Self::MNewExpression(expr) => expr.needs_parentheses(),
            Self::MObjectExpression(expr) => expr.needs_parentheses(),
            Self::MParenthesizedExpression(expr) => expr.needs_parentheses(),
            Self::MPostUpdateExpression(expr) => expr.needs_parentheses(),
            Self::MPreUpdateExpression(expr) => expr.needs_parentheses(),
            Self::MSequenceExpression(expr) => expr.needs_parentheses(),
            Self::MStaticMemberExpression(expr) => expr.needs_parentheses(),
            Self::MSuperExpression(expr) => expr.needs_parentheses(),
            Self::MThisExpression(expr) => expr.needs_parentheses(),
            Self::MUnaryExpression(expr) => expr.needs_parentheses(),
            Self::MConstantExpression(expr) => expr.needs_parentheses(),
            Self::MHashMapExpression(expr) => expr.needs_parentheses(),
            Self::MHashSetExpression(expr) => expr.needs_parentheses(),
            Self::MBogusExpression(_) => false,
        }
    }
}

impl NeedsParentheses for AnyMExpressionLeftSide {
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::AnyMExpression(expression) => expression.needs_parentheses(),
            Self::AnyMAssignment(assignment) => assignment.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for AnyMLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::MBooleanLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MNullLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MNumberLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MStringLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MLongStringLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MTimeLiteralExpression(expr) => expr.needs_parentheses(),
            Self::MDateLiteralExpression(expr) => expr.needs_parentheses(),
        }
    }
}

impl NeedsParentheses for MArrayExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MHashSetExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MAssignmentExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match_ast! {
            match &parent {
                MAssignmentExpression(_) => false,
                // `[a = b]`
                MComputedMemberName(_) => false,
                // `x = x + b`
                MExpressionStatement(_) => false,

                MForStatement(for_statement) => {
                     let is_initializer = match for_statement.initializer() {
                        Some(AnyMForInitializer::AnyMExpression(expr)) => {
                            expr.syntax() == self.syntax()
                        }
                        None | Some(_) => false,
                    };
                    let is_update = for_statement
                        .update()
                        .is_some_and(|update| update.syntax() == self.syntax());
                    !(is_initializer || is_update)
                },
                MSequenceExpression(_) => {
                    let mut child = Cow::Borrowed(&parent);
                    for ancestor in parent.ancestors().skip(1) {
                        match ancestor.kind() {
                            MSyntaxKind::M_SEQUENCE_EXPRESSION
                            | MSyntaxKind::M_PARENTHESIZED_EXPRESSION => child = Cow::Owned(ancestor),
                            _ => {
                                let Some(for_statement) = MForStatement::cast(ancestor) else {
                                    break;
                                };
                                let is_initializer = match for_statement.initializer() {
                                    Some(AnyMForInitializer::AnyMExpression(expression)) => {
                                        expression.syntax() == child.as_ref()
                                    }
                                    None | Some(_) => false,
                                };
                                let is_update = for_statement
                                    .update()
                                    .is_some_and(|update| update.syntax() == child.as_ref());
                                return !(is_initializer || is_update);
                            }
                        }
                    }
                    true
                },
                _ => true,
            }
        }
    }
}

impl NeedsParentheses for MBinaryExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        binary_like_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for MBooleanLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MCallExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == MSyntaxKind::M_NEW_EXPRESSION
    }
}

impl NeedsParentheses for MComputedMemberExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        parent.kind() == MSyntaxKind::M_NEW_EXPRESSION
            && member_chain_callee_needs_parens(self.clone().into())
    }
}

impl NeedsParentheses for MConditionalExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        match parent.kind() {
            MSyntaxKind::M_UNARY_EXPRESSION
            // in spread
            | MSyntaxKind::M_SPREAD
            // Binary-like
            | MSyntaxKind::M_LOGICAL_EXPRESSION
            | MSyntaxKind::M_BINARY_EXPRESSION
            | MSyntaxKind::M_IN_EXPRESSION => true,
            _ => {
                match MConditionalExpression::try_cast(parent) {
                    Ok(cond) => cond.test().is_ok_and(|test| test.syntax() == self.syntax()),
                    Err(parent) => update_or_lower_expression_needs_parens(self.syntax(), parent),
                }
            }
        }
    }
}

impl NeedsParentheses for MFunctionExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(
            parent.kind(),
            MSyntaxKind::M_CALL_EXPRESSION | MSyntaxKind::M_NEW_EXPRESSION
        ) || is_first_in_statement(self.syntax())
    }
}

impl NeedsParentheses for MIdentifierExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        let Ok(name) = self.name().and_then(|x| x.value_token()) else {
            return false;
        };
        // In non-strict mode (sloppy mode), `let` may be a variable name.
        // To disambiguate `let` from a let-declaration,
        // we have to enclose `let` between parentheses in some dge cases.
        match parent.kind() {
            MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION
            | MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT => {
                // `(let)[0];`
                // `(let)[0] = 0;`
                // `for( (let)[0] = 0, b = 0;;;) {}`
                // `for( (let)[0] of []) {}`
                name.text_trimmed() == "let"
                    && parent
                        .ancestors()
                        .find(|x| {
                            !AnyMExpression::can_cast(x.kind())
                                && !AnyMMemberAssignment::can_cast(x.kind())
                        })
                        .filter(|x| {
                            matches!(
                                x.kind(),
                                MSyntaxKind::M_EXPRESSION_STATEMENT
                                    | MSyntaxKind::M_FOR_ALL_IN_STATEMENT
                                    | MSyntaxKind::M_FOR_STATEMENT
                            )
                        })
                        .and_then(|x| x.first_child()?.first_token())
                        .is_some_and(|token| token == name)
            }
            _ => false,
        }
    }
}

impl NeedsParentheses for MConstantExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MInExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        is_in_for_initializer(self) || binary_like_needs_parens(self.syntax())
    }
}
/// Add parentheses if the `in` is inside of a `for` initializer (see tests).
fn is_in_for_initializer(expression: &MInExpression) -> bool {
    let statement = expression
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyMStatement::cast);
    match statement {
        Some(AnyMStatement::MForAllInStatement(for_in_statement)) => for_in_statement
            .initializer()
            .is_ok_and(|initializer| initializer.range().contains(expression.range().start())),
        Some(AnyMStatement::MForStatement(for_statement)) => for_statement
            .initializer()
            .is_some_and(|initializer| initializer.range().contains(expression.range().start())),
        _ => false,
    }
}

impl NeedsParentheses for MLogicalExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        if let Some(parent) = self.parent::<MLogicalExpression>() {
            parent.operator() != self.operator()
        } else {
            binary_like_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for MNewExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .is_some_and(|node| node.kind() == MSyntaxKind::M_EXTENDS_CLAUSE)
    }
}

impl NeedsParentheses for MNullLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MNumberLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        self.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                MSyntaxKind::M_STATIC_MEMBER_EXPRESSION | MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT,
            ) || AnyMComputedMember::cast(parent)
                .and_then(|member| member.object())
                .is_some_and(|object| object.syntax() == self.syntax())
        })
    }
}

impl NeedsParentheses for MObjectExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MHashMapExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MParenthesizedExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MPostUpdateExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        unary_like_expression_needs_parens(self.syntax())
    }
}

impl NeedsParentheses for MPreUpdateExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(unary) = self.parent::<MUnaryExpression>() {
            let parent_operator = unary.operator();
            let operator = self.operator();
            (parent_operator == Ok(MUnaryOperator::Plus)
                && operator == Ok(MPreUpdateOperator::Increment))
                || (parent_operator == Ok(MUnaryOperator::Minus)
                    && operator == Ok(MPreUpdateOperator::Decrement))
        } else {
            unary_like_expression_needs_parens(self.syntax())
        }
    }
}

impl NeedsParentheses for MSequenceExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        !matches!(
            parent.kind(),
            MSyntaxKind::M_RETURN_STATEMENT |
            // There's a precedence for writing `x++, y++`
            MSyntaxKind::M_FOR_STATEMENT |
            MSyntaxKind::M_EXPRESSION_STATEMENT |
            MSyntaxKind::M_SEQUENCE_EXPRESSION |
            MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION |
            MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT
        )
    }
}

impl NeedsParentheses for MStaticMemberExpression {
    fn needs_parentheses(&self) -> bool {
        let Some(parent) = self.syntax().parent() else {
            return false;
        };
        matches!(parent.kind(), MSyntaxKind::M_NEW_EXPRESSION)
            && member_chain_callee_needs_parens(self.clone().into())
    }
}

impl NeedsParentheses for MStringLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(expression_statement) = self.parent::<MExpressionStatement>() {
            expression_statement
                .syntax()
                .parent()
                .is_some_and(|grand_parent| {
                    matches!(
                        grand_parent.kind(),
                        MSyntaxKind::M_STATEMENT_LIST | MSyntaxKind::M_MODULE_ITEM_LIST
                    )
                })
        } else {
            false
        }
    }
}

impl NeedsParentheses for MLongStringLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        if let Some(expression_statement) = self.parent::<MExpressionStatement>() {
            expression_statement
                .syntax()
                .parent()
                .is_some_and(|grand_parent| {
                    matches!(
                        grand_parent.kind(),
                        MSyntaxKind::M_STATEMENT_LIST | MSyntaxKind::M_MODULE_ITEM_LIST
                    )
                })
        } else {
            false
        }
    }
}

impl NeedsParentheses for MDateLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MTimeLiteralExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MSuperExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MThisExpression {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MUnaryExpression {
    fn needs_parentheses(&self) -> bool {
        match self.parent::<AnyMExpression>() {
            Some(AnyMExpression::MUnaryExpression(parent_unary)) => {
                let parent_operator = parent_unary.operator();
                let operator = self.operator();

                matches!(operator, Ok(MUnaryOperator::Plus | MUnaryOperator::Minus))
                    && parent_operator == operator
            }
            // A user typing `!foo in bar` probably intended `!(foo instanceof Bar)`,
            // so format to `(!foo) in bar` to what is really happening
            Some(AnyMExpression::MInExpression(_)) => true,
            _ => unary_like_expression_needs_parens(self.syntax()),
        }
    }
}

/// Implements the rules when a node needs parentheses that are common across all [AnyMBinaryLikeExpression] nodes.
fn binary_like_needs_parens(node: &MSyntaxNode) -> bool {
    debug_assert!(AnyMBinaryLikeExpression::can_cast(node.kind()));
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        MSyntaxKind::M_EXTENDS_CLAUSE
        | MSyntaxKind::M_UNARY_EXPRESSION
        // in spread
        | MSyntaxKind::M_SPREAD
        // Callee
        | MSyntaxKind::M_CALL_EXPRESSION
        | MSyntaxKind::M_NEW_EXPRESSION
        // Static member
        | MSyntaxKind::M_STATIC_MEMBER_EXPRESSION
        | MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT => true,
        _ => {
            let Some(node) = AnyMBinaryLikeExpression::cast_ref(node) else {
                return false;
            };
            match AnyMBinaryLikeExpression::try_cast(parent) {
                Ok(parent) => {
                    let (Ok(operator), Ok(parent_operator)) = (node.operator(), parent.operator()) else {
                        // Just to be sure
                        return true;
                    };
                    let precedence = operator.precedence();
                    let parent_precedence = parent_operator.precedence();

                    // If the parent has a higher precedence than parentheses are necessary to not change the semantic meaning
                    // when re-parsing.
                    if parent_precedence > precedence {
                        return true;
                    }

                    let is_right =
                        parent.right().map(AstNode::into_syntax).as_ref() == Ok(node.syntax());
                    // `a ** b ** c`
                    if is_right && parent_precedence == precedence {
                        return true;
                    }

                    // Add parentheses around bitwise and bit shift operators
                    // `a * 3 >> 5` -> `(a * 3) >> 5`
                    if parent_precedence.is_bitwise() || parent_precedence.is_shift() {
                        return true;
                    }

                    // `a % 4 + 4` -> `(a % 4) + 4)`
                    if parent_precedence < precedence && operator.is_remainder() {
                        return parent_precedence.is_additive();
                    }

                    parent_precedence == precedence && !should_flatten(parent_operator, operator)
                }
                Err(parent) => {
                    AnyMComputedMember::cast(parent)
                        .and_then(|member| member.object())
                        .is_some_and(|object| object.syntax() == node.syntax())
                }
            }
        }
    }
}

fn member_chain_callee_needs_parens(node: AnyMExpression) -> bool {
    let mut object_chain = std::iter::successors(Some(node), |expr| match expr {
        AnyMExpression::MStaticMemberExpression(expr) => expr.object().ok(),
        AnyMExpression::MComputedMemberExpression(expr) => expr.object(),
        _ => None,
    });
    object_chain.any(|object| matches!(object, AnyMExpression::MCallExpression(_)))
}

/// Implements the shared logic for when parentheses are necessary for [MPreUpdateExpression], [MPostUpdateExpression], or [MUnaryExpression] expressions.
/// Each expression may implement node specific rules, which is why calling `needs_parens` on the node is preferred.
fn unary_like_expression_needs_parens(expression: &MSyntaxNode) -> bool {
    let Some(parent) = expression.parent() else {
        return false;
    };
    match MBinaryExpression::try_cast(parent) {
        Ok(_binary) => false,
        Err(parent) => update_or_lower_expression_needs_parens(expression, parent),
    }
}

/// Returns `true` if an expression with lower precedence than an update expression needs parentheses.
///
/// This is generally the case if the expression is used in a left hand side, or primary expression context.
fn update_or_lower_expression_needs_parens(expression: &MSyntaxNode, parent: MSyntaxNode) -> bool {
    matches!(
        parent.kind(),
        MSyntaxKind::M_EXTENDS_CLAUSE
            // Callee
            | MSyntaxKind::M_CALL_EXPRESSION
            | MSyntaxKind::M_NEW_EXPRESSION
            // Static member
            | MSyntaxKind::M_STATIC_MEMBER_EXPRESSION
            | MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT
    ) || AnyMComputedMember::cast(parent)
        .and_then(|member| member.object())
        .is_some_and(|object| object.syntax() == expression)
}

/// Returns `true` if this node is at the start of an expression (depends on the passed `mode`).
///
/// Traverses upwards the tree for as long as the `node` is the left most expression until the node isn't
/// the left most node or reached a statement.
fn is_first_in_statement(node: &MSyntaxNode) -> bool {
    let mut current = Cow::Borrowed(node);
    while let Some(parent) = current.parent() {
        let parent = match parent.kind() {
            MSyntaxKind::M_EXPRESSION_STATEMENT => {
                return true;
            }
            MSyntaxKind::M_STATIC_MEMBER_EXPRESSION
            | MSyntaxKind::M_STATIC_MEMBER_ASSIGNMENT
            | MSyntaxKind::M_CALL_EXPRESSION
            | MSyntaxKind::M_NEW_EXPRESSION => parent,
            MSyntaxKind::M_SEQUENCE_EXPRESSION => {
                let expr = MSequenceExpression::unwrap_cast(parent);
                let is_left = expr
                    .left()
                    .is_ok_and(|left| left.syntax() == current.as_ref());
                if is_left {
                    expr.into_syntax()
                } else {
                    break;
                }
            }
            MSyntaxKind::M_COMPUTED_MEMBER_EXPRESSION => {
                let expr = MComputedMemberExpression::unwrap_cast(parent);
                let is_object = expr
                    .object()
                    .is_some_and(|object| object.syntax() == current.as_ref());
                if is_object {
                    expr.into_syntax()
                } else {
                    break;
                }
            }
            MSyntaxKind::M_COMPUTED_MEMBER_ASSIGNMENT => {
                let assignment = MComputedMemberAssignment::unwrap_cast(parent);
                let is_object = assignment
                    .object()
                    .is_some_and(|object| object.syntax() == current.as_ref());
                if is_object {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }
            MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
                let assignment = MAssignmentExpression::unwrap_cast(parent);
                let is_left = assignment
                    .left()
                    .is_ok_and(|left| left.syntax() == current.as_ref());
                if is_left {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }
            MSyntaxKind::M_CONDITIONAL_EXPRESSION => {
                let cond = MConditionalExpression::unwrap_cast(parent);
                let is_test = cond
                    .test()
                    .is_ok_and(|test| test.syntax() == current.as_ref());
                if is_test {
                    cond.into_syntax()
                } else {
                    break;
                }
            }
            _ => {
                let Some(binary_like) = AnyMBinaryLikeExpression::cast(parent) else {
                    break;
                };
                let is_left = binary_like.left().is_ok_and(|left| match left {
                    AnyMBinaryLikeLeftExpression::AnyMExpression(expression) => {
                        expression.syntax() == current.as_ref()
                    }
                });
                if is_left {
                    binary_like.into_syntax()
                } else {
                    break;
                }
            }
        };
        current = Cow::Owned(parent);
    }
    false
}
