use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MComputedMemberName;
use mlang_syntax::MComputedMemberNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMComputedMemberName;
impl_format_with_rule!(MComputedMemberName, FormatMComputedMemberName);

impl FormatNodeRule<MComputedMemberName> for FormatMComputedMemberName {
    fn fmt_fields(&self, node: &MComputedMemberName, f: &mut MFormatter) -> FormatResult<()> {
        let MComputedMemberNameFields {
            l_brack_token,
            expression,
            r_brack_token,
        } = node.as_fields();

        write![
            f,
            [
                l_brack_token.format(),
                expression.format(),
                r_brack_token.format(),
            ]
        ]
    }
}
