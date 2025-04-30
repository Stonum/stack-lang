use crate::formatter::prelude::*;
use crate::syntax::MClassMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMClassMemberList;
impl_format!(MClassMemberList, FormatMClassMemberList);

impl FormatRule<MClassMemberList> for FormatMClassMemberList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MClassMemberList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(member.format()));
        }

        join.finish()
    }
}
