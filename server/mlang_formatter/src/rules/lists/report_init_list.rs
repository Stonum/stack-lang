use crate::prelude::*;
use mlang_syntax::MReportInitList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReportInitList;
impl_format!(MReportInitList, FormatMReportInitList);

impl FormatRule<MReportInitList> for FormatMReportInitList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MReportInitList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();
        for statement in node.iter() {
            join.entry(statement.syntax(), &format_or_verbatim(statement.format()));
        }

        join.finish()
    }
}
