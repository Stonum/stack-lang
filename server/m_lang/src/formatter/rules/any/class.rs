//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::MClass;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMClass;
impl_format!(MClass, FormatMClass);

impl FormatRule<MClass> for FormatMClass {
    type Context = MFormatContext;
    fn fmt(&self, node: &MClass, f: &mut MFormatter) -> FormatResult<()> {
        node.format().fmt(f)
    }
}
