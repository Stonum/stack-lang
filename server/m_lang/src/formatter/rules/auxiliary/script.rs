use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::syntax::MScript;
use crate::syntax::MScriptFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMScript;
impl_format_with_rule!(MScript, FormatMScript);

impl FormatNodeRule<MScript> for FormatMScript {
    fn fmt_fields(&self, node: &MScript, f: &mut MFormatter) -> FormatResult<()> {
        let MScriptFields {
            statements,
            eof_token,
        } = node.as_fields();

        write![f, [format_leading_comments(node.syntax())]]?;

        write![
            f,
            [
                statements.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        ]
    }

    fn fmt_leading_comments(&self, _: &MScript, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, node: &MScript, f: &mut MFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(node.syntax()),
            "Scrip should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &MScript, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
