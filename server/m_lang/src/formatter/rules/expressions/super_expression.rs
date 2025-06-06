use crate::formatter::prelude::*;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MSuperExpression;
use crate::syntax::MSuperExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSuperExpression;
impl_format_with_rule!(MSuperExpression, FormatMSuperExpression);

impl FormatNodeRule<MSuperExpression> for FormatMSuperExpression {
    fn fmt_fields(&self, node: &MSuperExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MSuperExpressionFields { super_token } = node.as_fields();

        write![f, [super_token.format()]]
    }

    fn needs_parentheses(&self, item: &MSuperExpression) -> bool {
        item.needs_parentheses()
    }
}
