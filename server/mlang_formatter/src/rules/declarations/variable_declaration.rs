use crate::prelude::*;
use biome_formatter::{format_args, write};

use mlang_syntax::MVariableDeclaration;
use mlang_syntax::MVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableDeclaration;
impl_format_with_rule!(MVariableDeclaration, FormatMVariableDeclaration);

impl FormatNodeRule<MVariableDeclaration> for FormatMVariableDeclaration {
    fn fmt_fields(&self, node: &MVariableDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        let MVariableDeclarationFields {
            kind_token,
            declarators,
        } = node.as_fields();

        write![
            f,
            [group(&format_args![
                kind_token.format(),
                space(),
                declarators.format()
            ])]
        ]
    }
}
