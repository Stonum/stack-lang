use crate::prelude::*;
use mlang_syntax::MAnnotationBinding;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationBinding;
impl_format_with_rule!(MAnnotationBinding, FormatMAnnotationBinding);
impl FormatNodeRule<MAnnotationBinding> for FormatMAnnotationBinding {
    fn fmt_fields(&self, node: &MAnnotationBinding, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
