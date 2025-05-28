use crate::formatter::prelude::*;

use crate::syntax::MReportSectionName;
use crate::syntax::MReportSectionNameFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportSectionName;
impl_format_with_rule!(MReportSectionName, FormatMReportSectionName);

impl FormatNodeRule<MReportSectionName> for FormatMReportSectionName {
    fn fmt_fields(&self, node: &MReportSectionName, f: &mut MFormatter) -> FormatResult<()> {
        let MReportSectionNameFields { ff_token, m_name } = node.as_fields();

        write![f, [ff_token.format(), m_name.format()]]
    }
}
