use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MExpressionSnipped;
use mlang_syntax::MExpressionSnippedFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMExpressionSnipped;
impl_format_with_rule!(MExpressionSnipped, FormatMExpressionSnipped);

impl FormatNodeRule<MExpressionSnipped> for FormatMExpressionSnipped {
    fn fmt_fields(&self, node: &MExpressionSnipped, f: &mut MFormatter) -> FormatResult<()> {
        let MExpressionSnippedFields {
            expression,
            eof_token,
        } = node.as_fields();

        write![f, [expression.format(), format_removed(&eof_token?),]]
    }
}
