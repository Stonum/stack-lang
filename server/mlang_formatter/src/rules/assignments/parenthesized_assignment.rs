use crate::prelude::*;

use mlang_syntax::MParenthesizedAssignment;
use mlang_syntax::MParenthesizedAssignmentFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMParenthesizedAssignment;
impl_format_with_rule!(MParenthesizedAssignment, FormatMParenthesizedAssignment);

impl FormatNodeRule<MParenthesizedAssignment> for FormatMParenthesizedAssignment {
    fn fmt_fields(&self, node: &MParenthesizedAssignment, f: &mut MFormatter) -> FormatResult<()> {
        let MParenthesizedAssignmentFields {
            l_paren_token,
            assignment,
            r_paren_token,
        } = node.as_fields();

        write![
            f,
            [
                l_paren_token.format(),
                assignment.format(),
                r_paren_token.format(),
            ]
        ]
    }
}
