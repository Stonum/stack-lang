use crate::prelude::*;
use crate::rules::expressions::computed_member_expression::FormatComputedMemberLookup;
use mlang_syntax::{
    MCallExpression, MCallExpressionFields, MComputedMemberExpression, MStaticMemberExpression,
    MStaticMemberExpressionFields, MSyntaxNode,
};
use biome_formatter::write;
use biome_rowan::AstNode;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub(crate) enum CallExpressionPosition {
    /// At the start of a call chain.
    /// `of` in `of().test`
    Start,

    /// Somewhere in the middle.
    ///
    /// `b` in `a.b().c()`
    Middle,

    /// At the end of a call chain (root)
    /// `c` in `a.b.c()`
    End,
}

/// Data structure that holds the node with its formatted version
#[derive(Clone, Debug)]
pub(crate) enum ChainMember {
    /// Holds onto a [mlang_syntax::MStaticMemberExpression]
    StaticMember { expression: MStaticMemberExpression },

    /// Holds onto a [mlang_syntax::MCallExpression]
    CallExpression {
        expression: MCallExpression,
        position: CallExpressionPosition,
    },

    /// Holds onto a [mlang_syntax::MComputedMemberExpression]
    ComputedMember {
        expression: MComputedMemberExpression,
    },

    /// Any other node that are not  [mlang_syntax::MCallExpression] or [mlang_syntax::MStaticMemberExpression]
    /// Are tracked using this variant
    Node(MSyntaxNode),
}

impl ChainMember {
    /// checks if the current node is a [mlang_syntax::MCallExpression], or a [mlang_syntax::MImportExpression]
    pub fn is_call_like_expression(&self) -> bool {
        match self {
            ChainMember::CallExpression { .. } => true,
            ChainMember::Node(node) => MCallExpression::can_cast(node.kind()),
            _ => false,
        }
    }

    pub(crate) const fn is_call_expression(&self) -> bool {
        matches!(self, ChainMember::CallExpression { .. })
    }

    pub(crate) fn syntax(&self) -> &MSyntaxNode {
        match self {
            ChainMember::StaticMember { expression, .. } => expression.syntax(),
            ChainMember::CallExpression { expression, .. } => expression.syntax(),
            ChainMember::ComputedMember { expression, .. } => expression.syntax(),
            ChainMember::Node(node) => node,
        }
    }

    pub const fn is_computed_expression(&self) -> bool {
        matches!(self, ChainMember::ComputedMember { .. })
    }
}

impl Format<MFormatContext> for ChainMember {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            ChainMember::StaticMember { expression } => {
                let MStaticMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    operator_token,
                    member,
                } = expression.as_fields();

                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        operator_token.format(),
                        member.format(),
                        format_trailing_comments(expression.syntax())
                    ]
                )
            }

            ChainMember::CallExpression {
                expression,
                position,
            } => {
                let MCallExpressionFields {
                    // Formatted as part of the previous item
                    callee: _,
                    arguments,
                } = expression.as_fields();

                match position {
                    CallExpressionPosition::Start => write!(f, [expression.format()]),
                    CallExpressionPosition::Middle => {
                        write!(
                            f,
                            [
                                format_leading_comments(expression.syntax()),
                                arguments.format(),
                                format_trailing_comments(expression.syntax())
                            ]
                        )
                    }
                    CallExpressionPosition::End => {
                        write!(f, [arguments.format(),])
                    }
                }
            }
            ChainMember::ComputedMember { expression } => {
                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        FormatComputedMemberLookup::new(&expression.clone().into()),
                        format_trailing_comments(expression.syntax())
                    ]
                )
            }
            ChainMember::Node(node) => {
                write!(f, [node.format()])
            }
        }
    }
}
