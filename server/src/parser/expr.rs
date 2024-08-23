use chumsky::{input::ValueInput, prelude::*};

use super::Token;
use super::{Span, Spanned};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null(String),
    Bool(String),
    Num(f64),
    Str(String),
    LongStr(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Error,
    Value(Value),
    Ident(String),
    Call(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    Arr(Vec<Spanned<Self>>),
    Obj(Vec<(String, Spanned<Self>)>),
}

pub(crate) fn parser_expr<'source, I>(
) -> impl Parser<'source, I, Spanned<Expr>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let ident = select! { Token::Identifier(ident) => ident.to_string() };

        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Null(s) => Expr::Value(Value::Null(s.to_string())),
                Token::Bool(s) => Expr::Value(Value::Bool(s.to_string())),
                Token::Number(n) => Expr::Value(Value::Num(n.parse().unwrap())),
                Token::String(s) => Expr::Value(Value::Str(s.to_string())),
                Token::LongString(s) => Expr::Value(Value::LongStr(s.to_string())),
            };

            // An object literal
            let obj = ident
                .clone()
                .then_ignore(just(Token::Colon))
                .then(inline_expr.clone())
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .map(Expr::Obj)
                .delimited_by(just(Token::ObjectLbracket), just(Token::Ctrl("}")));

            // A list of expressions
            let items = expr
                .clone()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<_>>();

            // An array
            let arr = items
                .clone()
                .map(Expr::Arr)
                .delimited_by(just(Token::ArrayLbracket), just(Token::Ctrl("]")));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(ident.map(Expr::Ident))
                .or(arr)
                .or(obj)
                .map_with(|expr, e| (expr, e.span()))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("("),
                    Token::Ctrl(")"),
                    [
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                    ],
                    |span| (Expr::Error, span),
                )))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("["),
                    Token::Ctrl("]"),
                    [
                        (Token::Ctrl("("), Token::Ctrl(")")),
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                    ],
                    |span| (Expr::Error, span),
                )))
                .boxed();

            // Function calls have very high precedence so we prioritise them
            let call = atom.foldl_with(
                items
                    .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                    .map_with(|args, e| (args, e.span()))
                    .repeated(),
                |f, args, e| (Expr::Call(Box::new(f), args), e.span()),
            );

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*"))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/")).to(BinaryOp::Div));
            let product = call
                .clone()
                .foldl_with(op.then(call).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+"))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-")).to(BinaryOp::Sub));
            let sum = product
                .clone()
                .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::CondOp("=="))
                .to(BinaryOp::Eq)
                .or(just(Token::CondOp("!=")).to(BinaryOp::NotEq))
                .or(just(Token::CondOp("<")).to(BinaryOp::Lt))
                .or(just(Token::CondOp("<=")).to(BinaryOp::LtEq))
                .or(just(Token::CondOp(">")).to(BinaryOp::Gt))
                .or(just(Token::CondOp(">=")).to(BinaryOp::GtEq));
            let compare = sum
                .clone()
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                });

            compare
        });

        inline_expr
    })
}

#[cfg(test)]
mod tests {
    use super::super::token_stream_from_str;
    use super::*;
    use super::{BinaryOp::*, Expr::*, Value::*};

    #[test]
    fn test_parse_simple_expr() {
        let source = r#"x + y - 5"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((
                    Binary(
                        Box::new((Ident("x".to_string()), SimpleSpan::from(0..1))),
                        Add,
                        Box::new((Ident("y".to_string()), SimpleSpan::from(4..5))),
                    ),
                    SimpleSpan::from(0..5),
                )),
                Sub,
                Box::new((Value(Num(5.0)), SimpleSpan::from(8..9))),
            ),
            SimpleSpan::from(0..9),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_array() {
        let source = r#"@[1, null, "hello", 5.55, true, x]"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Expr::Arr(vec![
                (Value(Num(1.0)), SimpleSpan::from(2..3)),
                (Value(Null("null".to_string())), SimpleSpan::from(5..9)),
                (Value(Str("hello".to_string())), SimpleSpan::from(11..18)),
                (Value(Num(5.55)), SimpleSpan::from(20..24)),
                (Value(Bool("true".to_string())), SimpleSpan::from(26..30)),
                (Ident("x".to_string()), SimpleSpan::from(32..33)),
            ]),
            SimpleSpan::from(0..34),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_simple_object() {
        let source = r#"@{a: 1, b: null, c: "hello", d: 5.55, e: true}"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Expr::Obj(vec![
                ("a".to_string(), (Value(Num(1.0)), SimpleSpan::from(5..6))),
                (
                    "b".to_string(),
                    (Value(Null("null".to_string())), SimpleSpan::from(11..15)),
                ),
                (
                    "c".to_string(),
                    (Value(Str("hello".to_string())), SimpleSpan::from(20..27)),
                ),
                (
                    "d".to_string(),
                    (Value(Num(5.55)), SimpleSpan::from(32..36)),
                ),
                (
                    "e".to_string(),
                    (Value(Bool("true".to_string())), SimpleSpan::from(41..45)),
                ),
            ]),
            SimpleSpan::from(0..46),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_call_function() {
        let source = r#"sum(x, 5)"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Call(
                Box::new((Ident("sum".to_string()), SimpleSpan::from(0..3))),
                (
                    vec![
                        (Ident("x".to_string()), SimpleSpan::from(4..5)),
                        (Value(Num(5.0)), SimpleSpan::from(7..8)),
                    ],
                    SimpleSpan::from(3..9),
                ),
            ),
            SimpleSpan::from(0..9),
        ));
        assert_eq!(parsed, expected);
    }
}
