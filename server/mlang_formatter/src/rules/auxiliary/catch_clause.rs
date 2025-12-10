use crate::prelude::*;
use biome_formatter::write;
use mlang_syntax::MCatchClause;
use mlang_syntax::MCatchClauseFields;

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

        if let Ok(block) = &body
            && block.statements().is_empty() {
                return write!(f, [space(), body.format()]);
            }
        write!(f, [hard_line_break(), body.format()])
    }
}
