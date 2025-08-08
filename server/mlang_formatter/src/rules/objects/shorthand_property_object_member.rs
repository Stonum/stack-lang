use crate::prelude::*;

use mlang_syntax::MShorthandPropertyObjectMember;
use mlang_syntax::MShorthandPropertyObjectMemberFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMShorthandPropertyObjectMember;
impl_format_with_rule!(
    MShorthandPropertyObjectMember,
    FormatMShorthandPropertyObjectMember
);

impl FormatNodeRule<MShorthandPropertyObjectMember> for FormatMShorthandPropertyObjectMember {
    fn fmt_fields(
        &self,
        node: &MShorthandPropertyObjectMember,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MShorthandPropertyObjectMemberFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
