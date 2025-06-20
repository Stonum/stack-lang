//! This module implements the formatting of binary like nodes.
//! Binary like nodes are nodes with `left` and `right` expressions. They include:
//! - [MBinaryExpression]
//! - [MLogicalExpression]
//! - [MInExpression]
//! - [MInstanceofExpression]
//!
//! The challenge of formatting binary like expressions is that we want to format binary expression
//! chains, when possible, together but they are represented as a deep structured tree in the CST.
//!
//! For example,
//!
//! ```JavaScript
//! some && thing && elsewhere || happy
//! ```
//!
//! Is parsed as
//!
//! ```block
//! MLogicalExpression {
//!     left: MLogicalExpression {
//!         left: MLogicalExpression {
//!             left: "some"
//!             operator: "&&",
//!             right: "thing"
//!         }
//!         operator: "&&"
//!         right: "elsewhere"
//!     }
//!     operator: "||"
//!     right: "happy"
//! }
//! ```
//!
//! The goal is to format all the left and right sides together that don't require parentheses (mainly comes down to whether the parent and its left side's operator have the same precedence).
//!
//! This is achieved by traversing down the left side of a binary expression until it reaches the first expression that can't be flattened.
//! For `some && thing && elsewhere || happy`, the implementation checks if the first left-side `some && thing && elsewhere` can be grouped.
//! This isn't the case because the left side operator `&&` differs from the parent's `||` operator.
//!
//! That means, we found the end of the first `group` and the left-side of the group is `some && thing && elsewhere`.
//! The algorithm traverses upwards and adds all right-sides of the parent binary like expressions to the group until it reaches the root.
//! In the example, this only is the `|| happy`.
//!
//! Thus, the first group is: `[Left(some && thing && elsewhere), Right(|| happy)]`. The formatting formats the left side
//! as is (the call will recurse into the [AnyMBinaryLikeExpression] formatting again) but formats the operator with the right side.
//!
//! Now, let's see how the implementation groups the `some && thing && elsewhere`. It first traverses to the left most binary like expression,
//! which is `some && thing`. It then adds this as a `Left` side to the group. From here, the algorithm traverses upwards and adds all right sides
//! of the binary expression. These are: `&& thing` and `&& elsewhere`.
//! The complete group is: `[Left(some), Right(&& thing), Right(&& elsewhere)]`.
//!
//! Each side in the group gets formatted in order, starting with the left, then formatting the operator
//! and right side of each Right side.

use crate::formatter::prelude::*;
use crate::syntax::binary_like_expression::{
    AnyMBinaryLikeExpression, AnyMBinaryLikeLeftExpression,
};
use crate::syntax::{MSyntaxKind, MSyntaxNode, MUnaryExpression};
use biome_formatter::{format_args, write, Buffer, CstFormatContext};

use crate::formatter::rules::expressions::static_member_expression::AnyMStaticMemberLike;
use biome_rowan::{AstNode, SyntaxResult};
use std::fmt::Debug;
use std::iter::FusedIterator;

impl Format<MFormatContext> for AnyMBinaryLikeExpression {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let parent = self.syntax().parent();

        let is_inside_condition = self.is_inside_condition(parent.as_ref());
        let parts = split_into_left_and_right_sides(self, is_inside_condition)?;

        // Don't indent inside of conditions because conditions add their own indent and grouping.
        if is_inside_condition {
            return write!(f, [&format_once(|f| { f.join().entries(parts).finish() })]);
        }

        if let Some(parent) = parent.as_ref() {
            // Add a group with a soft block indent in cases where it is necessary to parenthesize the binary expression.
            // For example, `(a+b)(call)`, `!(a + b)`, `(a + b).test`.
            let is_callee = matches!(
                parent.kind(),
                MSyntaxKind::M_CALL_EXPRESSION | MSyntaxKind::M_NEW_EXPRESSION
            );
            if is_callee
                || MUnaryExpression::can_cast(parent.kind())
                || AnyMStaticMemberLike::can_cast(parent.kind())
            {
                return write!(
                    f,
                    [group(&soft_block_indent(&format_once(|f| {
                        f.join().entries(parts).finish()
                    })))]
                );
            }
        }

        let inline_logical_expression = self.should_inline_logical_expression();
        let should_indent_if_inlines = should_indent_if_parent_inlines(parent.as_ref());
        let should_not_indent = self.should_not_indent_if_parent_indents(parent);

        let flattened = parts.len() > 2;

        if should_not_indent
            || (inline_logical_expression && !flattened)
            || (!inline_logical_expression && should_indent_if_inlines)
        {
            return write!(
                f,
                [group(&format_once(|f| {
                    f.join().entries(parts).finish()
                }))]
            );
        }

        if let Some(first) = parts.first() {
            let tail_parts = &parts[1..];

            let group_id = f.group_id("logicalChain");

            let format_parts = format_with(|f| {
                write!(
                    f,
                    [group(&format_args![
                        first,
                        indent(&format_once(|f| {
                            f.join().entries(tail_parts.iter()).finish()
                        }))
                    ])
                    .with_group_id(Some(group_id))]
                )
            });

            write!(f, [format_parts])
        } else {
            // Empty, should never ever happen but let's gracefully recover.
            Ok(())
        }
    }
}

/// Creates a [BinaryLeftOrRightSide::Left] for the first left hand side that:
/// * isn't a [MBinaryLikeExpression]
/// * is a [MBinaryLikeExpression] but it should be formatted as its own group (see [AnyMBinaryLikeExpression::can_flatten]).
///
/// It then traverses upwards from the left most node and creates [BinaryLikeLeftOrRightSide::Right]s for
/// every [MBinaryLikeExpression] until it reaches the root again.
fn split_into_left_and_right_sides(
    root: &AnyMBinaryLikeExpression,
    inside_condition: bool,
) -> SyntaxResult<Vec<BinaryLeftOrRightSide>> {
    // Stores the left and right parts of the binary expression in sequence (rather than nested as they
    // appear in the tree).
    let mut items = Vec::new();

    let mut expressions = BinaryLikePreorder::new(root.clone());

    while let Some(event) = expressions.next() {
        match event {
            VisitEvent::Enter(binary) => {
                if !binary.can_flatten()? {
                    // Stop at this expression. This is either not a binary expression OR it has
                    // different precedence and needs to be grouped separately.
                    // Calling skip_subtree prevents the exit event being triggered for this event.
                    expressions.skip_subtree();

                    items.push(BinaryLeftOrRightSide::Left { parent: binary });
                }
            }
            VisitEvent::Exit(expression) => items.push(BinaryLeftOrRightSide::Right {
                print_parent_comments: expression.syntax() != root.syntax(),
                parent: expression,
                inside_condition,
            }),
        }
    }

    Ok(items)
}

/// There are cases where the parent decides to inline the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(parent: Option<&MSyntaxNode>) -> bool {
    parent.is_some_and(|parent| match parent.kind() {
        MSyntaxKind::M_ASSIGNMENT_EXPRESSION | MSyntaxKind::M_PROPERTY_OBJECT_MEMBER => true,

        MSyntaxKind::M_INITIALIZER_CLAUSE => parent.parent().is_some_and(|grand_parent| {
            matches!(
                grand_parent.kind(),
                MSyntaxKind::M_VARIABLE_DECLARATOR | MSyntaxKind::M_PROPERTY_CLASS_MEMBER
            )
        }),
        _ => false,
    })
}

/// Represents the right or left hand side of a binary expression.
#[derive(Debug, Clone)]
enum BinaryLeftOrRightSide {
    /// A terminal left hand side of a binary expression.
    ///
    /// Formats the left hand side only.
    Left { parent: AnyMBinaryLikeExpression },

    /// The right hand side of a binary expression.
    /// Formats the operand together with the right hand side.
    Right {
        parent: AnyMBinaryLikeExpression,
        /// Is the parent the condition of a `if` / `while` / `do-while` / `for` statement?
        inside_condition: bool,

        /// Indicates if the comments of the parent should be printed or not.
        /// Must be true if `parent` isn't the root `MAnyBinaryLike` for which `format` is called.
        print_parent_comments: bool,
    },
}

impl Format<MFormatContext> for BinaryLeftOrRightSide {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            Self::Left { parent } => {
                write!(f, [group(&parent.left())])
            }
            Self::Right {
                parent: binary_like_expression,
                inside_condition: inside_parenthesis,
                print_parent_comments,
            } => {
                // It's only possible to suppress the formatting of the whole binary expression formatting OR
                // the formatting of the right hand side value but not of a nested binary expression.
                // This aligns with Prettier's behaviour.
                f.context()
                    .comments()
                    .mark_suppression_checked(binary_like_expression.syntax());

                let right = binary_like_expression.right()?;
                let operator_token = binary_like_expression.operator_token()?;

                let operator_and_right_expression = format_with(|f| {
                    let should_inline = binary_like_expression.should_inline_logical_expression();

                    write!(f, [space(), operator_token.format()])?;

                    if should_inline {
                        write!(f, [space()])?;
                    } else {
                        write!(f, [soft_line_break_or_space()])?;
                    }

                    write!(f, [right.format()])?;

                    Ok(())
                });

                let syntax = binary_like_expression.syntax();
                let parent = syntax.parent();

                // Doesn't match prettier that only distinguishes between logical and binary
                let parent_has_same_kind = parent.as_ref().is_some_and(|parent| {
                    is_same_binary_expression_kind(binary_like_expression, parent)
                });

                let left_has_same_kind = binary_like_expression
                    .left()?
                    .into_expression()
                    .is_some_and(|left| {
                        is_same_binary_expression_kind(binary_like_expression, left.syntax())
                    });
                let right_has_same_kind =
                    is_same_binary_expression_kind(binary_like_expression, right.syntax());

                let should_break = f
                    .context()
                    .comments()
                    .trailing_comments(binary_like_expression.left()?.syntax())
                    .iter()
                    .any(|comment| comment.kind().is_line());

                let should_group = !(parent_has_same_kind
                    || left_has_same_kind
                    || right_has_same_kind
                    || (*inside_parenthesis
                        && matches!(
                            binary_like_expression,
                            AnyMBinaryLikeExpression::MLogicalExpression(_)
                        )));

                if *print_parent_comments {
                    write!(
                        f,
                        [format_leading_comments(binary_like_expression.syntax())]
                    )?;
                }

                if should_group {
                    write!(
                        f,
                        [group(&operator_and_right_expression).should_expand(should_break)]
                    )?;
                } else {
                    write!(f, [operator_and_right_expression])?;
                }

                if *print_parent_comments {
                    write!(
                        f,
                        [format_trailing_comments(binary_like_expression.syntax())]
                    )?;
                }

                Ok(())
            }
        }
    }
}

impl Format<MFormatContext> for AnyMBinaryLikeLeftExpression {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            Self::AnyMExpression(expression) => {
                write![f, [expression.format()]]
            }
        }
    }
}

fn is_same_binary_expression_kind(binary: &AnyMBinaryLikeExpression, other: &MSyntaxNode) -> bool {
    match binary {
        AnyMBinaryLikeExpression::MLogicalExpression(_) => {
            matches!(other.kind(), MSyntaxKind::M_LOGICAL_EXPRESSION)
        }
        AnyMBinaryLikeExpression::MBinaryExpression(_)
        | AnyMBinaryLikeExpression::MInExpression(_) => {
            matches!(
                other.kind(),
                MSyntaxKind::M_BINARY_EXPRESSION | MSyntaxKind::M_IN_EXPRESSION
            )
        }
    }
}

/// The [BinaryLikePreorder] visits every node twice. First on the way down to find the left most binary
/// like expression, then on the way back up. This enum encodes the information whatever the
/// iterator is on its way down (`Enter`) or traversing upwards (`Exit`).
#[derive(Debug, Eq, PartialEq, Clone)]
enum VisitEvent {
    Enter(AnyMBinaryLikeExpression),
    Exit(AnyMBinaryLikeExpression),
}

/// Iterator that visits [AnyMBinaryLikeExpression]s in pre-order.
/// This is similar to [MSyntaxNode::descendants] but it only traverses into [AnyMBinaryLikeExpression] and their left side
/// (the right side is never visited).
///
/// # Examples
///
/// ```JavaScript
/// a && b && c && d
/// ```
/// This produces a tree with the following shape:
///
/// ```txt
///         &&
///        / \
///       /   \
///      &&   d && e
///     / \
///    /   \
///   &&    c
///  / \
/// a   b
/// ```
///
/// The iterator emits the following events:
///
/// * Enter(`a && b && c && d && e`)
/// * Enter(`a && b && c`)
/// * Enter(`a && b`)
/// * Exit(`a && b`)
/// * Exit(`a && b && c`)
/// * Exit(`a && b && c && d && e`)
///
/// Notice how the iterator doesn't yield events for the terminal identifiers `a`, `b`, `c`, `d`, and `e`,
/// nor for the right hand side expression `d && e`. This is because the visitor only traverses into
/// [AnyMBinaryLikeExpression]s and of those, only along the left side.
struct BinaryLikePreorder {
    /// The next node to visit or [None] if the iterator passed the start node (is at its end).
    next: Option<VisitEvent>,

    /// The start node. Necessary to know when to stop iterating.
    start: MSyntaxNode,

    skip_subtree: bool,
}

impl BinaryLikePreorder {
    fn new(start: AnyMBinaryLikeExpression) -> Self {
        Self {
            start: start.syntax().clone(),
            next: Some(VisitEvent::Enter(start)),
            skip_subtree: false,
        }
    }

    fn skip_subtree(&mut self) {
        self.next = self.next.take().and_then(|next| match next {
            VisitEvent::Enter(binary) => {
                if binary.syntax() == &self.start {
                    None
                } else {
                    // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                    // if it is a valid binary like expression and it is guaranteed to have a parent.
                    let expression = binary
                        .syntax()
                        .parent()
                        .and_then(AnyMBinaryLikeExpression::cast)
                        .unwrap();

                    Some(VisitEvent::Exit(expression))
                }
            }
            VisitEvent::Exit(node) => Some(VisitEvent::Exit(node)),
        });
        self.skip_subtree = false;
    }
}

impl Iterator for BinaryLikePreorder {
    type Item = VisitEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_subtree {
            self.skip_subtree();
        }

        let next = self.next.take()?;
        match &next {
            VisitEvent::Enter(binary) => {
                let next = binary
                    .left()
                    .ok()
                    .and_then(|left| left.into_expression())
                    .and_then(|expression| {
                        AnyMBinaryLikeExpression::cast(expression.into_syntax())
                    });

                if let Some(binary) = next {
                    self.next = Some(VisitEvent::Enter(binary));
                } else {
                    // If left is missing or it isn't a binary like expression, then format it as part of the parent binary like expression
                    self.next = Some(VisitEvent::Exit(binary.clone()));
                }
            }
            VisitEvent::Exit(node) => {
                if node.syntax() != &self.start {
                    self.next = node.syntax().parent().map(|parent| {
                        // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                        // if it is a valid binary like expression.
                        let expression = AnyMBinaryLikeExpression::cast(parent).unwrap();
                        VisitEvent::Exit(expression)
                    });
                }
            }
        };

        Some(next)
    }
}

impl FusedIterator for BinaryLikePreorder {}
