use crate::formatter::prelude::*;

use crate::formatter::utils::MObjectLike;
use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MHashMapExpression;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMHashMapExpression;
impl_format_with_rule!(MHashMapExpression, FormatMHashMapExpression);

impl FormatNodeRule<MHashMapExpression> for FormatMHashMapExpression {
    fn fmt_fields(&self, node: &MHashMapExpression, f: &mut MFormatter) -> FormatResult<()> {
        write!(f, [MObjectLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &MHashMapExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &MHashMapExpression,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
