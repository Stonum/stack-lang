use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::FormatBogusNodeRule;
use crate::prelude::*;
use mlang_syntax::MBogusParameter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusParameter;
impl FormatBogusNodeRule<MBogusParameter> for FormatMBogusParameter {}
impl FormatRule<mlang_syntax::MBogusParameter>
    for crate::rules::bogus::bogus_parameter::FormatMBogusParameter
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusParameter, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusParameter,
        crate::rules::bogus::bogus_parameter::FormatMBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_parameter::FormatMBogusParameter::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusParameter {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusParameter,
        crate::rules::bogus::bogus_parameter::FormatMBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_parameter::FormatMBogusParameter::default(),
        )
    }
}
