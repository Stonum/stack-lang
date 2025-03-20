use crate::formatter::prelude::*;
use biome_formatter::write;

use crate::syntax::{MSyntaxKind, MVariableDeclaratorList};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMVariableDeclaratorList;
impl_format!(MVariableDeclaratorList, FormatMVariableDeclaratorList);

impl FormatRule<MVariableDeclaratorList> for FormatMVariableDeclaratorList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MVariableDeclaratorList, f: &mut MFormatter) -> FormatResult<()> {
        let length = node.len();

        let is_parent_for_loop = node.syntax().grand_parent().map_or(false, |grand_parent| {
            matches!(
                grand_parent.kind(),
                MSyntaxKind::M_FOR_STATEMENT
                    | MSyntaxKind::M_FOR_ALL_STATEMENT
                    | MSyntaxKind::M_FOR_ALL_IN_STATEMENT
            )
        });

        let has_any_initializer = node.iter().any(|declarator| {
            declarator.map_or(false, |declarator| declarator.initializer().is_some())
        });

        let format_separator = format_with(|f| {
            if !is_parent_for_loop && has_any_initializer {
                write!(f, [hard_line_break()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        let mut declarators = node.iter().zip(
            node.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Disallowed),
        );

        let (first_declarator, format_first_declarator) = match declarators.next() {
            Some((syntax, format_first_declarator)) => (syntax?, format_first_declarator),
            None => return Err(FormatError::SyntaxError),
        };

        if length == 1 && !f.comments().has_leading_comments(first_declarator.syntax()) {
            return write!(f, [format_first_declarator]);
        }

        write!(
            f,
            [indent(&format_once(|f| {
                write!(f, [format_first_declarator])?;

                if length > 1 {
                    write!(f, [format_separator])?;
                }

                f.join_with(&format_separator)
                    .entries(declarators.map(|(_, format)| format))
                    .finish()
            }))]
        )
    }
}
