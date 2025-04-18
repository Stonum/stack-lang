use crate::formatter::prelude::*;
use crate::syntax::MDefaultClause;
use crate::syntax::{AnyMStatement, MDefaultClauseFields};
use biome_formatter::{format_args, write};
use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMDefaultClause;
impl_format_with_rule!(MDefaultClause, FormatMDefaultClause);

impl FormatNodeRule<MDefaultClause> for FormatMDefaultClause {
    fn fmt_fields(&self, node: &MDefaultClause, f: &mut MFormatter) -> FormatResult<()> {
        let MDefaultClauseFields {
            else_token,
            consequent,
        } = node.as_fields();

        // Whether the first statement in the clause is a BlockStatement, and
        // there are no other non-empty statements. Empties may show up when
        // parsing depending on if the input code includes certain newlines.
        //
        // See the comments in `case_clause.rs` for a detailed example.
        let is_single_block_statement = matches!(
            consequent.iter().next(),
            Some(AnyMStatement::MBlockStatement(_))
        ) && consequent
            .iter()
            .filter(|statement| !matches!(statement, AnyMStatement::MEmptyStatement(_)))
            .count()
            == 1;

        write!(f, [else_token.format()])?;

        if f.comments().has_dangling_comments(node.syntax()) {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        if consequent.is_empty() {
            return Ok(());
        }

        if is_single_block_statement {
            write!(f, [space(), consequent.format()])
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args!(
                    hard_line_break(),
                    consequent.format()
                ))]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &MDefaultClause, _: &mut MFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
