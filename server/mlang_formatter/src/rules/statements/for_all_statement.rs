use crate::prelude::*;

use crate::utils::FormatStatementBody;
use mlang_syntax::MForAllStatement;
use mlang_syntax::MForAllStatementFields;
use biome_formatter::{format_args, write};

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
                space(),
                iter,
                space(),
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ))]
        )
    }
}
