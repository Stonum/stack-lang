use crate::prelude::*;
use mlang_syntax::MAnnotationGroupList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationGroupList;
impl_format!(MAnnotationGroupList, FormatMAnnotationGroupList);
impl FormatRule<MAnnotationGroupList> for FormatMAnnotationGroupList {
    type Context = MFormatContext;
    fn fmt(&self, node: &MAnnotationGroupList, f: &mut MFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
