use crate::formatter::prelude::*;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MThisExpression;
use crate::syntax::MThisExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMThisExpression;
impl_format_with_rule!(MThisExpression, FormatMThisExpression);

impl FormatNodeRule<MThisExpression> for FormatMThisExpression {
    fn fmt_fields(&self, node: &MThisExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MThisExpressionFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }

    fn needs_parentheses(&self, item: &MThisExpression) -> bool {
        item.needs_parentheses()
    }
}
