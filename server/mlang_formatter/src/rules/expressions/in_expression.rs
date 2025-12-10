use crate::prelude::*;

use mlang_syntax::MInExpression;
use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMInExpression;
impl_format_with_rule!(MInExpression, FormatMInExpression);

impl FormatNodeRule<MInExpression> for FormatMInExpression {
    fn fmt_fields(&self, node: &MInExpression, formatter: &mut MFormatter) -> FormatResult<()> {
        AnyMBinaryLikeExpression::MInExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &MInExpression) -> bool {
        item.needs_parentheses()
    }
}
