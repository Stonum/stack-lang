use biome_rowan::{AstNode, declare_node_union};

use crate::{
    AnyMAssignment, AnyMExpression,
    binary_like_expression::{AnyMBinaryLikeExpression, AnyMBinaryLikeLeftExpression},
};

declare_node_union! {
    pub AnyMExpressionLeftSide = AnyMExpression |  AnyMAssignment
}

impl AnyMExpressionLeftSide {
    /// Returns the left most expression of `expression`.
    ///
    /// For example, returns `a` for `(a ? b : c) + d` because it first resolves the
    /// left hand expression of the binary expression, then resolves to the inner expression of the parenthesized
    /// expression, and finally resolves to the test condition of the conditional expression.
    pub fn leftmost(expression: AnyMExpression) -> Self {
        let mut current: Self = expression.into();
        loop {
            match current.left_expression() {
                None => {
                    break current;
                }
                Some(left) => {
                    current = left;
                }
            }
        }
    }

    /// Returns the left side of an expression (an expression where the first child is a `Node` or [None]
    /// if the expression has no left side.
    pub fn left_expression(&self) -> Option<Self> {
        match self {
            Self::AnyMExpression(expression) => {
                let left_expression = match expression {
                    AnyMExpression::MSequenceExpression(expr) => expr.left().ok(),
                    AnyMExpression::MStaticMemberExpression(expr) => expr.object().ok(),
                    AnyMExpression::MComputedMemberExpression(expr) => expr.object(),
                    AnyMExpression::MNewExpression(expr) => expr.callee().ok(),
                    AnyMExpression::MCallExpression(expr) => expr.callee().ok(),
                    AnyMExpression::MConditionalExpression(expr) => expr.test().ok(),
                    AnyMExpression::MAssignmentExpression(expr) => {
                        return expr.left().ok().map(Self::from);
                    }
                    AnyMExpression::MPostUpdateExpression(expr) => {
                        return expr.operand().ok().map(Self::from);
                    }
                    expr => {
                        return AnyMBinaryLikeExpression::cast_ref(expr.syntax()).and_then(
                            |binary_like| {
                                binary_like.left().ok().map(
                                    |AnyMBinaryLikeLeftExpression::AnyMExpression(expression)| {
                                        Self::from(expression)
                                    },
                                )
                            },
                        );
                    }
                };
                left_expression.map(Self::from)
            }
            Self::AnyMAssignment(assignment) => {
                let left: Option<Self> = match assignment {
                    AnyMAssignment::MComputedMemberAssignment(computed) => {
                        return computed.object().map(Self::from);
                    }
                    AnyMAssignment::MStaticMemberAssignment(member) => {
                        return member.object().ok().map(Self::from);
                    }
                    AnyMAssignment::MParenthesizedAssignment(_)
                    | AnyMAssignment::MIdentifierAssignment(_)
                    | AnyMAssignment::MBogusAssignment(_) => None,
                };
                left
            }
        }
    }
}
