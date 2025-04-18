use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::syntax::MTryFinallyStatement;
use crate::syntax::MTryFinallyStatementFields;

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

        write![f, [try_token.format(), space(), body.format()]]?;

        if let Some(catch_clause) = catch_clause {
            write!(f, [space(), catch_clause.format()])?;
        }

        write!(f, [space(), finally_clause.format()])
    }
}
