use crate::formatter::prelude::*;
use crate::formatter::utils::AnyMAssignmentLike;

use crate::syntax::MPropertyObjectMember;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMPropertyObjectMember;
impl_format_with_rule!(MPropertyObjectMember, FormatMPropertyObjectMember);

impl FormatNodeRule<MPropertyObjectMember> for FormatMPropertyObjectMember {
    fn fmt_fields(&self, node: &MPropertyObjectMember, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }
}
