use crate::formatter::prelude::*;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MIdentifierExpression;
use crate::syntax::MIdentifierExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMIdentifierExpression;
impl_format_with_rule!(MIdentifierExpression, FormatMIdentifierExpression);

impl FormatNodeRule<MIdentifierExpression> for FormatMIdentifierExpression {
    fn fmt_fields(&self, node: &MIdentifierExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MIdentifierExpressionFields { name } = node.as_fields();

        write![f, [name.format()]]
    }

    fn needs_parentheses(&self, item: &MIdentifierExpression) -> bool {
        item.needs_parentheses()
    }
}
