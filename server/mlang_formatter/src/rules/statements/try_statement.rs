use crate::prelude::*;

use mlang_syntax::MTryStatement;
use mlang_syntax::MTryStatementFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMTryStatement;
impl_format_with_rule!(MTryStatement, FormatMTryStatement);

impl FormatNodeRule<MTryStatement> for FormatMTryStatement {
    fn fmt_fields(&self, node: &MTryStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = node.as_fields();

        write![
            f,
            [
                try_token.format(),
                hard_line_break(),
                body.format(),
                hard_line_break(),
                catch_clause.format(),
            ]
        ]
    }
}
