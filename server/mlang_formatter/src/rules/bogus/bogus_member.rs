use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::prelude::*;
use crate::FormatBogusNodeRule;
use mlang_syntax::MBogusMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusMember;
impl FormatBogusNodeRule<MBogusMember> for FormatMBogusMember {}
impl FormatRule<mlang_syntax::MBogusMember>
    for crate::rules::bogus::bogus_member::FormatMBogusMember
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusMember, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusMember,
        crate::rules::bogus::bogus_member::FormatMBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_member::FormatMBogusMember::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusMember {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusMember,
        crate::rules::bogus::bogus_member::FormatMBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_member::FormatMBogusMember::default(),
        )
    }
}
