use crate::formatter::prelude::*;
use crate::formatter::rules::expressions::static_member_expression::AnyMStaticMemberLike;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MStaticMemberAssignment;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStaticMemberAssignment;
impl_format_with_rule!(MStaticMemberAssignment, FormatMStaticMemberAssignment);

impl FormatNodeRule<MStaticMemberAssignment> for FormatMStaticMemberAssignment {
    fn fmt_fields(&self, node: &MStaticMemberAssignment, f: &mut MFormatter) -> FormatResult<()> {
        AnyMStaticMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MStaticMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}
