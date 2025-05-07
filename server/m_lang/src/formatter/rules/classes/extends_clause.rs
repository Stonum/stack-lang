use crate::formatter::prelude::*;

use crate::syntax::MExtendsClause;
use crate::syntax::MExtendsClauseFields;
use crate::syntax::MSyntaxKind::M_ASSIGNMENT_EXPRESSION;
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMExtendsClause;
impl_format_with_rule!(MExtendsClause, FormatMExtendsClause);

impl FormatNodeRule<MExtendsClause> for FormatMExtendsClause {
    fn fmt_fields(&self, node: &MExtendsClause, f: &mut MFormatter) -> FormatResult<()> {
        let MExtendsClauseFields {
            extends_token,
            super_class,
        } = node.as_fields();

        let super_class = super_class?;

        let format_super = format_with(|f| {
            let content = format_with(|f| write!(f, [super_class.format()]));

            let comments = f.comments();
            let has_trailing_comments = comments.has_trailing_comments(super_class.syntax());

            if node
                .syntax()
                .grand_parent().is_some_and(|p| p.kind() == M_ASSIGNMENT_EXPRESSION)
            {
                if comments.has_leading_comments(super_class.syntax()) || has_trailing_comments {
                    write!(f, [text("("), &content, text(")")])
                } else {
                    let content = content.memoized();
                    write!(
                        f,
                        [
                            if_group_breaks(&format_args![
                                text("("),
                                &soft_block_indent(&content),
                                text(")"),
                            ]),
                            if_group_fits_on_line(&content)
                        ]
                    )
                }
            } else {
                content.fmt(f)
            }
        });

        write![f, [extends_token.format(), space(), group(&format_super)]]
    }
}
