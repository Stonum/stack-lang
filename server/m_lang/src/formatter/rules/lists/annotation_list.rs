use crate::formatter::prelude::*;
use crate::syntax::MAnnotationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationList;
impl_format!(MAnnotationList, FormatMAnnotationList);
impl FormatRule<MAnnotationList> for FormatMAnnotationList {
    type Context = MFormatContext;
    fn fmt(&self, node: &MAnnotationList, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
