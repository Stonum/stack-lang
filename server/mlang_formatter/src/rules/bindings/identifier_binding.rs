use crate::prelude::*;

use mlang_syntax::MIdentifierBinding;
use mlang_syntax::MIdentifierBindingFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMIdentifierBinding;
impl_format_with_rule!(MIdentifierBinding, FormatMIdentifierBinding);

impl FormatNodeRule<MIdentifierBinding> for FormatMIdentifierBinding {
    fn fmt_fields(&self, node: &MIdentifierBinding, f: &mut MFormatter) -> FormatResult<()> {
        let MIdentifierBindingFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }
}
