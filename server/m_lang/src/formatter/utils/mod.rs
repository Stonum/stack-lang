pub(crate) mod array;
mod assignment_like;
pub mod string_utils;

pub(crate) mod format_binary_like_expression;
pub(crate) mod format_class;
pub(crate) mod function_body;
pub(crate) mod member_chain;
pub(crate) mod object;
mod object_like;
#[cfg(test)]
mod quickcheck_utils;

use super::context;
use super::context::trailing_commas::FormatTrailingCommas;
use super::context::Semicolons;
use super::prelude::*;
use crate::syntax::{AnyMExpression, AnyMStatement, MCallExpression, MInitializerClause};
use crate::syntax::{MSyntaxKind, MSyntaxToken};
pub(crate) use assignment_like::{
    with_assignment_layout, AnyMAssignmentLike, AssignmentLikeLayout,
};
use biome_formatter::{format_args, write, Buffer};
use biome_rowan::AstNode;

pub(crate) use object_like::*;
pub(crate) use string_utils::*;

/// Tests if expression is a long curried call
///
/// ```javascript
/// `connect(a, b, c)(d)`
/// ```
pub(crate) fn is_long_curried_call(expression: Option<&MCallExpression>) -> bool {
    if let Some(expression) = expression {
        if let Some(parent_call) = expression.parent::<MCallExpression>() {
            if let (Ok(arguments), Ok(parent_arguments)) =
                (expression.arguments(), parent_call.arguments())
            {
                let is_callee = matches!(
                    parent_call.syntax().kind(),
                    MSyntaxKind::M_CALL_EXPRESSION | MSyntaxKind::M_NEW_EXPRESSION
                );
                return is_callee
                    && arguments.args().len() > parent_arguments.args().len()
                    && !parent_arguments.args().is_empty();
            }
        }
    }

    false
}

/// Utility function to format the node [crate::syntax::MInitializerClause]
pub(crate) struct FormatInitializerClause<'a> {
    initializer: Option<&'a MInitializerClause>,
}

impl<'a> FormatInitializerClause<'a> {
    pub fn new(initializer: Option<&'a MInitializerClause>) -> Self {
        Self { initializer }
    }
}

impl Format<MFormatContext> for FormatInitializerClause<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        if let Some(initializer) = self.initializer {
            write!(f, [space(), initializer.format()])
        } else {
            Ok(())
        }
    }
}

/// Formats the body of a statement where it can either be a single statement, an empty statement,
/// or a block statement.
pub(crate) struct FormatStatementBody<'a> {
    body: &'a AnyMStatement,
}

impl<'a> FormatStatementBody<'a> {
    pub fn new(body: &'a AnyMStatement) -> Self {
        Self { body }
    }
}

impl Format<MFormatContext> for FormatStatementBody<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        use AnyMStatement::*;

        if let MEmptyStatement(empty) = &self.body {
            write!(f, [empty.format()])
        } else if !matches!(&self.body, MBlockStatement(_)) {
            write!(f, [block_indent(&self.body.format())])
        } else {
            write!(f, [&format_args![hard_line_break(), self.body.format()]])
        }
    }
}

pub(crate) type FormatStatementSemicolon<'a> = FormatOptionalSemicolon<'a>;

/// Formats a semicolon in a position where it is optional (not needed to maintain syntactical correctness).
///
/// * Inserts a new semicolon if it is absent and [MFormatOptions::semicolons] is [Semicolons::Always].
/// * Removes the semicolon if it is present and [MFormatOptions::semicolons] is [Semicolons::AsNeeded].
pub(crate) struct FormatOptionalSemicolon<'a> {
    semicolon: Option<&'a MSyntaxToken>,
}

impl<'a> FormatOptionalSemicolon<'a> {
    pub(crate) fn new(semicolon: Option<&'a MSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<MFormatContext> for FormatOptionalSemicolon<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match f.options().semicolons() {
            Semicolons::Always => FormatSemicolon::new(self.semicolon).fmt(f),
            Semicolons::AsNeeded => match self.semicolon {
                None => Ok(()),
                Some(semicolon) => format_removed(semicolon).fmt(f),
            },
        }
    }
}

/// Format some code followed by an optional semicolon.
/// Performs semicolon insertion if it is missing in the input source, the [semicolons option](crate::MFormatOptions::semicolons) is [Semicolons::Always], and the
/// preceding element isn't an bogus node
pub(crate) struct FormatSemicolon<'a> {
    semicolon: Option<&'a MSyntaxToken>,
}

impl<'a> FormatSemicolon<'a> {
    pub fn new(semicolon: Option<&'a MSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<MFormatContext> for FormatSemicolon<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self.semicolon {
            Some(semicolon) => semicolon.format().fmt(f),
            None => {
                let is_after_bogus = f.elements().start_tag(TagKind::Verbatim).is_some_and(|signal| match signal {
                        Tag::StartVerbatim(kind) => kind.is_bogus(),
                        _ => unreachable!(),
                    });

                if !is_after_bogus {
                    write!(f, [text(";")])?;
                }

                Ok(())
            }
        }
    }
}

/// A call like expression is one of:
///
/// - [MNewExpression]
/// - [MImportCallExpression]
/// - [MCallExpression]
pub(crate) fn is_call_like_expression(expression: &AnyMExpression) -> bool {
    matches!(
        expression,
        AnyMExpression::MNewExpression(_) | AnyMExpression::MCallExpression(_)
    )
}

/// This function is in charge to format the call arguments.
pub(crate) fn write_arguments_multi_line<S: Format<MFormatContext>, I>(
    separated: I,
    f: &mut MFormatter,
) -> FormatResult<()>
where
    I: Iterator<Item = S>,
{
    let mut iterator = separated.peekable();
    let mut join_with = f.join_with(soft_line_break_or_space());

    while let Some(element) = iterator.next() {
        let last = iterator.peek().is_none();

        if last {
            join_with.entry(&format_args![&element, FormatTrailingCommas::All]);
        } else {
            join_with.entry(&element);
        }
    }

    join_with.finish()
}
