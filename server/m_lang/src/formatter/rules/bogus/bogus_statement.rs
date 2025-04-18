use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::formatter::prelude::*;
use crate::formatter::FormatBogusNodeRule;
use crate::syntax::MBogusStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusStatement;
impl FormatBogusNodeRule<MBogusStatement> for FormatMBogusStatement {}
impl FormatRule<crate::syntax::MBogusStatement>
    for crate::formatter::rules::bogus::bogus_statement::FormatMBogusStatement
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &crate::syntax::MBogusStatement, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<crate::syntax::MBogusStatement>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for crate::syntax::MBogusStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        crate::syntax::MBogusStatement,
        crate::formatter::rules::bogus::bogus_statement::FormatMBogusStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_statement::FormatMBogusStatement::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for crate::syntax::MBogusStatement {
    type Format = FormatOwnedWithRule<
        crate::syntax::MBogusStatement,
        crate::formatter::rules::bogus::bogus_statement::FormatMBogusStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::formatter::rules::bogus::bogus_statement::FormatMBogusStatement::default(),
        )
    }
}
