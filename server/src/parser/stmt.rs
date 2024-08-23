use chumsky::{input::ValueInput, prelude::*};

use super::{expr::Expr, parser_expr};
use super::{KwLang, Token};
use super::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Spanned<Expr>),
    Var(Option<KwLang>, String, Option<Spanned<Expr>>),
}

pub(crate) fn parser_stmt<'source, I>(
) -> impl Parser<'source, I, Vec<Stmt>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let kw = select! {
        Token::Var(KwLang::Eng) => KwLang::Eng,
        Token::Var(KwLang::Ru) => KwLang::Ru,
    };

    let ident = select! { Token::Identifier(ident) => ident.to_string() };

    let expr = parser_expr()
        .then_ignore(just(Token::SemiColon))
        .map(|e| Stmt::Expr(e));

    let var = kw
        .or_not()
        .then(ident)
        .then_ignore(just(Token::Equals))
        .then(parser_expr())
        .then_ignore(just(Token::SemiColon))
        .map(|((kw, ident), expr)| Stmt::Var(kw, ident, Some(expr)));

    expr.or(var).repeated().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::super::expr::{BinaryOp::*, Expr::*, Value::*};
    use super::super::token_stream_from_str;
    use super::*;

    #[test]
    fn test_parse_simple_stmt() {
        let source = r#"
            перем y = 10.5;
            var z = "hello";
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![
            Stmt::Var(
                Some(KwLang::Ru),
                "y".to_string(),
                Some((Value(Num(10.5)), SimpleSpan::from(28..32))),
            ),
            Stmt::Var(
                Some(KwLang::Eng),
                "z".to_string(),
                Some((Value(Str("hello".to_string())), SimpleSpan::from(54..61))),
            ),
        ]);
        assert_eq!(parsed, expected);
    }
}
