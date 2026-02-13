use crate::prelude::*;

use biome_formatter::write;
use biome_rowan::AstNodeList;
use mlang_syntax::{MSwitchStatement, MSwitchStatementFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSwitchStatement;
impl_format_with_rule!(MSwitchStatement, FormatMSwitchStatement);

impl FormatNodeRule<MSwitchStatement> for FormatMSwitchStatement {
    fn fmt_fields(&self, node: &MSwitchStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MSwitchStatementFields {
            switch_token,
            l_paren_token,
            discriminant,
            r_paren_token,
            l_curly_token,
            cases,
            r_curly_token,
        } = node.as_fields();

        let format_cases = format_with(|f| {
            if cases.is_empty() {
                hard_line_break().fmt(f)?;
            } else {
                cases.format().fmt(f)?;
            }

            Ok(())
        });

        write![
            f,
            [
                switch_token.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&discriminant.format())),
                r_paren_token.format(),
                hard_line_break(),
                l_curly_token.format(),
                block_indent(&format_cases),
                r_curly_token.format()
            ]
        ]
    }
}
