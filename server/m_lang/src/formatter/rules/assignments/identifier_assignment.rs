use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::{MIdentifierAssignment, MIdentifierAssignmentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMIdentifierAssignment;
impl_format_with_rule!(MIdentifierAssignment, FormatMIdentifierAssignment);

impl FormatNodeRule<MIdentifierAssignment> for FormatMIdentifierAssignment {
    fn fmt_fields(&self, node: &MIdentifierAssignment, f: &mut MFormatter) -> FormatResult<()> {
        let MIdentifierAssignmentFields { name_token } = node.as_fields();

        write![f, [name_token.format()]]
    }

    fn needs_parentheses(&self, item: &MIdentifierAssignment) -> bool {
        item.needs_parentheses()
    }
}
