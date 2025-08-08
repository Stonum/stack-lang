use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::prelude::*;
use crate::FormatBogusNodeRule;
use mlang_syntax::MBogusExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusExpression;
impl FormatBogusNodeRule<MBogusExpression> for FormatMBogusExpression {}
impl FormatRule<mlang_syntax::MBogusExpression>
    for crate::rules::bogus::bogus_expression::FormatMBogusExpression
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusExpression, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusExpression,
        crate::rules::bogus::bogus_expression::FormatMBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_expression::FormatMBogusExpression::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusExpression {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusExpression,
        crate::rules::bogus::bogus_expression::FormatMBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_expression::FormatMBogusExpression::default(),
        )
    }
}
