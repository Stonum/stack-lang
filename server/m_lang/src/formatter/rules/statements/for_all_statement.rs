use crate::formatter::prelude::*;

use crate::formatter::utils::FormatStatementBody;
use crate::syntax::MForAllStatement;
use crate::syntax::MForAllStatementFields;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForAllStatement;
impl_format_with_rule!(MForAllStatement, FormatMForAllStatement);

impl FormatNodeRule<MForAllStatement> for FormatMForAllStatement {
    fn fmt_fields(&self, node: &MForAllStatement, f: &mut MFormatter) -> FormatResult<()> {
        todo!("implement me")
        // let MForAllInStatementFields {
        //     forall_token,
        //     l_paren_token,
        //     initializer,
        //     in_token,
        //     expression,
        //     r_paren_token,
        //     body,
        // } = node.as_fields();

        // let forall_token = forall_token.format();
        // let initializer = initializer.format();
        // let in_token = in_token.format();
        // let expression = expression.format();

        // write!(
        //     f,
        //     [group(&format_args!(
        //         forall_token,
        //         space(),
        //         l_paren_token.format(),
        //         initializer,
        //         space(),
        //         in_token,
        //         space(),
        //         expression,
        //         r_paren_token.format(),
        //         FormatStatementBody::new(&body?)
        //     ))]
        // )
    }
}
