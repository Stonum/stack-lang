use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use mlang_syntax::MContinueStatement;
use mlang_syntax::MContinueStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMContinueStatement;
impl_format_with_rule!(MContinueStatement, FormatMContinueStatement);

impl FormatNodeRule<MContinueStatement> for FormatMContinueStatement {
    fn fmt_fields(&self, node: &MContinueStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MContinueStatementFields {
            continue_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [continue_token.format()])?;

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }

    fn fmt_dangling_comments(
        &self,
        node: &MContinueStatement,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        if !f.comments().has_dangling_comments(node.syntax()) {
            return Ok(());
        }
        let content =
            format_with(|f| write!(f, [space(), format_dangling_comments(node.syntax())]));
        write!(f, [line_suffix(&content), expand_parent()])
    }
}
