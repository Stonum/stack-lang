use crate::formatter::prelude::*;
use crate::syntax::{MHashMapExpression, MObjectExpression, MSyntaxToken};
use biome_formatter::write;
use biome_formatter::{Format, FormatResult};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, SyntaxResult};

declare_node_union! {
    pub (crate) MObjectLike = MObjectExpression | MHashMapExpression
}
impl MObjectLike {
    fn at_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            MObjectLike::MObjectExpression(oe) => oe.at_token(),
            MObjectLike::MHashMapExpression(ot) => ot.at_token(),
        }
    }
    fn l_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            MObjectLike::MObjectExpression(oe) => oe.l_curly_token(),
            MObjectLike::MHashMapExpression(ot) => ot.l_paren_token(),
        }
    }
    fn r_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            MObjectLike::MObjectExpression(oe) => oe.r_curly_token(),
            MObjectLike::MHashMapExpression(ot) => ot.r_paren_token(),
        }
    }

    fn members_have_leading_newline(&self) -> bool {
        match self {
            MObjectLike::MObjectExpression(oe) => oe.members().syntax().has_leading_newline(),
            MObjectLike::MHashMapExpression(ot) => ot.members().syntax().has_leading_newline(),
        }
    }

    fn members_are_empty(&self) -> bool {
        match self {
            MObjectLike::MObjectExpression(oe) => oe.members().is_empty(),
            MObjectLike::MHashMapExpression(ot) => ot.members().is_empty(),
        }
    }

    fn write_members(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            MObjectLike::MObjectExpression(oe) => {
                write!(f, [oe.members().format()])
            }
            MObjectLike::MHashMapExpression(ot) => {
                write!(f, [ot.members().format()])
            }
        }
    }
}

impl Format<MFormatContext> for MObjectLike {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let members = format_with(|f| self.write_members(f));

        write!(f, [self.at_token().format(), self.l_token().format(),])?;

        if self.members_are_empty() {
            write!(
                f,
                [format_dangling_comments(self.syntax()).with_block_indent(),]
            )?;
        } else {
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            let should_expand = self.members_have_leading_newline();

            let inner =
                &soft_block_indent_with_maybe_space(&members, should_insert_space_around_brackets);

            write!(f, [group(inner).should_expand(should_expand)])?;
        }

        write!(f, [self.r_token().format()])
    }
}
