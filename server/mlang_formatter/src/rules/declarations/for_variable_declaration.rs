use crate::prelude::*;

use biome_formatter::{format_args, write};
use mlang_syntax::MForVariableDeclaration;
use mlang_syntax::MForVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForVariableDeclaration;
impl_format_with_rule!(MForVariableDeclaration, FormatMForVariableDeclaration);

impl FormatNodeRule<MForVariableDeclaration> for FormatMForVariableDeclaration {
    fn fmt_fields(&self, node: &MForVariableDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        let MForVariableDeclarationFields {
            var_token,
            declarator,
        } = node.as_fields();

        write![
            f,
            [group(&format_args![
                var_token.format(),
                space(),
                declarator.format()
            ])]
        ]
    }
}
