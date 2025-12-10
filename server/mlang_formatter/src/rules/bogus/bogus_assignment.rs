use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::FormatBogusNodeRule;
use crate::prelude::*;
use mlang_syntax::MBogusAssignment;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusAssignment;
impl FormatBogusNodeRule<MBogusAssignment> for FormatMBogusAssignment {}
impl FormatRule<mlang_syntax::MBogusAssignment>
    for crate::rules::bogus::bogus_assignment::FormatMBogusAssignment
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusAssignment, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusAssignment,
        crate::rules::bogus::bogus_assignment::FormatMBogusAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_assignment::FormatMBogusAssignment::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusAssignment {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusAssignment,
        crate::rules::bogus::bogus_assignment::FormatMBogusAssignment,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_assignment::FormatMBogusAssignment::default(),
        )
    }
}
