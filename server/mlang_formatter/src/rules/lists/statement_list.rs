use crate::prelude::*;
use mlang_syntax::{AnyMStatement, MStatementList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStatementList;
impl_format!(MStatementList, FormatMStatementList);

impl FormatRule<MStatementList> for FormatMStatementList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MStatementList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for statement in node.iter() {
            match statement {
                AnyMStatement::MEmptyStatement(empty) => {
                    join.entry_no_separator(&empty.format());
                }
                _ => {
                    join.entry(statement.syntax(), &format_or_verbatim(statement.format()));
                }
            }
        }

        join.finish()
    }
}
