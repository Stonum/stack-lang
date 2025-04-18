use crate::formatter::prelude::*;
use crate::formatter::utils::FormatStatementSemicolon;
use biome_formatter::write;

use crate::syntax::MVariableDeclarationClause;
use crate::syntax::MVariableDeclarationClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableDeclarationClause;
impl_format_with_rule!(MVariableDeclarationClause, FormatMVariableDeclarationClause);

impl FormatNodeRule<MVariableDeclarationClause> for FormatMVariableDeclarationClause {
    fn fmt_fields(
        &self,
        node: &MVariableDeclarationClause,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                declaration.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
