use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusBinding;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusBinding;
impl FormatBogusNodeRule<MBogusBinding> for FormatMBogusBinding {}
impl FormatRule<crate::syntax::MBogusBinding>
    for crate::formatter::rules::bogus::bogus_binding::FormatMBogusBinding
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusBinding, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusBinding>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusBinding,
        crate::formatter::rules::bogus::bogus_binding::FormatMBogusBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_binding::FormatMBogusBinding::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusBinding {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusBinding,
        crate::formatter::rules::bogus::bogus_binding::FormatMBogusBinding,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_binding::FormatMBogusBinding::default(),
        )
    }
}
