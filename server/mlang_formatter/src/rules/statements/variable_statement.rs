use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use mlang_syntax::MVariableStatement;
use mlang_syntax::MVariableStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableStatement;
impl_format_with_rule!(MVariableStatement, FormatMVariableStatement);

impl FormatNodeRule<MVariableStatement> for FormatMVariableStatement {
    fn fmt_fields(&self, node: &MVariableStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MVariableStatementFields {
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
