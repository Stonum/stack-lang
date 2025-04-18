use crate::formatter::prelude::*;

use crate::formatter::utils::MObjectLike;
use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MObjectExpression;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectExpression;
impl_format_with_rule!(MObjectExpression, FormatMObjectExpression);

impl FormatNodeRule<MObjectExpression> for FormatMObjectExpression {
    fn fmt_fields(&self, node: &MObjectExpression, f: &mut MFormatter) -> FormatResult<()> {
        write!(f, [MObjectLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &MObjectExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(&self, _: &MObjectExpression, _: &mut MFormatter) -> FormatResult<()> {
        Ok(())
    }
}
