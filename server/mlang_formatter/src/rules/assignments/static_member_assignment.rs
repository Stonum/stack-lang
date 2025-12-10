use crate::prelude::*;
use crate::rules::expressions::static_member_expression::AnyMStaticMemberLike;

use mlang_syntax::MStaticMemberAssignment;
use mlang_syntax::parentheses::NeedsParentheses;

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
