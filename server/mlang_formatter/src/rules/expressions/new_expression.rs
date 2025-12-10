use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MNewExpression;
use mlang_syntax::MNewExpressionFields;
use mlang_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMNewExpression;
impl_format_with_rule!(MNewExpression, FormatMNewExpression);

impl FormatNodeRule<MNewExpression> for FormatMNewExpression {
    fn fmt_fields(&self, node: &MNewExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MNewExpressionFields {
            new_token,
            callee,
            arguments,
        } = node.as_fields();

        write![f, [new_token.format(), space(), callee.format(),]]?;

        match arguments {
            Some(arguments) => {
                write!(f, [arguments.format()])
            }
            None => {
                write!(f, [text("("), text(")")])
            }
        }
    }

    fn needs_parentheses(&self, item: &MNewExpression) -> bool {
        item.needs_parentheses()
    }
}
