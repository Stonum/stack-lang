use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;

use biome_formatter::{CstFormatContext, write};
use biome_rowan::SyntaxNodeOptionExt;
use mlang_syntax::MExpressionStatementFields;
use mlang_syntax::expression_left_side::AnyMExpressionLeftSide;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    AnyMAssignment, AnyMExpression, MExpressionStatement, MSyntaxKind, MUnaryOperator,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMExpressionStatement;
impl_format_with_rule!(MExpressionStatement, FormatMExpressionStatement);

impl FormatNodeRule<MExpressionStatement> for FormatMExpressionStatement {
    fn fmt_node(&self, node: &MExpressionStatement, f: &mut MFormatter) -> FormatResult<()> {
        let needs_parentheses = self.needs_parentheses(node);
        let is_after_bogus = f
            .elements()
            .start_tag(TagKind::Verbatim)
            .is_some_and(|signal| match signal {
                Tag::StartVerbatim(kind) => kind.is_bogus(),
                _ => unreachable!(),
            });

        if f.options().semicolons().is_as_needed()
            // Don't perform semicolon insertion if the previous statement is an bogus statement.
            && !is_after_bogus
            && (needs_parentheses || needs_semicolon(node))
        {
            write!(f, [text(";")])?;
        }

        if needs_parentheses {
            write!(f, [text("(")])?;
        }

        self.fmt_fields(node, f)?;

        if needs_parentheses {
            write!(f, [text(")")])?;
        }

        Ok(())
    }

    fn fmt_fields(&self, node: &MExpressionStatement, f: &mut MFormatter) -> FormatResult<()> {
        let MExpressionStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        let has_dangling_comments = f.context().comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [
                expression.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )?;

        if has_dangling_comments {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        _: &MExpressionStatement,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Returns `true` if a semicolon is required to keep the semantics of the program.
///
/// Semicolons are optional in most places in JavaScript, but they are sometimes required. Generally,
/// semicolons are necessary if an identifier + start of a statement may form a valid expression. For example:
///
/// ```javascript
/// a
/// ["b"]
/// ```
///
/// The above can either be the computed member expression `a["b"]` or the identifier `a` followed by an
/// expression statement `["b"]`.
///
/// Tokens that need a semicolon are:
///
/// * binary operators: `<`, `+`, `-`,  ...
/// * `[` or `(`
/// * ticks: `\``
fn needs_semicolon(node: &MExpressionStatement) -> bool {
    use AnyMExpression::*;

    if !matches!(
        node.syntax().parent().kind(),
        Some(MSyntaxKind::M_MODULE_ITEM_LIST | MSyntaxKind::M_STATEMENT_LIST)
    ) {
        return false;
    }

    let Ok(expression) = node.expression() else {
        return false;
    };

    let mut expression: Option<AnyMExpressionLeftSide> = Some(expression.into());

    while let Some(current) = expression.take() {
        let needs_semi = match &current {
            AnyMExpressionLeftSide::AnyMExpression(expression) => match expression {
                MArrayExpression(_) | MParenthesizedExpression(_) => true,

                MUnaryExpression(unary) => matches!(
                    unary.operator(),
                    Ok(MUnaryOperator::Plus | MUnaryOperator::Minus)
                ),

                _ => false,
            },
            AnyMExpressionLeftSide::AnyMAssignment(assignment) => {
                matches!(assignment, AnyMAssignment::MParenthesizedAssignment(_))
            }
        };

        if needs_semi || current.needs_parentheses() {
            return true;
        }

        expression = match current.left_expression() {
            Some(inner) => Some(inner),
            None => return false,
        };
    }

    false
}
