use crate::prelude::*;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::MPreUpdateExpression;
use mlang_syntax::MPreUpdateExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMPreUpdateExpression;
impl_format_with_rule!(MPreUpdateExpression, FormatMPreUpdateExpression);

impl FormatNodeRule<MPreUpdateExpression> for FormatMPreUpdateExpression {
    fn fmt_fields(&self, node: &MPreUpdateExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MPreUpdateExpressionFields {
            operator_token,
            operand,
        } = node.as_fields();

        write![f, [operator_token.format(), operand.format(),]]
    }

    fn needs_parentheses(&self, item: &MPreUpdateExpression) -> bool {
        item.needs_parentheses()
    }
}
