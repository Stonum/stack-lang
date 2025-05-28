use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::syntax::MReport;
use crate::syntax::MReportFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReport;
impl_format_with_rule!(MReport, FormatMReport);

impl FormatNodeRule<MReport> for FormatMReport {
    fn fmt_fields(&self, node: &MReport, f: &mut MFormatter) -> FormatResult<()> {
        let MReportFields {
            name,
            init,
            sections,
            default,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                hard_line_break(),
                init.format(),
                hard_line_break(),
                default.format(),
                hard_line_break(),
                sections.format(),
            ]
        )
    }
}
