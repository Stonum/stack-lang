use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use mlang_syntax::MBreakStatement;
use mlang_syntax::MBreakStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBreakStatement;
impl_format_with_rule!(MBreakStatement, FormatMBreakStatement);

impl FormatNodeRule<MBreakStatement> for FormatMBreakStatement {
    fn fmt_fields(&self, node: &MBreakStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MBreakStatementFields {
            break_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [break_token.format()])?;

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }
}
