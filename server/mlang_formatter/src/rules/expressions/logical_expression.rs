use crate::prelude::*;

use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::MLogicalExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLogicalExpression;
impl_format_with_rule!(MLogicalExpression, FormatMLogicalExpression);

impl FormatNodeRule<MLogicalExpression> for FormatMLogicalExpression {
    fn fmt_fields(
        &self,
        node: &MLogicalExpression,
        formatter: &mut MFormatter,
    ) -> FormatResult<()> {
        AnyMBinaryLikeExpression::MLogicalExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &MLogicalExpression) -> bool {
        item.needs_parentheses()
    }
}
