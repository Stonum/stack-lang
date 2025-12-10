use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MBooleanLiteralExpression;
use mlang_syntax::MBooleanLiteralExpressionFields;
use mlang_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBooleanLiteralExpression;
impl_format_with_rule!(MBooleanLiteralExpression, FormatMBooleanLiteralExpression);

impl FormatNodeRule<MBooleanLiteralExpression> for FormatMBooleanLiteralExpression {
    fn fmt_fields(&self, node: &MBooleanLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MBooleanLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &MBooleanLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
