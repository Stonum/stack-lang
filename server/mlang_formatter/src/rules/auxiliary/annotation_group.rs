use crate::prelude::*;
use mlang_syntax::MAnnotationGroup;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationGroup;
impl_format_with_rule!(MAnnotationGroup, FormatMAnnotationGroup);
impl FormatNodeRule<MAnnotationGroup> for FormatMAnnotationGroup {
    fn fmt_fields(&self, node: &MAnnotationGroup, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
