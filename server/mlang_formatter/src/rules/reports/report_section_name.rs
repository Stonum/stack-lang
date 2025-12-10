use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MReportSectionName;
use mlang_syntax::MReportSectionNameFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportSectionName;
impl_format_with_rule!(MReportSectionName, FormatMReportSectionName);

impl FormatNodeRule<MReportSectionName> for FormatMReportSectionName {
    fn fmt_fields(&self, node: &MReportSectionName, f: &mut MFormatter) -> FormatResult<()> {
        let MReportSectionNameFields { ff_token, m_name } = node.as_fields();

        write![f, [ff_token.format(), m_name.format()]]
    }
}
