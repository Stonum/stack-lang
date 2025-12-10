use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_rowan::AstNode;
use mlang_syntax::MSyntaxKind::M_SEQUENCE_EXPRESSION;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{MSequenceExpression, MSequenceExpressionFields, MSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSequenceExpression;
impl_format_with_rule!(MSequenceExpression, FormatMSequenceExpression);

impl FormatNodeRule<MSequenceExpression> for FormatMSequenceExpression {
    fn fmt_fields(&self, node: &MSequenceExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = node.as_fields();

        let mut is_nested = false;
        let mut first_non_sequence_or_paren_parent = None;

        // Skip 1 because ancestor starts with the current node but we're interested in the parent
        for parent in node.syntax().ancestors().skip(1) {
            if parent.kind() == M_SEQUENCE_EXPRESSION {
                is_nested = true;
            } else {
                first_non_sequence_or_paren_parent = Some(parent);
                break;
            }
        }

        let format_inner = format_with(|f| {
            if let Some(parent) = &first_non_sequence_or_paren_parent
                && matches!(
                    parent.kind(),
                    MSyntaxKind::M_EXPRESSION_STATEMENT | MSyntaxKind::M_FOR_STATEMENT
                )
            {
                return write!(
                    f,
                    [
                        left.format(),
                        comma_token.format(),
                        line_suffix_boundary(),
                        indent(&format_args![soft_line_break_or_space(), right.format()])
                    ]
                );
            }

            write!(
                f,
                [
                    left.format(),
                    comma_token.format(),
                    line_suffix_boundary(),
                    soft_line_break_or_space(),
                    right.format()
                ]
            )
        });

        if is_nested {
            write!(f, [format_inner])
        } else {
            write!(f, [group(&format_inner)])
        }
    }

    fn needs_parentheses(&self, item: &MSequenceExpression) -> bool {
        item.needs_parentheses()
    }
}
