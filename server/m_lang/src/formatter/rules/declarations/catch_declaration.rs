use crate::formatter::prelude::*;

use crate::syntax::MCatchDeclaration;
use crate::syntax::MCatchDeclarationFields;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCatchDeclaration;
impl_format_with_rule!(MCatchDeclaration, FormatMCatchDeclaration);

impl FormatNodeRule<MCatchDeclaration> for FormatMCatchDeclaration {
    fn fmt_fields(&self, node: &MCatchDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        let MCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
        } = node.as_fields();

        let binding = binding?;

        let leading_comment_with_break = f
            .comments()
            .leading_comments(binding.syntax())
            .iter()
            .any(|comment| comment.lines_after() > 0 || comment.kind().is_line());

        let last_parameter_node = binding.syntax();

        let trailing_comment_with_break = f
            .comments()
            .trailing_comments(last_parameter_node)
            .iter()
            .any(|comment| comment.lines_before() > 0 || comment.kind().is_line());

        if leading_comment_with_break || trailing_comment_with_break {
            write!(
                f,
                [
                    l_paren_token.format(),
                    soft_block_indent(&format_args![binding.format()]),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    l_paren_token.format(),
                    binding.format(),
                    r_paren_token.format()
                ]
            )
        }
    }
}
