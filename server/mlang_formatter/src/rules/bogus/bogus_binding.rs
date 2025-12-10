use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::FormatBogusNodeRule;
use crate::prelude::*;
use mlang_syntax::MBogusBinding;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusBinding;
impl FormatBogusNodeRule<MBogusBinding> for FormatMBogusBinding {}
impl FormatRule<mlang_syntax::MBogusBinding>
    for crate::rules::bogus::bogus_binding::FormatMBogusBinding
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusBinding, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusBinding>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusBinding,
        crate::rules::bogus::bogus_binding::FormatMBogusBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_binding::FormatMBogusBinding::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusBinding {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusBinding,
        crate::rules::bogus::bogus_binding::FormatMBogusBinding,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_binding::FormatMBogusBinding::default(),
        )
    }
}
