use biome_formatter::FormatOwnedWithRule;
use biome_formatter::FormatRefWithRule;

use crate::prelude::*;
use crate::FormatBogusNodeRule;
use mlang_syntax::MBogus;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogus;
impl FormatBogusNodeRule<MBogus> for FormatMBogus {}
impl FormatRule<mlang_syntax::MBogus> for crate::rules::bogus::bogus::FormatMBogus {
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogus, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogus>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogus,
        crate::rules::bogus::bogus::FormatMBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus::FormatMBogus::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogus {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogus,
        crate::rules::bogus::bogus::FormatMBogus,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus::FormatMBogus::default(),
        )
    }
}
