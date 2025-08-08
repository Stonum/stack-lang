use crate::prelude::*;
use mlang_syntax::{AnyMStatement, MModuleItemList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMModuleItemList;
impl_format!(MModuleItemList, FormatMModuleItemList);

impl FormatRule<MModuleItemList> for FormatMModuleItemList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MModuleItemList, f: &mut MFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for module_item in node {
            match module_item {
                AnyMStatement::MEmptyStatement(empty) => {
                    join.entry_no_separator(&empty.format());
                }
                _ => {
                    join.entry(
                        module_item.syntax(),
                        &format_or_verbatim(module_item.format()),
                    );
                }
            }
        }

        join.finish()
    }
}
