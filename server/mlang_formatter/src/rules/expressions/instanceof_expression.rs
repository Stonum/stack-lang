use crate::prelude::*;

use mlang_syntax::MInstanceofExpression;
use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::parentheses::NeedsParentheses;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMInstanceofExpression;
impl_format_with_rule!(MInstanceofExpression, FormatMInstanceofExpression);

impl FormatNodeRule<MInstanceofExpression> for FormatMInstanceofExpression {
    fn fmt_fields(&self, node: &MInstanceofExpression, f: &mut MFormatter) -> FormatResult<()> {
        AnyMBinaryLikeExpression::MInstanceofExpression(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MInstanceofExpression) -> bool {
        item.needs_parentheses()
    }
}
