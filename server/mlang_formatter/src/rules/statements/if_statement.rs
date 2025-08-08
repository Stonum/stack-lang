use crate::prelude::*;
use biome_formatter::{CstFormatContext, format_args, write};

use crate::utils::FormatStatementBody;
use mlang_syntax::MIfStatement;
use mlang_syntax::MIfStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMIfStatement;
impl_format_with_rule!(MIfStatement, FormatMIfStatement);

impl FormatNodeRule<MIfStatement> for FormatMIfStatement {
    fn fmt_fields(&self, node: &MIfStatement, f: &mut MFormatter) -> FormatResult<()> {
        use mlang_syntax::AnyMStatement::*;

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
                FormatStatementBody::new(&consequent),
            ]),]
        )?;

        if let Some(else_clause) = else_clause {
            let comments = f.context().comments();
            let dangling_comments = comments.dangling_comments(node.syntax());
            let dangling_line_comment = dangling_comments
                .last()
                .is_some_and(|comment| comment.kind().is_line());
            let has_dangling_comments = !dangling_comments.is_empty();

            let trailing_line_comment = comments
                .trailing_comments(consequent.syntax())
                .iter()
                .any(|comment| comment.kind().is_line());

            let else_on_same_line = matches!(consequent, MBlockStatement(_))
                && !trailing_line_comment
                && !dangling_line_comment;

            if else_on_same_line {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            if has_dangling_comments {
                write!(f, [format_dangling_comments(node.syntax())])?;

                if trailing_line_comment || dangling_line_comment {
                    write!(f, [hard_line_break()])?
                } else {
                    write!(f, [space()])?;
                }
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(&self, _: &MIfStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
