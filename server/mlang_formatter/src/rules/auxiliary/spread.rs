use crate::prelude::*;

use mlang_syntax::MSpread;
use mlang_syntax::MSpreadFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSpread;
impl_format_with_rule!(MSpread, FormatMSpread);

impl FormatNodeRule<MSpread> for FormatMSpread {
    fn fmt_fields(&self, node: &MSpread, f: &mut MFormatter) -> FormatResult<()> {
        let MSpreadFields {
            dotdotdot_token,
            argument,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), argument.format()]]
    }
}
