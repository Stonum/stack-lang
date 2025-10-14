use crate::prelude::*;

use biome_formatter::{format_args, write};
use mlang_syntax::MUnaryExpression;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{MUnaryExpressionFields, MUnaryOperator};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMUnaryExpression;
impl_format_with_rule!(MUnaryExpression, FormatMUnaryExpression);

impl FormatNodeRule<MUnaryExpression> for FormatMUnaryExpression {
    fn fmt_fields(&self, node: &MUnaryExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MUnaryExpressionFields {
            operator_token,
            argument,
        } = node.as_fields();

        let operation = node.operator()?;
        let operator_token = operator_token?;
        let argument = argument?;

        write!(f, [operator_token.format()])?;

        let is_keyword_operator =
            matches!(operation, MUnaryOperator::Delete | MUnaryOperator::Classof);

        if is_keyword_operator {
            write!(f, [space()])?;
        }

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            write!(
                f,
                [group(&format_args![
                    text("("),
                    soft_block_indent(&argument.format()),
                    text(")")
                ])]
            )
        } else {
            write![f, [argument.format()]]
        }
    }

    fn needs_parentheses(&self, item: &MUnaryExpression) -> bool {
        item.needs_parentheses()
    }
}
