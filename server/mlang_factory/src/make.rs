use std::fmt::Display;

use biome_rowan::TriviaPiece;
use mlang_syntax::{AnyMExpression, MParenthesizedExpression, MSyntaxKind, MSyntaxToken};

pub use super::generated::node_factory::*;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> MSyntaxToken {
    MSyntaxToken::new_detached(MSyntaxKind::IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn m_string_literal(text: &str) -> MSyntaxToken {
    MSyntaxToken::new_detached(
        MSyntaxKind::M_STRING_LITERAL,
        &format!("\"{text}\""),
        [],
        [],
    )
}

/// Create a new string literal token with no attached trivia, using single quotes
pub fn m_long_string_literal(text: &str) -> MSyntaxToken {
    MSyntaxToken::new_detached(MSyntaxKind::M_STRING_LITERAL, &format!("'{text}'"), [], [])
}

/// Create a new string literal token with no attached trivia
pub fn m_number_literal<N>(text: N) -> MSyntaxToken
where
    N: Display + Copy,
{
    MSyntaxToken::new_detached(MSyntaxKind::M_NUMBER_LITERAL, &text.to_string(), [], [])
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: MSyntaxKind) -> MSyntaxToken {
    if let Some(text) = kind.to_string() {
        MSyntaxToken::new_detached(kind, text, [], [])
    } else {
        panic!("token kind {kind:?} cannot be transformed to text")
    }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: MSyntaxKind) -> MSyntaxToken {
    if let Some(text) = kind.to_string() {
        MSyntaxToken::new_detached(
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
pub fn eof() -> MSyntaxToken {
    MSyntaxToken::new_detached(MSyntaxKind::EOF, "", [], [])
}

/// Wrap `expr` in a new parenthesized expression
pub fn parenthesized(expr: impl Into<AnyMExpression>) -> MParenthesizedExpression {
    m_parenthesized_expression(
        token(MSyntaxKind::L_PAREN),
        expr.into(),
        token(MSyntaxKind::R_PAREN),
    )
}
