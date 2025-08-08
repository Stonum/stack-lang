use crate::prelude::*;
use mlang_syntax::MCatchClause;
use mlang_syntax::MCatchClauseFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCatchClause;
impl_format_with_rule!(MCatchClause, FormatMCatchClause);

impl FormatNodeRule<MCatchClause> for FormatMCatchClause {
    fn fmt_fields(&self, node: &MCatchClause, f: &mut MFormatter) -> FormatResult<()> {
        let MCatchClauseFields {
            catch_token,
            declaration,
            body,
        } = node.as_fields();

        write!(f, [catch_token.format()])?;

        if let Some(declaration) = declaration {
            write![f, [declaration.format()]]?;
        }

        write!(f, [hard_line_break(), body.format()])
    }
}
