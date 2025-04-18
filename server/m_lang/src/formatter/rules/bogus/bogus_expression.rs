use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusExpression;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusExpression;
impl FormatBogusNodeRule<MBogusExpression> for FormatMBogusExpression {}
impl FormatRule<crate::syntax::MBogusExpression>
    for crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusExpression, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusExpression,
        crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusExpression {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusExpression,
        crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression::default(),
        )
    }
}
