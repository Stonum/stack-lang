use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusMember;
impl FormatBogusNodeRule<MBogusMember> for FormatMBogusMember {}
impl FormatRule<crate::syntax::MBogusMember>
    for crate::formatter::rules::bogus::bogus_member::FormatMBogusMember
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusMember, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusMember,
        crate::formatter::rules::bogus::bogus_member::FormatMBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_member::FormatMBogusMember::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusMember {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusMember,
        crate::formatter::rules::bogus::bogus_member::FormatMBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_member::FormatMBogusMember::default(),
        )
    }
}
