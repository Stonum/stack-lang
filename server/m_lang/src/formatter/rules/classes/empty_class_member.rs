use crate::formatter::prelude::*;

use crate::syntax::MEmptyClassMember;
use crate::syntax::MEmptyClassMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMEmptyClassMember;
impl_format_with_rule!(MEmptyClassMember, FormatMEmptyClassMember);

impl FormatNodeRule<MEmptyClassMember> for FormatMEmptyClassMember {
    fn fmt_fields(&self, node: &MEmptyClassMember, f: &mut MFormatter) -> FormatResult<()> {
        let MEmptyClassMemberFields { semicolon_token } = node.as_fields();

        format_removed(&semicolon_token?).fmt(f)
    }
}
