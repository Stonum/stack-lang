//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::MName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMName;
impl_format!(MName, FormatMName);
impl FormatRule<MName> for FormatMName {
    type Context = MFormatContext;
    fn fmt(&self, node: &MName, f: &mut MFormatter) -> FormatResult<()> {
        node.format().fmt(f)
    }
}
