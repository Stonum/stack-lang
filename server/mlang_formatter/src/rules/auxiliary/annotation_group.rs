use crate::prelude::*;
use biome_rowan::AstNode;
use mlang_syntax::MAnnotationGroup;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationGroup;
impl_format_with_rule!(MAnnotationGroup, FormatMAnnotationGroup);
impl FormatNodeRule<MAnnotationGroup> for FormatMAnnotationGroup {
    fn fmt_fields(&self, node: &MAnnotationGroup, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
