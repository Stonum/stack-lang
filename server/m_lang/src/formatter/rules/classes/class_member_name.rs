use crate::formatter::prelude::*;
use crate::syntax::{MClassMemberName, MClassMemberNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMClassMemberName;
impl_format_with_rule!(MClassMemberName, FormatMClassMemberName);
// impl_format!(MClassMemberName, FormatMClassMemberName);

impl FormatNodeRule<MClassMemberName> for FormatMClassMemberName {
    fn fmt_fields(&self, node: &MClassMemberName, f: &mut MFormatter) -> FormatResult<()> {
        let MClassMemberNameFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
