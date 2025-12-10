use crate::prelude::*;
use crate::utils::AnyMAssignmentLike;
use biome_formatter::write;
use mlang_syntax::MVariableDeclarator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableDeclarator;
impl_format_with_rule!(MVariableDeclarator, FormatMVariableDeclarator);

impl FormatNodeRule<MVariableDeclarator> for FormatMVariableDeclarator {
    fn fmt_fields(&self, node: &MVariableDeclarator, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [AnyMAssignmentLike::from(node.clone())]]
    }
}
