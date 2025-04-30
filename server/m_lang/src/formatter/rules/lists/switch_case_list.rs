use crate::formatter::prelude::*;
use crate::syntax::MSwitchCaseList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSwitchCaseList;
impl_format!(MSwitchCaseList, FormatMSwitchCaseList);

impl FormatRule<MSwitchCaseList> for FormatMSwitchCaseList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MSwitchCaseList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for case in node {
            join.entry(case.syntax(), &format_or_verbatim(case.format()));
        }

        join.finish()
    }
}
