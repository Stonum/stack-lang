use crate::prelude::*;

use crate::utils::FormatStatementBody;
use biome_formatter::write;
use mlang_syntax::MElseClause;
use mlang_syntax::MElseClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMElseClause;
impl_format_with_rule!(MElseClause, FormatMElseClause);

impl FormatNodeRule<MElseClause> for FormatMElseClause {
    fn fmt_fields(&self, node: &MElseClause, f: &mut MFormatter) -> FormatResult<()> {
        let MElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        let alternate = alternate?;

        write!(
            f,
            [
                hard_line_break(),
                else_token.format(),
                space(),
                format_dangling_comments(node.syntax()),
                group(&FormatStatementBody::new(&alternate))
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &MElseClause, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
