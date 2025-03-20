use crate::formatter::prelude::*;
use crate::syntax::MAnnotationAttributeList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationAttributeList;
impl_format!(MAnnotationAttributeList, FormatMAnnotationAttributeList);
impl FormatRule<MAnnotationAttributeList> for FormatMAnnotationAttributeList {
    type Context = MFormatContext;
    fn fmt(&self, node: &MAnnotationAttributeList, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
