use crate::formatter::prelude::*;
use crate::formatter::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use crate::syntax::MLiteralMemberNameFields;
use crate::syntax::{MLiteralMemberName, MSyntaxKind};
use biome_formatter::token::number::format_number_token;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLiteralMemberName;
impl_format_with_rule!(MLiteralMemberName, FormatMLiteralMemberName);

impl FormatNodeRule<MLiteralMemberName> for FormatMLiteralMemberName {
    fn fmt_fields(&self, node: &MLiteralMemberName, f: &mut MFormatter) -> FormatResult<()> {
        let MLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            MSyntaxKind::M_STRING_LITERAL => {
                write![
                    f,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Member
                    )]
                ]
            }
            MSyntaxKind::M_NUMBER_LITERAL => format_number_token(&value).fmt(f),
            _ => write![f, [value.format()]],
        }
    }
}
