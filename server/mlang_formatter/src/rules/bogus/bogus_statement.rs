use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use crate::prelude::*;
use crate::FormatBogusNodeRule;
use mlang_syntax::MBogusStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMBogusStatement;
impl FormatBogusNodeRule<MBogusStatement> for FormatMBogusStatement {}
impl FormatRule<mlang_syntax::MBogusStatement>
    for crate::rules::bogus::bogus_statement::FormatMBogusStatement
{
    type Context = MFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &mlang_syntax::MBogusStatement, f: &mut MFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<mlang_syntax::MBogusStatement>::fmt(self, node, f)
    }
}
impl AsFormat<MFormatContext> for mlang_syntax::MBogusStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        mlang_syntax::MBogusStatement,
        crate::rules::bogus::bogus_statement::FormatMBogusStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::rules::bogus::bogus_statement::FormatMBogusStatement::default(),
        )
    }
}
impl IntoFormat<MFormatContext> for mlang_syntax::MBogusStatement {
    type Format = FormatOwnedWithRule<
        mlang_syntax::MBogusStatement,
        crate::rules::bogus::bogus_statement::FormatMBogusStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::rules::bogus::bogus_statement::FormatMBogusStatement::default(),
        )
    }
}
