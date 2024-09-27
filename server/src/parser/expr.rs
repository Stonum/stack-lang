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
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq)]
pub enum Expr {
    Error,
    Value(Value),
    Ident(String),
    Call(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    Binary(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
    UnaryRight(Box<Spanned<Self>>, UnaryOp),
    UnaryLeft(UnaryOp, Box<Spanned<Self>>),
    Parentheses(Box<Spanned<Self>>),
    Arr(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Obj(Vec<(String, Spanned<Self>)>),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Error => f.write_str("Error"),
            Expr::Value(f0) => write!(f, "Value({f0:?})"),
            Expr::Ident(f0) => write!(f, "Ident({f0})"),
            Expr::Call(f0, f1) => write!(f, "Call({f0:?}, {f1:?})"),
            Expr::Binary(f0, f1, f2) => write!(f, "Binary({f0:?}, {f1:?}, {f2:?})"),
            Expr::UnaryRight(f0, f1) => write!(f, "UnaryRight({f0:?}, {f1:?})"),
            Expr::UnaryLeft(f0, f1) => write!(f, "UnaryLeft({f0:?}, {f1:?})"),
            Expr::Parentheses(f0) => write!(f, "Parentheses({f0:?})"),
            Expr::Arr(f0) => write!(f, "Arr({f0:?})"),
            Expr::Set(f0) => write!(f, "Set({f0:?})"),
            Expr::Obj(f0) => write!(f, "Obj({f0:?})"),
        }
    }
}

pub(crate) fn parser_expr<'source, I>(
) -> impl Parser<'source, I, Spanned<Expr>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let ident =
            select! { Token::Identifier(ident) => ident.to_string() }.labelled("identifier");

        let inline_expr = recursive(|inline_expr| {
            let val = select! {
                Token::Null(s) => Expr::Value(Value::Null(s.to_string())),
                Token::Bool(s) => Expr::Value(Value::Bool(s.to_string())),
                Token::Number(n) => Expr::Value(Value::Num(n.parse().unwrap())),
                Token::String(s) => Expr::Value(Value::Str(s.to_string())),
                Token::LongString(s) => Expr::Value(Value::LongStr(s.to_string())),
            }
            .labelled("value");

            let unary = {
                let unary_op = select! {
                    Token::Op("++") => UnaryOp::Add,
                    Token::Op("--") => UnaryOp::Sub,
                };

                let unary_expr = ident.clone().map(Expr::Ident).or(val.clone());
                let unary_left = unary_op
                    .then(unary_expr.clone())
                    .map_with(|(op, expr), e| Expr::UnaryLeft(op, Box::new((expr, e.span()))));
                let unary_right = unary_expr
                    .then(unary_op)
                    .map_with(|(expr, op), e| Expr::UnaryRight(Box::new((expr, e.span())), op));

                unary_right.or(unary_left)
            };

            // A list key values
            let items = ident
                .clone()
                .then_ignore(just(Token::Colon))
                .then(inline_expr.clone())
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<_>>()
                .boxed();

            // An object literal
            let obj = items
                .map(Expr::Obj)
                .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("{"),
                    Token::Ctrl("}"),
                    [
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("("), Token::Ctrl(")")),
                    ],
                    |_| Expr::Error,
                )));

            //stack object
            let obj = just(Token::At).ignore_then(obj);

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
                .delimited_by(just(Token::Ctrl("[")), just(Token::Ctrl("]")))
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("["),
                    Token::Ctrl("]"),
                    [
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("("), Token::Ctrl(")")),
                    ],
                    |_| Expr::Error,
                )));

            // stack array
            let arr = just(Token::At).ignore_then(arr);

            // set
            let set = items
                .clone()
                .map(Expr::Set)
                .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("("),
                    Token::Ctrl(")"),
                    [
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("("), Token::Ctrl(")")),
                    ],
                    |_| Expr::Error,
                )));

            // stack set
            let set = just(Token::At).ignore_then(set);

            // 'Atoms' are expressions that contain no ambiguity
            let atom = unary
                .or(val)
                .or(ident.map(Expr::Ident))
                .or(arr)
                .or(obj)
                .or(set)
                .map_with(|expr, e| (expr, e.span()))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                    .map_with(|expr, e| (Expr::Parentheses(Box::new(expr)), e.span())))
                // Attempt to recover anything that looks like a parenthesised expression but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("("),
                    Token::Ctrl(")"),
                    [
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                        (Token::Ctrl("("), Token::Ctrl(")")),
                    ],
                    |span| (Expr::Error, span),
                )))
                // Attempt to recover anything that looks like a list but contains errors
                .recover_with(via_parser(nested_delimiters(
                    Token::Ctrl("["),
                    Token::Ctrl("]"),
                    [
                        (Token::Ctrl("["), Token::Ctrl("]")),
                        (Token::Ctrl("("), Token::Ctrl(")")),
                        (Token::Ctrl("{"), Token::Ctrl("}")),
                    ],
                    |span| (Expr::Error, span),
                )))
                .boxed();

            // Function calls have very high precedence so we prioritise them
            let call = atom
                .foldl_with(
                    items
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                        .map_with(|args, e| (args, e.span()))
                        .repeated(),
                    |f, args, e| (Expr::Call(Box::new(f), args), e.span()),
                )
                .boxed();

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op("*"))
                .to(BinaryOp::Mul)
                .or(just(Token::Op("/")).to(BinaryOp::Div))
                .boxed();
            let product = call
                .clone()
                .foldl_with(op.then(call).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op("+"))
                .to(BinaryOp::Add)
                .or(just(Token::Op("-")).to(BinaryOp::Sub))
                .boxed();
            let sum = product
                .clone()
                .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Add equal and other op
            let op = just(Token::Op("+="))
                .to(BinaryOp::AddEq)
                .or(just(Token::Op("-=")).to(BinaryOp::SubEq))
                .or(just(Token::Op("*=")).to(BinaryOp::MulEq))
                .or(just(Token::Op("/=")).to(BinaryOp::DivEq))
                .boxed();
            let res = sum
                .clone()
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Comparison ops (equal, not-equal) have equal precedence
            let op = just(Token::CondOp("=="))
                .to(BinaryOp::Eq)
                .or(just(Token::CondOp("!=")).to(BinaryOp::NotEq))
                .or(just(Token::CondOp("<")).to(BinaryOp::Lt))
                .or(just(Token::CondOp("<=")).to(BinaryOp::LtEq))
                .or(just(Token::CondOp(">")).to(BinaryOp::Gt))
                .or(just(Token::CondOp(">=")).to(BinaryOp::GtEq))
                .boxed();
            let compare = res
                .clone()
                .foldl_with(op.then(res).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            compare.labelled("expression")
        });

        inline_expr
    })
}

#[cfg(test)]
mod tests {
    use super::super::token_stream_from_str;
    use super::*;
    use super::{BinaryOp::*, Expr::*, Value::*};

    #[inline]
    fn span(range: std::ops::Range<usize>) -> SimpleSpan {
        SimpleSpan::from(range)
    }

    #[test]
    fn test_parse_simple_expr() {
        let source = r#"x + (y - 5) * 6"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((Ident("x".to_string()), span(0..1))),
                Add,
                Box::new((
                    Binary(
                        Box::new((
                            Parentheses(Box::new((
                                Binary(
                                    Box::new((Ident("y".to_string()), span(5..6))),
                                    Sub,
                                    Box::new((Value(Num(5.0)), span(9..10))),
                                ),
                                span(5..10),
                            ))),
                            span(4..11),
                        )),
                        Mul,
                        Box::new((Value(Num(6.0)), span(14..15))),
                    ),
                    span(4..15),
                )),
            ),
            span(0..15),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_unary_expr() {
        let source = r#"x++ - 5++"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((
                    UnaryRight(Box::new((Ident("x".to_string()), span(0..3))), UnaryOp::Add),
                    span(0..3),
                )),
                Sub,
                Box::new((
                    UnaryRight(Box::new((Value(Num(5.0)), span(6..9))), UnaryOp::Add),
                    span(6..9),
                )),
            ),
            span(0..9),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_add_eq_expr() {
        let source = r#"x += 5 - 10"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((Ident("x".to_string()), span(0..1))),
                AddEq,
                Box::new((
                    Binary(
                        Box::new((Value(Num(5.0)), span(5..6))),
                        Sub,
                        Box::new((Value(Num(10.0)), span(9..11))),
                    ),
                    span(5..11),
                )),
            ),
            span(0..11),
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
                (Value(Num(1.0)), span(2..3)),
                (Value(Null("null".to_string())), span(5..9)),
                (Value(Str("hello".to_string())), span(11..18)),
                (Value(Num(5.55)), span(20..24)),
                (Value(Bool("true".to_string())), span(26..30)),
                (Ident("x".to_string()), span(32..33)),
            ]),
            span(0..34),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_set() {
        let source = r#"@(1, null, "hello", 5.55, true, x)"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Expr::Set(vec![
                (Value(Num(1.0)), span(2..3)),
                (Value(Null("null".to_string())), span(5..9)),
                (Value(Str("hello".to_string())), span(11..18)),
                (Value(Num(5.55)), span(20..24)),
                (Value(Bool("true".to_string())), span(26..30)),
                (Ident("x".to_string()), span(32..33)),
            ]),
            span(0..34),
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
                ("a".to_string(), (Value(Num(1.0)), span(5..6))),
                (
                    "b".to_string(),
                    (Value(Null("null".to_string())), span(11..15)),
                ),
                (
                    "c".to_string(),
                    (Value(Str("hello".to_string())), span(20..27)),
                ),
                ("d".to_string(), (Value(Num(5.55)), span(32..36))),
                (
                    "e".to_string(),
                    (Value(Bool("true".to_string())), span(41..45)),
                ),
            ]),
            span(0..46),
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
                Box::new((Ident("sum".to_string()), span(0..3))),
                (
                    vec![
                        (Ident("x".to_string()), span(4..5)),
                        (Value(Num(5.0)), span(7..8)),
                    ],
                    span(3..9),
                ),
            ),
            span(0..9),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_call_method() {
        let source = r#"x.sum(x, 5)"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Call(
                Box::new((Ident("sum".to_string()), span(0..3))),
                (
                    vec![
                        (Ident("x".to_string()), span(4..5)),
                        (Value(Num(5.0)), span(7..8)),
                    ],
                    span(3..9),
                ),
            ),
            span(0..9),
        ));
        assert_eq!(parsed, expected);
    }
}
