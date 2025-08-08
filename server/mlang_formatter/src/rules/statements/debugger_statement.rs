use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use mlang_syntax::MDebugStatement;
use mlang_syntax::MDebugStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMDebugStatement;
impl_format_with_rule!(MDebugStatement, FormatMDebugStatement);

impl FormatNodeRule<MDebugStatement> for FormatMDebugStatement {
    fn fmt_fields(&self, node: &MDebugStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MDebugStatementFields {
            debug_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [debug_token.format(),])?;

        if f.comments().has_dangling_comments(node.syntax()) {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        FormatStatementSemicolon::new(semicolon_token.as_ref()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &MDebugStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Handled in `fmt_fields`
        Ok(())
    }
}
