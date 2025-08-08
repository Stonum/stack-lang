use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use mlang_syntax::MHashMapMemberList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMHashMapMemberList;
impl_format!(MHashMapMemberList, FormatMHashMapMemberList);

impl FormatRule<MHashMapMemberList> for FormatMHashMapMemberList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MHashMapMemberList, f: &mut MFormatter) -> FormatResult<()> {
        let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(
            node.format_separated(",")
                .with_trailing_separator(trailing_separator),
        ) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
