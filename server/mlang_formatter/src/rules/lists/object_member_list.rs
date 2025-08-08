use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;
use mlang_syntax::MObjectMemberList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMObjectMemberList;
impl_format!(MObjectMemberList, FormatMObjectMemberList);

impl FormatRule<MObjectMemberList> for FormatMObjectMemberList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MObjectMemberList, f: &mut MFormatter) -> FormatResult<()> {
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
