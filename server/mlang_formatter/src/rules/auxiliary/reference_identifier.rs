use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MReferenceIdentifier;
use mlang_syntax::MReferenceIdentifierFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReferenceIdentifier;
impl_format_with_rule!(MReferenceIdentifier, FormatMReferenceIdentifier);

impl FormatNodeRule<MReferenceIdentifier> for FormatMReferenceIdentifier {
    fn fmt_fields(&self, node: &MReferenceIdentifier, f: &mut MFormatter) -> FormatResult<()> {
        let MReferenceIdentifierFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }
}
