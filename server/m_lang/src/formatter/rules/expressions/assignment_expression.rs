use crate::formatter::prelude::*;
use crate::formatter::utils::AnyMAssignmentLike;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MAssignmentExpression;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAssignmentExpression;
impl_format_with_rule!(MAssignmentExpression, FormatMAssignmentExpression);

impl FormatNodeRule<MAssignmentExpression> for FormatMAssignmentExpression {
    fn fmt_fields(&self, node: &MAssignmentExpression, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }

    fn needs_parentheses(&self, item: &MAssignmentExpression) -> bool {
        item.needs_parentheses()
    }
}
