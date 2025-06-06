use crate::formatter::prelude::*;
use biome_formatter::{format_args, write, CstFormatContext};

use crate::formatter::utils::FormatStatementBody;
use crate::syntax::MForStatement;
use crate::syntax::MForStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForStatement;
impl_format_with_rule!(MForStatement, FormatMForStatement);

impl FormatNodeRule<MForStatement> for FormatMForStatement {
    fn fmt_fields(&self, node: &MForStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MForStatementFields {
            for_token,
            l_paren_token,
            initializer,
            first_semi_token,
            test,
            second_semi_token,
            update,
            r_paren_token,
            body,
        } = node.as_fields();

        let body = body?;
        let l_paren_token = l_paren_token?;

        let format_body = FormatStatementBody::new(&body);

        // Move dangling trivia between the `for /* this */ (` to the top of the `for` and
        // add a line break after.
        let comments = f.context().comments();
        let dangling_comments = comments.dangling_comments(node.syntax());
        if !dangling_comments.is_empty() {
            write!(
                f,
                [
                    format_dangling_comments(node.syntax()),
                    soft_line_break_or_space()
                ]
            )?;
        }

        if initializer.is_none() && test.is_none() && update.is_none() {
            return write!(
                f,
                [group(&format_args![
                    for_token.format(),
                    l_paren_token.format(),
                    first_semi_token.format(),
                    second_semi_token.format(),
                    r_paren_token.format(),
                    format_body
                ])]
            );
        }

        let format_inner = format_with(|f| {
            write!(
                f,
                [
                    for_token.format(),
                    l_paren_token.format(),
                    group(&soft_block_indent(&format_args![
                        initializer.format(),
                        first_semi_token.format(),
                        soft_line_break_or_space(),
                        test.format(),
                        second_semi_token.format(),
                        soft_line_break_or_space(),
                        update.format()
                    ])),
                    r_paren_token.format(),
                    format_body
                ]
            )
        });

        write!(f, [group(&format_inner)])
    }

    fn fmt_dangling_comments(&self, _: &MForStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
