use crate::prelude::*;

use biome_formatter::{CstFormatContext, format_args, write};
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{AnyMExpression, MParenthesizedExpression, MParenthesizedExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMParenthesizedExpression;
impl_format_with_rule!(MParenthesizedExpression, FormatMParenthesizedExpression);

impl FormatNodeRule<MParenthesizedExpression> for FormatMParenthesizedExpression {
    fn fmt_fields(&self, node: &MParenthesizedExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let expression = expression?;
        let comments = f.context().comments();

        let should_hug = !comments.has_comments(expression.syntax())
            && (matches!(
                expression,
                AnyMExpression::MObjectExpression(_) | AnyMExpression::MArrayExpression(_)
            ));

        if should_hug {
            write!(
                f,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&expression.format()),
                    r_paren_token.format()
                ])]
            )
        }
    }

    fn needs_parentheses(&self, item: &MParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}
