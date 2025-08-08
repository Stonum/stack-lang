use crate::prelude::*;
use mlang_syntax::AnyMStatement;
use mlang_syntax::MCaseClause;
use mlang_syntax::MCaseClauseFields;
use biome_formatter::{format_args, write};
use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCaseClause;
impl_format_with_rule!(MCaseClause, FormatMCaseClause);

impl FormatNodeRule<MCaseClause> for FormatMCaseClause {
    fn fmt_fields(&self, node: &MCaseClause, f: &mut MFormatter) -> FormatResult<()> {
        let MCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                case_token.format(),
                // space(),
                soft_line_indent_or_space(&format_args![test.format()]),
                // test.format(),
                colon_token.format()
            ))]
        )?;

        // Whether the first statement in the clause is a BlockStatement, and
        // there are no other non-empty statements. Empties may show up when
        // parsing depending on if the input code includes certain newlines.
        let is_single_block_statement = matches!(
            consequent.iter().next(),
            Some(AnyMStatement::MBlockStatement(_))
        ) && consequent
            .iter()
            .filter(|statement| !matches!(statement, AnyMStatement::MEmptyStatement(_)))
            .count()
            == 1;

        // When the case block is empty, the case becomes a fallthrough, so it
        // is collapsed directly on top of the next case (just a single
        // hardline).
        // When the block is a single statement _and_ it's a block statement,
        // then the opening brace of the block can hug the same line as the
        // case. But, if there's more than one statement, then the block
        // _cannot_ hug. This distinction helps clarify that the case continues
        // past the end of the block statement, despite the braces making it
        // seem like it might end.
        // Lastly, the default case is just to break and indent the body.
        //
        // switch(key) {
        //   case fallthrough: // trailing comment
        //   case normalBody:
        //     someWork();
        //     break;
        //
        //   case blockBody:
        //   {
        //     var a = 1;
        //     break;
        //   }
        //
        //   case separateBlockBody:
        //   {
        //     breakIsNotInsideTheBlock();
        //     break;
        //   }
        //
        //   else:
        //     break;
        // }
        if consequent.is_empty() {
            // Print nothing to ensure that trailing comments on the same line
            // are printed on the same line. The parent list formatter takes
            // care of inserting a hard line break between cases.
            Ok(())
        } else if is_single_block_statement {
            write![f, [hard_line_break(), consequent.format()]]
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args![
                    hard_line_break(),
                    consequent.format()
                ])]
            )
        }
    }
}
