use crate::formatter::prelude::*;

use super::return_statement::AnyMStatementWithArgument;
use crate::syntax::MThrowStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMThrowStatement;
impl_format_with_rule!(MThrowStatement, FormatMThrowStatement);

impl FormatNodeRule<MThrowStatement> for FormatMThrowStatement {
    fn fmt_fields(&self, node: &MThrowStatement, f: &mut MFormatter) -> FormatResult<()> {
        AnyMStatementWithArgument::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &MThrowStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `MAnyStatementWithArgument`
        Ok(())
    }
}
