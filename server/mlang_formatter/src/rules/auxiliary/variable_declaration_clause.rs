use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;
use biome_formatter::write;

use mlang_syntax::MVariableDeclarationClause;
use mlang_syntax::MVariableDeclarationClauseFields;

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
