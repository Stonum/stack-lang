use crate::prelude::*;

use crate::utils::FormatStatementBody;
use biome_formatter::{format_args, write};
use mlang_syntax::MForAllStatement;
use mlang_syntax::MForAllStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForAllStatement;
impl_format_with_rule!(MForAllStatement, FormatMForAllStatement);

impl FormatNodeRule<MForAllStatement> for FormatMForAllStatement {
    fn fmt_fields(&self, node: &MForAllStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MForAllStatementFields {
            forall_token,
            l_paren_token,
            iter,
            r_paren_token,
            body,
        } = node.as_fields();

        let forall_token = forall_token.format();
        let iter = iter.format();

        write!(
            f,
            [group(&format_args!(
                forall_token,
                l_paren_token.format(),
                iter,
                r_paren_token.format(),
                space(),
                format_dangling_comments(node.syntax()),
                FormatStatementBody::new(&body?)
            ))]
        )
    }

    fn fmt_dangling_comments(&self, _: &MForAllStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
