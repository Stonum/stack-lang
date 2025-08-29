use crate::prelude::*;
use biome_formatter::{format_args, write};

use crate::utils::FormatStatementBody;
use mlang_syntax::MIfStatement;
use mlang_syntax::MIfStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMIfStatement;
impl_format_with_rule!(MIfStatement, FormatMIfStatement);

impl FormatNodeRule<MIfStatement> for FormatMIfStatement {
    fn fmt_fields(&self, node: &MIfStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;
        let consequent = consequent?;

        write!(
            f,
            [group(&format_args![
                if_token.format(),
                l_paren_token.format(),
                space(),
                group(&soft_block_indent(&test.format())),
                space(),
                r_paren_token.format(),
                space(),
                format_dangling_comments(node.syntax()),
                FormatStatementBody::new(&consequent),
            ]),]
        )?;

        if let Some(else_clause) = else_clause {
            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(&self, _: &MIfStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
