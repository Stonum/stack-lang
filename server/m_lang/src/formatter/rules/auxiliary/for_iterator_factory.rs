use crate::formatter::prelude::*;
use crate::syntax::MForIteratorFactory;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForIteratorFactory;
impl_format_with_rule!(MForIteratorFactory, FormatMForIteratorFactory);
impl FormatNodeRule<MForIteratorFactory> for FormatMForIteratorFactory {
    fn fmt_fields(&self, node: &MForIteratorFactory, f: &mut MFormatter) -> FormatResult<()> {
        todo!("implement for iterator factory")
        // format_verbatim_node(node.syntax()).fmt(f)
    }
}
