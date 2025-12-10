use crate::{MParenthesizedAssignment, MParenthesizedExpression, MSyntaxNode, MSyntaxToken};

use biome_rowan::{AstNode, SyntaxResult, declare_node_union};

declare_node_union! {
    pub AnyMParenthesized =
        MParenthesizedExpression
        | MParenthesizedAssignment
}

impl AnyMParenthesized {
    pub fn l_paren_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            Self::MParenthesizedExpression(expression) => expression.l_paren_token(),
            Self::MParenthesizedAssignment(assignment) => assignment.l_paren_token(),
        }
    }

    pub fn inner(&self) -> SyntaxResult<MSyntaxNode> {
        match self {
            Self::MParenthesizedExpression(expression) => {
                expression.expression().map(AstNode::into_syntax)
            }
            Self::MParenthesizedAssignment(assignment) => {
                assignment.assignment().map(AstNode::into_syntax)
            }
        }
    }

    pub fn r_paren_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            Self::MParenthesizedExpression(expression) => expression.r_paren_token(),
            Self::MParenthesizedAssignment(assignment) => assignment.r_paren_token(),
        }
    }
}
