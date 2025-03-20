use crate::formatter::prelude::*;
use crate::syntax::MConditionalExpression;

use crate::syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMConditionalExpression;
impl_rule!(MConditionalExpression, FormatMConditionalExpression);

impl FormatNodeRule<MConditionalExpression> for FormatMConditionalExpression {
    fn fmt_fields(
        &self,
        node: &MConditionalExpression,
        formatter: &mut MFormatter,
    ) -> FormatResult<()> {
        MConditionalExpression::from(node.clone())
            .format()
            .fmt(formatter)
    }

    fn needs_parentheses(&self, item: &MConditionalExpression) -> bool {
        item.needs_parentheses()
    }
}
