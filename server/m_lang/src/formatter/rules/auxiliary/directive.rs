use crate::formatter::prelude::*;
use biome_formatter::token::number::format_number_token;
use biome_formatter::write;

use crate::syntax::MDirective;
use crate::syntax::MDirectiveFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMDirective;
impl_format_with_rule!(MDirective, FormatMDirective);

impl FormatNodeRule<MDirective> for FormatMDirective {
    fn fmt_fields(&self, node: &MDirective, f: &mut MFormatter) -> FormatResult<()> {
        let MDirectiveFields {
            version_token,
            value_token,
        } = node.as_fields();

        write!(
            f,
            [
                version_token.format(),
                space(),
                format_number_token(&value_token?)
            ]
        )
    }
}
