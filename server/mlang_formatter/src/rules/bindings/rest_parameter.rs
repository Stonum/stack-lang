use crate::prelude::*;

use mlang_syntax::MRestParameter;
use mlang_syntax::MRestParameterFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMRestParameter;
impl_format_with_rule!(MRestParameter, FormatMRestParameter);

impl FormatNodeRule<MRestParameter> for FormatMRestParameter {
    fn fmt_fields(&self, node: &MRestParameter, f: &mut MFormatter) -> FormatResult<()> {
        let MRestParameterFields {
            dotdotdot_token,
            binding,
        } = node.as_fields();

        write![f, [dotdotdot_token.format(), binding.format(),]]
    }
}
