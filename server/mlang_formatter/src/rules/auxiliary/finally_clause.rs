use crate::prelude::*;

use mlang_syntax::MFinallyClause;
use mlang_syntax::MFinallyClauseFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMFinallyClause;
impl_format_with_rule!(MFinallyClause, FormatMFinallyClause);

impl FormatNodeRule<MFinallyClause> for FormatMFinallyClause {
    fn fmt_fields(&self, node: &MFinallyClause, f: &mut MFormatter) -> FormatResult<()> {
        let MFinallyClauseFields {
            finally_token,
            body,
        } = node.as_fields();

        write![
            f,
            [finally_token.format(), hard_line_break(), body.format()]
        ]
    }
}
