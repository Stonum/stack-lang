use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusParameter;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusParameter;
impl FormatBogusNodeRule<MBogusParameter> for FormatMBogusParameter {}
impl FormatRule<crate::syntax::MBogusParameter>
    for crate::formatter::rules::bogus::bogus_parameter::FormatMBogusParameter
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusParameter, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusParameter,
        crate::formatter::rules::bogus::bogus_parameter::FormatMBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_parameter::FormatMBogusParameter::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusParameter {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusParameter,
        crate::formatter::rules::bogus::bogus_parameter::FormatMBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_parameter::FormatMBogusParameter::default(),
        )
    }
}
