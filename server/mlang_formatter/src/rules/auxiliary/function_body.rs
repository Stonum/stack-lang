use crate::prelude::*;

use mlang_syntax::MFunctionBody;
use mlang_syntax::MFunctionBodyFields;
use biome_formatter::{format_args, write, CstFormatContext};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMFunctionBody;
impl_format_with_rule!(MFunctionBody, FormatMFunctionBody);

impl FormatNodeRule<MFunctionBody> for FormatMFunctionBody {
    fn fmt_fields(&self, node: &MFunctionBody, f: &mut MFormatter) -> FormatResult<()> {
        let MFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        let r_curly_token = r_curly_token?;

        if statements.is_empty() && directives.is_empty() {
            let comments = f.context().comments();
            let has_dangling_comments = comments.has_dangling_comments(node.syntax());
            if has_dangling_comments {
                write!(f, [hard_line_break()])?;
            } else {
                write!(f, [space()])?;
            }
            write!(
                f,
                [
                    l_curly_token.format(),
                    format_dangling_comments(node.syntax()).with_block_indent(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    hard_line_break(),
                    l_curly_token.format(),
                    block_indent(&format_args![directives.format(), statements.format()]),
                    r_curly_token.format(),
                ]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &MFunctionBody, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
