use crate::formatter::prelude::*;
use crate::syntax::MDirectiveList;
use biome_formatter::write;
use biome_rowan::{AstNode, AstNodeList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMDirectiveList;
impl_format!(MDirectiveList, FormatMDirectiveList);

impl FormatRule<MDirectiveList> for FormatMDirectiveList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MDirectiveList, f: &mut MFormatter) -> FormatResult<()> {
        if node.is_empty() {
            return Ok(());
        }

        let syntax_node = node.syntax();
        let next_sibling = syntax_node.next_sibling();
        // if next_sibling's first leading_trivia has more than one new_line, we should add an extra empty line at the end of
        // MDirectiveList, for example:
        //```JavaScript
        // "use strict"; <- first leading new_line
        //  			 <- second leading new_line
        // function foo() {

        // }
        //```
        // so we should keep an extra empty line after MDirectiveList
        let need_extra_empty_line = if let Some(next_sibling) = next_sibling {
            get_lines_before(&next_sibling) > 1
        } else {
            false
        };

        let mut join = f.join_nodes_with_hardline();

        for directive in node {
            join.entry(directive.syntax(), &directive.format());
        }

        join.finish()?;

        if need_extra_empty_line {
            write!(f, [empty_line()])
        } else {
            write!(f, [hard_line_break()])
        }
    }
}
