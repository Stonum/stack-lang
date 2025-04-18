use crate::formatter::prelude::*;
use crate::syntax::MAnnotationElement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationElement;
impl_format_with_rule!(MAnnotationElement, FormatMAnnotationElement);
impl FormatNodeRule<MAnnotationElement> for FormatMAnnotationElement {
    fn fmt_fields(&self, node: &MAnnotationElement, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
