use crate::formatter::prelude::*;
use crate::formatter::utils::AnyMAssignmentLike;
use crate::syntax::MVariableDeclarator;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableDeclarator;
impl_format_with_rule!(MVariableDeclarator, FormatMVariableDeclarator);

impl FormatNodeRule<MVariableDeclarator> for FormatMVariableDeclarator {
    fn fmt_fields(&self, node: &MVariableDeclarator, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }
}
