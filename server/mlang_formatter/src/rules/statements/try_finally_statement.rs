use crate::prelude::*;
use biome_formatter::write;

use mlang_syntax::MTryFinallyStatement;
use mlang_syntax::MTryFinallyStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMTryFinallyStatement;
impl_format_with_rule!(MTryFinallyStatement, FormatMTryFinallyStatement);

impl FormatNodeRule<MTryFinallyStatement> for FormatMTryFinallyStatement {
    fn fmt_fields(&self, node: &MTryFinallyStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = node.as_fields();

        write![f, [try_token.format(), hard_line_break(), body.format()]]?;

        if let Some(catch_clause) = catch_clause {
            write!(f, [hard_line_break(), catch_clause.format()])?;
        }

        write!(f, [hard_line_break(), finally_clause.format()])
    }
}
