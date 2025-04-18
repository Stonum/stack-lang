use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusAssignment;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusAssignment;
impl FormatBogusNodeRule<MBogusAssignment> for FormatMBogusAssignment {}
impl FormatRule<crate::syntax::MBogusAssignment>
    for crate::formatter::rules::bogus::bogus_assignment::FormatMBogusAssignment
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusAssignment, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusAssignment,
        crate::formatter::rules::bogus::bogus_assignment::FormatMBogusAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_assignment::FormatMBogusAssignment::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusAssignment {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusAssignment,
        crate::formatter::rules::bogus::bogus_assignment::FormatMBogusAssignment,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_assignment::FormatMBogusAssignment::default(),
        )
    }
}
