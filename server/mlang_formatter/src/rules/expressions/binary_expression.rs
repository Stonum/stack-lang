use crate::prelude::*;

use mlang_syntax::MBinaryExpression;
use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBinaryExpression;
impl_format_with_rule!(MBinaryExpression, FormatMBinaryExpression);

impl FormatNodeRule<MBinaryExpression> for FormatMBinaryExpression {
    fn fmt_fields(&self, node: &MBinaryExpression, formatter: &mut MFormatter) -> FormatResult<()> {
        AnyMBinaryLikeExpression::MBinaryExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &MBinaryExpression) -> bool {
        item.needs_parentheses()
    }
}
