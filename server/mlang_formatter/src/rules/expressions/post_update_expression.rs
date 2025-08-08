use crate::prelude::*;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::MPostUpdateExpression;
use mlang_syntax::MPostUpdateExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMPostUpdateExpression;
impl_format_with_rule!(MPostUpdateExpression, FormatMPostUpdateExpression);

impl FormatNodeRule<MPostUpdateExpression> for FormatMPostUpdateExpression {
    fn fmt_fields(&self, node: &MPostUpdateExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MPostUpdateExpressionFields {
            operand,
            operator_token,
        } = node.as_fields();

        write![f, [operand.format(), operator_token.format()]]
    }

    fn needs_parentheses(&self, item: &MPostUpdateExpression) -> bool {
        item.needs_parentheses()
    }
}
