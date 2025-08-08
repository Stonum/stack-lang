use crate::prelude::*;

use mlang_syntax::MName;
use mlang_syntax::MNameFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMName;
impl_format_with_rule!(MName, FormatMName);

impl FormatNodeRule<MName> for FormatMName {
    fn fmt_fields(&self, node: &MName, f: &mut MFormatter) -> FormatResult<()> {
        let MNameFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
