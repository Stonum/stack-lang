use crate::prelude::*;
use biome_formatter::write;

use mlang_syntax::MReportSection;
use mlang_syntax::MReportSectionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportSection;
impl_format_with_rule!(MReportSection, FormatMReportSection);

impl FormatNodeRule<MReportSection> for FormatMReportSection {
    fn fmt_fields(&self, node: &MReportSection, f: &mut MFormatter) -> FormatResult<()> {
        let MReportSectionFields { name, body } = node.as_fields();

        write!(f, [name.format(), hard_line_break(), body.format()])
    }
}
