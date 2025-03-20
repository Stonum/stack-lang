use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::formatter::utils::FormatInterpreterToken;

use crate::syntax::MModule;
use crate::syntax::MModuleFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMModule;
impl_format_with_rule!(MModule, FormatMModule);

impl FormatNodeRule<MModule> for FormatMModule {
    fn fmt_fields(&self, node: &MModule, f: &mut MFormatter) -> FormatResult<()> {
        let MModuleFields {
            directives,
            items,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [format_leading_comments(node.syntax()), directives.format()]
        ]?;

        write!(
            f,
            [
                items.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        )
    }

    fn fmt_leading_comments(&self, _: &MModule, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, module: &MModule, f: &mut MFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(module.syntax()),
            "Module should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &MModule, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
