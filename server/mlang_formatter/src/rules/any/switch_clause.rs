//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMSwitchClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMSwitchClause;
impl_format!(AnyMSwitchClause, FormatAnyMSwitchClause);
impl FormatRule<AnyMSwitchClause> for FormatAnyMSwitchClause {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMSwitchClause, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMSwitchClause::MCaseClause(node) => node.format().fmt(f),
            AnyMSwitchClause::MDefaultClause(node) => node.format().fmt(f),
        }
    }
}
