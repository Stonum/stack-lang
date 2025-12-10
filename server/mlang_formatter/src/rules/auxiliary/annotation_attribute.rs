use crate::prelude::*;
use biome_rowan::AstNode;
use mlang_syntax::MAnnotationAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMAnnotationAttribute;
impl_format_with_rule!(MAnnotationAttribute, FormatMAnnotationAttribute);
impl FormatNodeRule<MAnnotationAttribute> for FormatMAnnotationAttribute {
    fn fmt_fields(&self, node: &MAnnotationAttribute, f: &mut MFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
