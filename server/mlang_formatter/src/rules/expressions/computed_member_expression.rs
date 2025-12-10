use crate::prelude::*;

use biome_formatter::{format_args, write};
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    AnyMComputedMember, AnyMExpression, AnyMLiteralExpression, MComputedMemberExpression,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMComputedMemberExpression;
impl_format_with_rule!(MComputedMemberExpression, FormatMComputedMemberExpression);

impl FormatNodeRule<MComputedMemberExpression> for FormatMComputedMemberExpression {
    fn fmt_fields(&self, node: &MComputedMemberExpression, f: &mut MFormatter) -> FormatResult<()> {
        AnyMComputedMember::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MComputedMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

impl Format<MFormatContext> for AnyMComputedMember {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        write!(f, [self.object().format()])?;

        FormatComputedMemberLookup(self).fmt(f)
    }
}

/// Formats the lookup portion (everything except the object) of a computed member like.
pub(crate) struct FormatComputedMemberLookup<'a>(&'a AnyMComputedMember);

impl<'a> FormatComputedMemberLookup<'a> {
    pub(crate) fn new(member_like: &'a AnyMComputedMember) -> Self {
        Self(member_like)
    }
}

impl Format<MFormatContext> for FormatComputedMemberLookup<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self.0.member()? {
            AnyMExpression::AnyMLiteralExpression(
                AnyMLiteralExpression::MNumberLiteralExpression(literal),
            ) => {
                write!(
                    f,
                    [
                        self.0.l_brack_token().format(),
                        literal.format(),
                        self.0.r_brack_token().format()
                    ]
                )
            }
            member => {
                write![
                    f,
                    [group(&format_args![
                        self.0.l_brack_token().format(),
                        soft_block_indent(&member.format()),
                        self.0.r_brack_token().format()
                    ])]
                ]
            }
        }
    }
}
