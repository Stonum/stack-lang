use crate::formatter::prelude::*;

use crate::formatter::utils::FormatStatementBody;
use crate::syntax::MElseClause;
use crate::syntax::MElseClauseFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMElseClause;
impl_format_with_rule!(MElseClause, FormatMElseClause);

impl FormatNodeRule<MElseClause> for FormatMElseClause {
    fn fmt_fields(&self, node: &MElseClause, f: &mut MFormatter) -> FormatResult<()> {
        use crate::syntax::AnyMStatement::*;

        let MElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        let alternate = alternate?;

        write!(
            f,
            [
                else_token.format(),
                group(
                    &FormatStatementBody::new(&alternate)
                        .with_forced_space(matches!(alternate, MIfStatement(_)))
                )
            ]
        )
    }
}
