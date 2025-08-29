use crate::prelude::*;
use biome_formatter::{CstFormatContext, write};

use biome_rowan::{AstNode, SyntaxNodeOptionExt};
use mlang_syntax::{MEmptyStatement, MEmptyStatementFields, MSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMEmptyStatement;
impl_format_with_rule!(MEmptyStatement, FormatMEmptyStatement);

impl FormatNodeRule<MEmptyStatement> for FormatMEmptyStatement {
    fn fmt_fields(&self, node: &MEmptyStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MEmptyStatementFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().kind();

        let leading_comments_with_break = f
            .context()
            .comments()
            .leading_comments(node.syntax())
            .iter()
            .any(|comment| comment.kind().is_line());

        if leading_comments_with_break {
            write!(f, [hard_line_break()])?;
        }
        write!(f, [format_leading_comments(node.syntax())])?;

        if matches!(
            parent_kind,
            Some(
                MSyntaxKind::M_IF_STATEMENT
                    | MSyntaxKind::M_ELSE_CLAUSE
                    | MSyntaxKind::M_WHILE_STATEMENT
                    | MSyntaxKind::M_FOR_ALL_IN_STATEMENT
                    | MSyntaxKind::M_FOR_ALL_STATEMENT
                    | MSyntaxKind::M_FOR_STATEMENT
            )
        ) {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }

    fn fmt_leading_comments(&self, _: &MEmptyStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
