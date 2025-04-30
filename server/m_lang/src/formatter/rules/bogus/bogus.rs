use biome_formatter::FormatOwnedWithRule;
use biome_formatter::FormatRefWithRule;

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogus;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogus;
impl FormatBogusNodeRule<MBogus> for FormatMBogus {}
impl FormatRule<crate::syntax::MBogus> for crate::formatter::rules::bogus::bogus::FormatMBogus {
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogus, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogus>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogus,
        crate::formatter::rules::bogus::bogus::FormatMBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus::FormatMBogus::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogus {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogus,
        crate::formatter::rules::bogus::bogus::FormatMBogus,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus::FormatMBogus::default(),
        )
    }
}
