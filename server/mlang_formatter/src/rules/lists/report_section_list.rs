use crate::prelude::*;
use mlang_syntax::MReportSectionList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportSectionList;
impl_format!(MReportSectionList, FormatMReportSectionList);

impl FormatRule<MReportSectionList> for FormatMReportSectionList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MReportSectionList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(member.format()));
        }

        join.finish()
    }
}
