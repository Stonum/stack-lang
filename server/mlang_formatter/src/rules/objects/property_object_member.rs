use crate::prelude::*;
use crate::utils::AnyMAssignmentLike;

use biome_formatter::write;
use mlang_syntax::MPropertyObjectMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMPropertyObjectMember;
impl_format_with_rule!(MPropertyObjectMember, FormatMPropertyObjectMember);

impl FormatNodeRule<MPropertyObjectMember> for FormatMPropertyObjectMember {
    fn fmt_fields(&self, node: &MPropertyObjectMember, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }
}
