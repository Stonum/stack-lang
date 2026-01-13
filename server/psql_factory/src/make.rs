use std::fmt::Display;

use biome_rowan::TriviaPiece;
use psql_syntax::{
    AnyPsqlExpression, PsqlParenthesizedExpression, PsqlSyntaxKind, PsqlSyntaxToken,
};

pub use super::generated::node_factory::*;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> PsqlSyntaxToken {
    PsqlSyntaxToken::new_detached(PsqlSyntaxKind::IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn psql_string_literal(text: &str) -> PsqlSyntaxToken {
    PsqlSyntaxToken::new_detached(
        PsqlSyntaxKind::PSQL_STRING_LITERAL,
        &format!("'{text}'"),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia
pub fn psql_number_literal<N>(text: N) -> PsqlSyntaxToken
where
    N: Display + Copy,
{
    PsqlSyntaxToken::new_detached(
        PsqlSyntaxKind::PSQL_NUMBER_LITERAL,
        &text.to_string(),
        [],
        [],
    )
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: PsqlSyntaxKind) -> PsqlSyntaxToken {
    if let Some(text) = kind.to_string() {
        PsqlSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: PsqlSyntaxKind) -> PsqlSyntaxToken {
    if let Some(text) = kind.to_string() {
        PsqlSyntaxToken::new_detached(
            kind,
            &format!(" {text} "),
            [TriviaPiece::whitespace(1)],
            [TriviaPiece::whitespace(1)],
        )
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// EOF token
pub fn eof() -> PsqlSyntaxToken {
    PsqlSyntaxToken::new_detached(PsqlSyntaxKind::EOF, "", [], [])
}

/// Wrap `expr` in a new parenthesized expression
pub fn parenthesized(expr: impl Into<AnyPsqlExpression>) -> PsqlParenthesizedExpression {
    psql_parenthesized_expression(
        token(PsqlSyntaxKind::L_PAREN),
        expr.into(),
        token(PsqlSyntaxKind::R_PAREN),
    )
}
