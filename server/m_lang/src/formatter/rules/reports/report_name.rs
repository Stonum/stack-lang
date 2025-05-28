use crate::formatter::prelude::*;

use crate::syntax::MReportName;
use crate::syntax::MReportNameFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportName;
impl_format_with_rule!(MReportName, FormatMReportName);

impl FormatNodeRule<MReportName> for FormatMReportName {
    fn fmt_fields(&self, node: &MReportName, f: &mut MFormatter) -> FormatResult<()> {
        let MReportNameFields { ff2_token, m_name } = node.as_fields();

        write![f, [ff2_token.format(), m_name.format()]]
    }
}
