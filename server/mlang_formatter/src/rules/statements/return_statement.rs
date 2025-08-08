use crate::prelude::*;
use crate::utils::{FormatOptionalSemicolon, FormatStatementSemicolon};

use biome_formatter::{CstFormatContext, format_args, write};
use biome_rowan::{SyntaxResult, declare_node_union};
use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::expression_left_side::AnyMExpressionLeftSide;
use mlang_syntax::{
    AnyMExpression, MReturnStatement, MSequenceExpression, MSyntaxToken, MThrowStatement,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMReturnStatement;
impl_format_with_rule!(MReturnStatement, FormatMReturnStatement);

impl FormatNodeRule<MReturnStatement> for FormatMReturnStatement {
    fn fmt_fields(&self, node: &MReturnStatement, f: &mut MFormatter) -> FormatResult<()> {
        AnyMStatementWithArgument::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &MReturnStatement, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `MAnyStatementWithArgument`
        Ok(())
    }
}

declare_node_union! {
    pub(super) AnyMStatementWithArgument = MThrowStatement | MReturnStatement
}

impl Format<MFormatContext> for AnyMStatementWithArgument {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        write!(f, [self.operation_token().format()])?;

        let argument = self.argument()?;

        if let Some(semicolon) = self.semicolon_token() {
            if let Some(argument) = argument {
                write!(f, [space(), FormatReturnOrThrowArgument(&argument)])?;
            }

            let comments = f.context().comments();
            let has_dangling_comments = comments.has_dangling_comments(self.syntax());

            let is_last_comment_line = comments
                .trailing_comments(self.syntax())
                .last()
                .or_else(|| comments.dangling_comments(self.syntax()).last())
                .is_some_and(|comment| comment.kind().is_line());

            if is_last_comment_line {
                write!(f, [FormatOptionalSemicolon::new(Some(&semicolon))])?;
            }

            if has_dangling_comments {
                write!(f, [space(), format_dangling_comments(self.syntax())])?;
            }

            if !is_last_comment_line {
                write!(f, [FormatOptionalSemicolon::new(Some(&semicolon))])?;
            }

            Ok(())
        } else {
            if let Some(argument) = &argument {
                write!(f, [space(), FormatReturnOrThrowArgument(argument)])?;
            }

            write!(f, [FormatStatementSemicolon::new(None)])
        }
    }
}

impl AnyMStatementWithArgument {
    fn operation_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMStatementWithArgument::MThrowStatement(throw) => throw.throw_token(),
            AnyMStatementWithArgument::MReturnStatement(ret) => ret.return_token(),
        }
    }

    fn argument(&self) -> SyntaxResult<Option<AnyMExpression>> {
        match self {
            AnyMStatementWithArgument::MThrowStatement(throw) => throw.argument().map(Some),
            AnyMStatementWithArgument::MReturnStatement(ret) => Ok(ret.argument()),
        }
    }

    fn semicolon_token(&self) -> Option<MSyntaxToken> {
        match self {
            AnyMStatementWithArgument::MThrowStatement(throw) => throw.semicolon_token(),
            AnyMStatementWithArgument::MReturnStatement(ret) => ret.semicolon_token(),
        }
    }
}

pub(super) struct FormatReturnOrThrowArgument<'a>(&'a AnyMExpression);

impl Format<MFormatContext> for FormatReturnOrThrowArgument<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let argument = self.0;
        let is_suppressed = f.comments().is_suppressed(argument.syntax());

        if has_argument_leading_comments(argument, f.context().comments()) && !is_suppressed {
            write!(f, [text("("), &block_indent(&argument.format()), text(")")])
        } else if is_binary_or_sequence_argument(argument) && !is_suppressed {
            write!(
                f,
                [group(&format_args![
                    if_group_breaks(&text("(")),
                    soft_block_indent(&argument.format()),
                    if_group_breaks(&text(")"))
                ])]
            )
        } else {
            write!(f, [argument.format()])
        }
    }
}

/// Tests if the passed in argument has any leading comments. This is the case if
/// * the argument has any leading comment
/// * the argument's left side has any leading comment (see [get_expression_left_side]).
///
/// Traversing the left nodes is necessary in case the first node is parenthesized because
/// parentheses will be removed (and be re-added by the return statement, but only if the argument breaks)
fn has_argument_leading_comments(argument: &AnyMExpression, comments: &MComments) -> bool {
    let mut current: Option<AnyMExpressionLeftSide> = Some(argument.clone().into());

    while let Some(expression) = current {
        if comments.has_leading_own_line_comment(expression.syntax()) {
            return true;
        }

        if comments
            .leading_comments(argument.syntax())
            .iter()
            .any(|comment| comment.piece().has_newline())
        {
            return true;
        };

        current = expression.left_expression();
    }

    false
}

fn is_binary_or_sequence_argument(argument: &AnyMExpression) -> bool {
    MSequenceExpression::can_cast(argument.syntax().kind())
        || AnyMBinaryLikeExpression::can_cast(argument.syntax().kind())
}
