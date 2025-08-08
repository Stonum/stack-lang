use crate::prelude::*;

use crate::utils::FormatStatementBody;
use mlang_syntax::MWhileStatement;
use mlang_syntax::MWhileStatementFields;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMWhileStatement;
impl_format_with_rule!(MWhileStatement, FormatMWhileStatement);

impl FormatNodeRule<MWhileStatement> for FormatMWhileStatement {
    fn fmt_fields(&self, node: &MWhileStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                while_token.format(),
                l_paren_token.format(),
                space(),
                group(&soft_block_indent(&test.format())),
                space(),
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ])]
        )
    }
}
