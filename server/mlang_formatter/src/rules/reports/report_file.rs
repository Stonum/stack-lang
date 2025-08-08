use crate::prelude::*;
use biome_formatter::write;

use mlang_syntax::MReportFile;
use mlang_syntax::MReportFileFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportFile;
impl_format_with_rule!(MReportFile, FormatMReportFile);

impl FormatNodeRule<MReportFile> for FormatMReportFile {
    fn fmt_fields(&self, node: &MReportFile, f: &mut MFormatter) -> FormatResult<()> {
        let MReportFileFields { reports, eof_token } = node.as_fields();
        print!("hey");
        write!(
            f,
            [
                reports.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        )
    }

    fn fmt_leading_comments(&self, _: &MReportFile, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, module: &MReportFile, f: &mut MFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(module.syntax()),
            "ReportFile should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &MReportFile, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
