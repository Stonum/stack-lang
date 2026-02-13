use crate::prelude::*;

use crate::utils::FormatStatementBody;
use biome_formatter::{format_args, write};
use mlang_syntax::MForAllInStatement;
use mlang_syntax::MForAllInStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForAllInStatement;
impl_format_with_rule!(MForAllInStatement, FormatMForAllInStatement);

impl FormatNodeRule<MForAllInStatement> for FormatMForAllInStatement {
    fn fmt_fields(&self, node: &MForAllInStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MForAllInStatementFields {
            forall_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let forall_token = forall_token.format();
        let initializer = initializer.format();
        let in_token = in_token.format();
        let expression = expression.format();

        write!(
            f,
            [group(&format_args!(
                forall_token,
                l_paren_token.format(),
                initializer,
                space(),
                in_token,
                space(),
                expression,
                r_paren_token.format(),
                space(),
                format_dangling_comments(node.syntax()),
                FormatStatementBody::new(&body?)
            ))]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &MForAllInStatement,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
