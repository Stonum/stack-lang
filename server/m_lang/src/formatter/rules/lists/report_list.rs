use crate::formatter::prelude::*;
use crate::syntax::MReportList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportList;
impl_format!(MReportList, FormatMReportList);

impl FormatRule<MReportList> for FormatMReportList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MReportList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for statement in node.iter() {
            join.entry(statement.syntax(), &format_or_verbatim(statement.format()));
        }

        join.finish()
    }
}
