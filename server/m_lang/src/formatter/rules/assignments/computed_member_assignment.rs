use crate::formatter::prelude::*;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::{AnyMComputedMember, MComputedMemberAssignment};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMComputedMemberAssignment;
impl_format_with_rule!(MComputedMemberAssignment, FormatMComputedMemberAssignment);

impl FormatNodeRule<MComputedMemberAssignment> for FormatMComputedMemberAssignment {
    fn fmt_fields(&self, node: &MComputedMemberAssignment, f: &mut MFormatter) -> FormatResult<()> {
        AnyMComputedMember::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MComputedMemberAssignment) -> bool {
        item.needs_parentheses()
    }
}
