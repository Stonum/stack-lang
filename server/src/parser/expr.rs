use crate::lexer::KwLang;

use super::cst::{BinaryOp, Expr, UnaryOp, Value};
use super::cst::{Span, Spanned};
use super::Token;
use chumsky::{input::ValueInput, prelude::*};

pub(crate) fn parser_expr<'source, I>(
) -> impl Parser<'source, I, Spanned<Expr<'source>>, extra::Err<Rich<'source, Token<'source>, Span>>>
       + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let newline = just(Token::NewLine).repeated().or_not();

    recursive(|expr| {
        let ident = select! {
            Token::Identifier(ident) => ident,
            Token::Dot => ".",
        }
        .labelled("identifier");

        let inline_expr = recursive(|_inline_expr| {
            let val = select! {
                Token::Null(s) => Expr::Value(Value::Null(s)),
                Token::Bool(s) => Expr::Value(Value::Bool(s)),
                Token::Number(n) => Expr::Value(Value::Num(n)),
                Token::String(s) => Expr::Value(Value::Str(s)),
                Token::LongString(s) => Expr::Value(Value::LongStr(s)),
            }
            .labelled("value");

            let unary = {
                let unary_op = select! {
                    Token::Op("++") => UnaryOp::Add,
                    Token::Op("--") => UnaryOp::Sub,
                    Token::CondOp("!") => UnaryOp::Not,
                };

                let unary_expr = ident.clone().map(Expr::Ident).or(val.clone());
                let unary_left = unary_op
                    .then(unary_expr.clone())
                    .map_with(|(op, expr), e| Expr::UnaryLeft(op, Box::new((expr, e.span()))));
                let unary_right = unary_expr
                    .then(unary_op)
                    .map_with(|(expr, op), e| Expr::UnaryRight(Box::new((expr, e.span())), op));

                let unary_minus = {
                    let unary_op = select! {
                        Token::Op("-") => UnaryOp::Minus,
                    };

                    let unary_expr = ident.clone().map(Expr::Ident).or(val.clone());
                    let unary_left = unary_op
                        .then(unary_expr.clone())
                        .map_with(|(op, expr), e| Expr::UnaryLeft(op, Box::new((expr, e.span()))));

                    unary_left
                };

                unary_right.or(unary_left).or(unary_minus)
            };

            let obj = {
                let ident = select! {
                    Token::Identifier(ident) => ident,
                    Token::String(s) => s,
                    Token::LongString(s) => s,
                    Token::Number(n) => n,
                }
                .labelled("identifier");

                // A list key values
                let items = ident
                    .clone()
                    .padded_by(newline.clone())
                    .then_ignore(just(Token::Colon))
                    .then(expr.clone().padded_by(newline.clone()))
                    .separated_by(just(Token::Comma).padded_by(newline.clone()))
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
                just(Token::At).ignore_then(obj)
            };

            // A list of expressions
            let items = expr
                .clone()
                .padded_by(newline.clone())
                .separated_by(just(Token::Comma).padded_by(newline.clone()))
                .allow_trailing()
                .collect::<Vec<_>>();

            let arr = {
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
                just(Token::At).ignore_then(arr)
            };

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

            let bracket = ident
                .clone()
                .padded_by(newline.clone())
                .then(
                    items
                        .clone()
                        .delimited_by(just(Token::Ctrl("[")), just(Token::Ctrl("]"))),
                )
                .map_with(|(a, b), e| Expr::IndexKey(Box::new((Expr::Ident(a), e.span())), b));

            // 'Atoms' are expressions that contain no ambiguity
            let atom = unary
                .or(val)
                .or(bracket)
                .or(ident.map(Expr::Ident))
                .or(arr)
                .or(obj)
                .or(set)
                .padded_by(newline.clone())
                .map_with(|expr, e| (expr, e.span()))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .padded_by(newline.clone())
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
            let op = select! {
                Token::Op("*") => BinaryOp::Mul,
                Token::Op("/") => BinaryOp::Div,
                Token::Op("%") => BinaryOp::Mod,
                Token::Op("&") => BinaryOp::BitAnd,
                Token::Op("|") => BinaryOp::BitOr,
            };
            let product = call
                .clone()
                .foldl_with(op.then(call).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Sum ops (add and subtract) have equal precedence
            let op = select! {
                Token::Op("+") => BinaryOp::Add,
                Token::Op("-") => BinaryOp::Sub,
            };
            let sum = product
                .clone()
                .foldl_with(op.then(product).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Add equal and other op
            let op = select! {
                Token::Op("+=") => BinaryOp::AddEq,
                Token::Op("-=") => BinaryOp::SubEq,
                Token::Op("*=") => BinaryOp::MulEq,
                Token::Op("/=") => BinaryOp::DivEq,
            };
            let res = sum
                .clone()
                .foldl_with(op.then(sum).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            // Comparison ops (equal, not-equal) have equal precedence
            let op = select! {
                Token::CondOp("==") => BinaryOp::Eq,
                Token::CondOp("!=") => BinaryOp::NotEq,
                Token::CondOp("<") => BinaryOp::Lt,
                Token::CondOp("<=") => BinaryOp::LtEq,
                Token::CondOp(">") => BinaryOp::Gt,
                Token::CondOp(">=") => BinaryOp::GtEq,
            };
            let compare = res
                .clone()
                .foldl_with(op.then(res).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            let op = select! {
                Token::And(s) => BinaryOp::And(s),
                Token::Or(s) => BinaryOp::Or(s),
            };
            let logical = compare
                .clone()
                .foldl_with(op.then(compare).repeated(), |a, (op, b), e| {
                    (Expr::Binary(Box::new(a), op, Box::new(b)), e.span())
                })
                .boxed();

            logical.labelled("expression")
        });

        // if keyword is used as an identifier
        let keyword_as_ident = select! {
            Token::Function(s) => s,
        }
        .map_with(|s, e| {
            (
                Expr::Ident(if s == KwLang::Ru {
                    "функция"
                } else {
                    "function"
                }),
                e.span(),
            )
        });

        let expr_chain = inline_expr
            .clone()
            .foldl_with(
                just(Token::Dot)
                    .ignore_then(inline_expr.clone().or(keyword_as_ident))
                    .repeated(),
                |a, b, e| (Expr::Then(Box::new(a), Box::new(b)), e.span()),
            )
            .boxed();

        let tern = expr_chain
            .clone()
            .then_ignore(just(Token::QuestionMark))
            .then(expr_chain.clone())
            .then_ignore(just(Token::Colon))
            .then(expr_chain.clone())
            .map_with(|((a, b), c), e| {
                (
                    Expr::Ternary(Box::new(a), Box::new(b), Box::new(c)),
                    e.span(),
                )
            })
            .boxed();

        let expr_chain = tern.or(expr_chain);

        let expr_eq = expr_chain
            .clone()
            .foldl_with(
                just(Token::Equals)
                    .ignore_then(expr_chain.clone())
                    .repeated(),
                |a, b, e| (Expr::ThenEquals(Box::new(a), Box::new(b)), e.span()),
            )
            .boxed();

        expr_eq
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
        let source = r#"-x + (y - 5) * 6"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((
                    UnaryLeft(
                        UnaryOp::Minus,
                        Box::new((Ident("x".to_string()), span(0..2))),
                    ),
                    span(0..2),
                )),
                Add,
                Box::new((
                    Binary(
                        Box::new((
                            Parentheses(Box::new((
                                Binary(
                                    Box::new((Ident("y".to_string()), span(6..7))),
                                    Sub,
                                    Box::new((Value(Num("5".to_string())), span(10..11))),
                                ),
                                span(6..11),
                            ))),
                            span(5..12),
                        )),
                        Mul,
                        Box::new((Value(Num("6".to_string())), span(15..16))),
                    ),
                    span(5..16),
                )),
            ),
            span(0..16),
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
                    UnaryRight(
                        Box::new((Value(Num("5".to_string())), span(6..9))),
                        UnaryOp::Add,
                    ),
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
                        Box::new((Value(Num("5".to_string())), span(5..6))),
                        Sub,
                        Box::new((Value(Num("10".to_string())), span(9..11))),
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
                (Value(Num("1".to_string())), span(2..3)),
                (Value(Null("null".to_string())), span(5..9)),
                (Value(Str("hello".to_string())), span(11..18)),
                (Value(Num("5.55".to_string())), span(20..24)),
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
                (Value(Num("1".to_string())), span(2..3)),
                (Value(Null("null".to_string())), span(5..9)),
                (Value(Str("hello".to_string())), span(11..18)),
                (Value(Num("5".to_string())), span(20..24)),
                (Value(Bool("true".to_string())), span(26..30)),
                (Ident("x".to_string()), span(32..33)),
            ]),
            span(0..34),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_simple_object() {
        let source = r#"@{a: 1, b: null, c: "hello", d: 5.55, "e": true}"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Expr::Obj(vec![
                ("a".to_string(), (Value(Num("1".to_string())), span(5..6))),
                (
                    "b".to_string(),
                    (Value(Null("null".to_string())), span(11..15)),
                ),
                (
                    "c".to_string(),
                    (Value(Str("hello".to_string())), span(20..27)),
                ),
                (
                    "d".to_string(),
                    (Value(Num("5.55".to_string())), span(32..36)),
                ),
                (
                    "e".to_string(),
                    (Value(Bool("true".to_string())), span(43..47)),
                ),
            ]),
            span(0..48),
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
                        (Value(Num("5".to_string())), span(7..8)),
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
            Then(
                Box::new((Ident("x".to_string()), span(0..1))),
                Box::new((
                    Call(
                        Box::new((Ident("sum".to_string()), span(2..5))),
                        (
                            vec![
                                (Ident("x".to_string()), span(6..7)),
                                (Value(Num("5".to_string())), span(9..10)),
                            ],
                            span(5..11),
                        ),
                    ),
                    span(2..11),
                )),
            ),
            span(0..11),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_tern() {
        let source = r#"x = y < 3 ? 5 : 10"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            ThenEquals(
                Box::new((Ident("x".to_string()), span(0..1))),
                Box::new((
                    Ternary(
                        Box::new((
                            Binary(
                                Box::new((Ident("y".to_string()), span(4..5))),
                                Lt,
                                Box::new((Value(Num("3".to_string())), span(8..9))),
                            ),
                            span(4..9),
                        )),
                        Box::new((Value(Num("5".to_string())), span(12..13))),
                        Box::new((Value(Num("10".to_string())), span(16..18))),
                    ),
                    span(4..18),
                )),
            ),
            span(0..18),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_bracket_notation() {
        let source = r#"params[10, 10]"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            IndexKey(
                Box::new((Ident("params".to_string()), span(0..14))),
                vec![
                    (Value(Num("10".to_string())), span(7..9)),
                    (Value(Num("10".to_string())), span(11..13)),
                ],
            ),
            span(0..14),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_logical_expression() {
        let source = r#"x == 5 && y == 10"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_expr().parse(token_stream).into_result();
        let expected = Ok((
            Binary(
                Box::new((
                    Binary(
                        Box::new((Ident("x".to_string()), span(0..1))),
                        Eq,
                        Box::new((Value(Num("5".to_string())), span(5..6))),
                    ),
                    span(0..6),
                )),
                And("&&".to_string()),
                Box::new((
                    Binary(
                        Box::new((Ident("y".to_string()), span(10..11))),
                        Eq,
                        Box::new((Value(Num("10".to_string())), span(15..17))),
                    ),
                    span(10..17),
                )),
            ),
            span(0..17),
        ));
        assert_eq!(parsed, expected);
    }
}
