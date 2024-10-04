use chumsky::{input::ValueInput, prelude::*};

use super::expr::{parser_expr, Expr};
use super::{KwLang, Token};
use super::{Span, Spanned};

#[derive(PartialEq)]
pub enum Stmt {
    Error(Spanned<String>),
    Comment(Spanned<String>),
    Expr(Spanned<Expr>),
    Var(Option<KwLang>, String, Option<Spanned<Expr>>),
    Ret(KwLang, Option<Spanned<Expr>>),
    Throw(KwLang, Option<Spanned<Expr>>),
    Block(Vec<Self>),
    If(KwLang, Spanned<Expr>, Box<Self>, Option<Box<Self>>),
    While(KwLang, Spanned<Expr>, Box<Self>),
    ForAll(KwLang, String, Spanned<Expr>, Box<Self>),
    ForAll2(KwLang, String, Spanned<Expr>, String, Box<Self>),
    For(KwLang, Box<Self>, Spanned<Expr>, Box<Self>, Box<Self>),
    Switch(
        KwLang,
        Spanned<Expr>,
        Vec<(Option<Vec<Spanned<Expr>>>, Box<Self>)>,
    ),
    TryCatch(
        KwLang,
        Box<Self>,
        Option<(Option<Spanned<Expr>>, Box<Self>)>,
        Option<Box<Self>>,
    ),
}

impl std::fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::Error(f0) => write!(f, "Error({f0:?})"),
            Stmt::Expr(f0) => write!(f, "Expr({f0:?})"),
            Stmt::Comment(f0) => write!(f, "Comment({f0:?})"),
            Stmt::Var(f0, f1, f2) => write!(f, "Var({f0:?}, {f1}, {f2:?})"),
            Stmt::Ret(f0, f1) => write!(f, "Ret({f0:?}, {f1:?})"),
            Stmt::Throw(f0, f1) => write!(f, "Throw({f0:?}, {f1:?})"),
            Stmt::Block(f0) => {
                f.write_str("Block(")?;
                f.debug_list().entries(f0).finish()?;
                f.write_str(")")
            }
            Stmt::If(f0, f1, f2, f3) => f
                .debug_tuple("If")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
            Stmt::While(f0, f1, f2) => f
                .debug_tuple("While")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .finish(),
            Stmt::ForAll(f0, f1, f2, f3) => f
                .debug_tuple("ForAll")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
            Stmt::ForAll2(f0, f1, f2, f3, f4) => f
                .debug_tuple("ForAll2")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .field(&f4)
                .finish(),
            Stmt::For(f0, f1, f2, f3, f4) => f
                .debug_tuple("For")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .field(&f4)
                .finish(),
            Stmt::Switch(f0, f1, f2) => f
                .debug_tuple("Switch")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .finish(),
            Stmt::TryCatch(f0, f1, f2, f3) => f
                .debug_tuple("TryCatch")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
        }
    }
}

pub(crate) fn parser_stmt<'source, I>(
) -> impl Parser<'source, I, Stmt, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let comment = select! { Token::CommentLine(comment) => comment.to_string() }
        .map_with(|comment, e| Stmt::Comment((comment, e.span())))
        .labelled("comment");

    let ident = select! { Token::Identifier(ident) => ident.to_string() }.labelled("identifier");

    let expr = parser_expr()
        .then_ignore(just(Token::SemiColon).or_not())
        .map(|e| Stmt::Expr(e))
        .labelled("expression");

    let var = {
        let kw = select! {
            Token::Var(KwLang::Eng) => KwLang::Eng,
            Token::Var(KwLang::Ru) => KwLang::Ru,
        };

        let expr = parser_expr().then_ignore(just(Token::SemiColon).or_not());

        let var = kw
            .then(ident)
            .then_ignore(just(Token::SemiColon).or_not())
            .map(|(kw, ident)| Stmt::Var(Some(kw), ident, None))
            .boxed();

        let var_eq = kw
            .then(ident)
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .map(|((kw, ident), expr)| Stmt::Var(Some(kw), ident, Some(expr)))
            .boxed();

        let var_eq2 = ident
            .then_ignore(just(Token::Equals))
            .then(expr.clone())
            .map(|(ident, expr)| Stmt::Var(None, ident, Some(expr)))
            .boxed();

        var_eq.or(var).or(var_eq2).labelled("variable").boxed()
    };

    let ret = {
        let ret_kw = select! {
            Token::Return(KwLang::Eng) => KwLang::Eng,
            Token::Return(KwLang::Ru) => KwLang::Ru,
        };
        ret_kw
            .then(parser_expr().or_not())
            .then_ignore(just(Token::SemiColon).or_not())
            .map(|(kw, expr)| Stmt::Ret(kw, expr))
            .boxed()
            .labelled("return")
    };

    let throw = {
        let kw = select! {
            Token::Throw(KwLang::Eng) => KwLang::Eng,
            Token::Throw(KwLang::Ru) => KwLang::Ru,
        };

        let expr = parser_expr().then_ignore(just(Token::SemiColon).or_not());

        kw.then(expr.clone().or_not())
            .map(|(kw, expr)| Stmt::Throw(kw, expr))
            .boxed()
            .labelled("throw")
    };

    let inline_expr = comment.or(var).or(ret).or(throw).or(expr.clone()).boxed();

    let expr = parser_expr();

    recursive(|block_expr| {
        let block = block_expr
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
            .map(Stmt::Block)
            .recover_with(via_parser(nested_delimiters(
                Token::Ctrl("{"),
                Token::Ctrl("}"),
                [
                    (Token::Ctrl("{"), Token::Ctrl("}")),
                    (Token::Ctrl("["), Token::Ctrl("]")),
                    (Token::Ctrl("("), Token::Ctrl(")")),
                ],
                |span| Stmt::Error((String::from("Error parsing block"), span)),
            )))
            .boxed();

        let _if = recursive(|_if| {
            let if_kw = select! {
                Token::If(KwLang::Eng) => KwLang::Eng,
                Token::If(KwLang::Ru) => KwLang::Ru,
            };

            let else_kw = select! {
                Token::Else(KwLang::Eng) => KwLang::Eng,
                Token::Else(KwLang::Ru) => KwLang::Ru,
            };

            if_kw
                .then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))),
                )
                .then(
                    block
                        .clone()
                        .or(inline_expr.clone().map(|e| Stmt::Block(vec![e]))),
                )
                .then(
                    else_kw
                        .ignore_then(
                            block
                                .clone()
                                .or(inline_expr.clone().map(|e| Stmt::Block(vec![e])))
                                .or(_if),
                        )
                        .or_not(),
                )
                .map(|(((if_kw, expr), block), else_block)| {
                    Stmt::If(if_kw, expr, Box::new(block), else_block.map(Box::new))
                })
                .boxed()
        });

        let _while = {
            let while_kw = select! {
                Token::While(KwLang::Eng) => KwLang::Eng,
                Token::While(KwLang::Ru) => KwLang::Ru,
            };

            while_kw
                .then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))),
                )
                .then(block.clone())
                .map(|((while_kw, expr), block)| Stmt::While(while_kw, expr, Box::new(block)))
                .boxed()
        };

        let _forall = {
            let forall_kw = select! {
                Token::ForAll(KwLang::Eng) => KwLang::Eng,
                Token::ForAll(KwLang::Ru) => KwLang::Ru,
            };

            let in_kw = select! { Token::In(KwLang::Eng) | Token::In(KwLang::Ru) => () };

            let loop_cond_in = ident.clone().then_ignore(in_kw).then(expr.clone());

            let forall = forall_kw
                .then(loop_cond_in.delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))))
                .then(block.clone())
                .map(|((forall_kw, (ident, expr)), block)| {
                    Stmt::ForAll(forall_kw, ident, expr, Box::new(block))
                })
                .boxed();

            let loop_cond_iter = ident
                .clone()
                .then_ignore(just(Token::Ctrl("(")))
                .then(expr.clone())
                .then_ignore(just(Token::Comma))
                .then(ident.clone())
                .then_ignore(just(Token::Ctrl(")")))
                .boxed();

            let forall2 = forall_kw
                .then(loop_cond_iter.delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))))
                .then(block.clone())
                .map(|((forall_kw, ((fabric, expr), ident)), block)| {
                    Stmt::ForAll2(forall_kw, fabric, expr, ident, Box::new(block))
                })
                .boxed();

            forall.or(forall2)
        };

        let _for = {
            let for_kw = select! {
                Token::For(KwLang::Eng) => KwLang::Eng,
                Token::For(KwLang::Ru) => KwLang::Ru,
            };

            let expr = ident
                .then_ignore(just(Token::Equals))
                .then(expr.clone())
                .map(|(ident, expr)| Stmt::Var(None, ident, Some(expr)))
                .boxed();

            let cond = parser_expr().labelled("expression");

            let loop_cond = expr
                .clone()
                .then_ignore(just(Token::SemiColon))
                .then(cond.clone())
                .then_ignore(just(Token::SemiColon))
                .then(expr.clone())
                .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                .boxed();

            for_kw
                .then(loop_cond)
                .then(block.clone())
                .map(|((for_kw, ((init, cond), step)), block)| {
                    Stmt::For(
                        for_kw,
                        Box::new(init),
                        cond,
                        Box::new(step),
                        Box::new(block),
                    )
                })
                .boxed()
        };

        let _switch = {
            let case_kw = select! {
                Token::Case(KwLang::Eng) => KwLang::Eng,
                Token::Case(KwLang::Ru) => KwLang::Ru,
            };

            let block_without_braces = inline_expr
                .clone()
                .repeated()
                .collect::<Vec<_>>()
                .map(Stmt::Block)
                .boxed();

            let block = block.clone().or(block_without_braces);

            let case = case_kw
                .ignore_then(
                    expr.clone()
                        .separated_by(just(Token::Comma))
                        .collect::<Vec<_>>(),
                )
                .then_ignore(just(Token::Colon))
                .then(block.clone())
                .map(|(expr, block)| (Some(expr), Box::new(block)))
                .boxed();

            let default_kw = select! {
                Token::Else(KwLang::Eng) => KwLang::Eng,
                Token::Else(KwLang::Ru) => KwLang::Ru,
            };

            let default = default_kw
                .ignore_then(block.clone())
                .map(|block| (None, Box::new(block)))
                .boxed();

            let switch_kw = select! {
                Token::Switch(KwLang::Eng) => KwLang::Eng,
                Token::Switch(KwLang::Ru) => KwLang::Ru,
            };

            switch_kw
                .then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))),
                )
                .then(
                    case.repeated()
                        .collect::<Vec<_>>()
                        .then(default.or_not())
                        .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
                        .map(|(mut cases, default)| {
                            if let Some(default) = default {
                                cases.push(default);
                            };
                            cases
                        }),
                )
                .map(|((switch_kw, expr), cases)| Stmt::Switch(switch_kw, expr, cases))
                .boxed()
        };

        let _try = {
            let try_kw = select! {
                Token::Try(KwLang::Eng) => KwLang::Eng,
                Token::Try(KwLang::Ru) => KwLang::Ru,
            };

            let catch_kw = select! {
                Token::Catch(KwLang::Eng) => KwLang::Eng,
                Token::Catch(KwLang::Ru) => KwLang::Ru,
            };

            let catch = catch_kw
                .ignore_then(
                    expr.clone()
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")"))),
                )
                .or_not()
                .then(block.clone())
                .map(|(expr, block)| (expr, Box::new(block)));

            let finally_kw = select! {
                Token::Finally(KwLang::Eng) => KwLang::Eng,
                Token::Finally(KwLang::Ru) => KwLang::Ru,
            };

            let finally = finally_kw.ignore_then(block.clone()).map(Box::new);

            try_kw
                .then(block.clone())
                .then(catch.or_not())
                .then(finally.or_not())
                .map(|(((try_kw, block), catch), finally)| {
                    Stmt::TryCatch(try_kw, Box::new(block), catch, finally)
                })
                .boxed()
        };

        let block_expr = block
            .or(_if)
            .or(_while)
            .or(_forall)
            .or(_for)
            .or(_switch)
            .or(_try)
            .boxed()
            .labelled("block");

        block_expr.or(inline_expr)
    })
}

#[cfg(test)]
mod tests {
    use super::super::expr::{BinaryOp::*, Expr::*, Value::*};
    use super::super::token_stream_from_str;
    use super::*;

    #[inline]
    fn span(range: std::ops::Range<usize>) -> SimpleSpan {
        SimpleSpan::from(range)
    }

    #[test]
    fn test_parse_simple() {
        let source = r#"перем y = 10.5;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Var(
            Some(KwLang::Ru),
            "y".to_string(),
            Some((Value(Num(10.5)), span(15..19))),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_return() {
        let source = r#"return y;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Ret(
            KwLang::Eng,
            Some((Ident("y".to_string()), span(7..8))),
        ));
        assert_eq!(parsed, expected);

        let source = r#"return;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Ret(KwLang::Eng, None));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block() {
        let source = r#"{ перем y = 10.5 var z = "hello"; x = @{ a: 1 }; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();

        let expected = Ok(Stmt::Block(vec![
            Stmt::Var(
                Some(KwLang::Ru),
                "y".to_string(),
                Some((Value(Num(10.5)), span(17..21))),
            ),
            Stmt::Var(
                Some(KwLang::Eng),
                "z".to_string(),
                Some((Value(Str("hello".to_string())), span(30..37))),
            ),
            Stmt::Var(
                None,
                "x".to_string(),
                Some((
                    Obj(vec![("a".to_string(), (Value(Num(1.0)), span(49..50)))]),
                    span(43..52),
                )),
            ),
        ]));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block_in_block() {
        let source = r#"{ var x; {var y = 1;} }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Block(vec![
            Stmt::Var(Some(KwLang::Eng), "x".to_string(), None),
            Stmt::Block(vec![Stmt::Var(
                Some(KwLang::Eng),
                "y".to_string(),
                Some((Value(Num(1.0)), span(18..19))),
            )]),
        ]));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if() {
        let expected = Ok(Stmt::If(
            KwLang::Eng,
            (
                Binary(
                    Box::new((Ident("x".to_string()), span(5..6))),
                    Eq,
                    Box::new((Value(Num(1.0)), span(10..11))),
                ),
                span(5..11),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some((Ident("x".to_string()), span(20..21))),
            )])),
            None,
        ));

        let source = r#"if ( x == 1 ) { y = x; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);

        let source = r#"if ( x == 1 )   y = x;  "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if_else() {
        let expected = Ok(Stmt::If(
            KwLang::Eng,
            (
                Binary(
                    Box::new((Ident("x".to_string()), span(5..6))),
                    Eq,
                    Box::new((Value(Num(1.0)), span(10..11))),
                ),
                span(5..11),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some((Ident("x".to_string()), span(20..21))),
            )])),
            Some(Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some((Value(Num(10.0)), span(36..38))),
            )]))),
        ));

        let source = r#"if ( x == 1 ) { y = x; } else { y = 10; } "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);

        let source = r#"if ( x == 1 )   y = x;   else   y = 10;   "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if_else_if() {
        let source = r#"if ( x == 1 ) { y = x; } else if (x == 2) { y = x; } else { y = 10; } "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert!(parsed.is_ok());

        let source = r#"if ( x == 1 ) y = x; else if (x == 2) y = x; else y = 10; "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_while() {
        let source = r#"while ( x < 1 ) { x = 1; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::While(
            KwLang::Eng,
            (
                Binary(
                    Box::new((Ident("x".to_string()), span(8..9))),
                    Lt,
                    Box::new((Value(Num(1.0)), span(12..13))),
                ),
                span(8..13),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x".to_string(),
                Some((Value(Num(1.0)), span(22..23))),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_forall() {
        let source = r#"forall ( i in @[1,2,3]  ) { x = x + i }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::ForAll(
            KwLang::Eng,
            "i".to_string(),
            (
                Arr(vec![
                    (Value(Num(1.0)), span(16..17)),
                    (Value(Num(2.0)), span(18..19)),
                    (Value(Num(3.0)), span(20..21)),
                ]),
                span(14..22),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x".to_string(),
                Some((
                    Binary(
                        Box::new((Ident("x".to_string()), span(32..33))),
                        Add,
                        Box::new((Ident("i".to_string()), span(36..37))),
                    ),
                    span(32..37),
                )),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_forall_fabric() {
        let source = r#"ДляВсех( Элементов(м, инд) ) {}"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::ForAll2(
            KwLang::Ru,
            "Элементов".to_string(),
            (Ident("м".to_string()), span(35..37)),
            "инд".to_string(),
            Box::new(Stmt::Block(vec![])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_for() {
        let source = r#"for ( i = 0; i < 10; i = i + 1 ) { x = x + i }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::For(
            KwLang::Eng,
            Box::new(Stmt::Var(
                None,
                "i".to_string(),
                Some((Value(Num(0.0)), span(10..11))),
            )),
            (
                Binary(
                    Box::new((Ident("i".to_string()), span(13..14))),
                    Lt,
                    Box::new((Value(Num(10.0)), span(17..19))),
                ),
                span(13..19),
            ),
            Box::new(Stmt::Var(
                None,
                "i".to_string(),
                Some((
                    Binary(
                        Box::new((Ident("i".to_string()), span(25..26))),
                        Add,
                        Box::new((Value(Num(1.0)), span(29..30))),
                    ),
                    span(25..30),
                )),
            )),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x".to_string(),
                Some((
                    Binary(
                        Box::new((Ident("x".to_string()), span(39..40))),
                        Add,
                        Box::new((Ident("i".to_string()), span(43..44))),
                    ),
                    span(39..44),
                )),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_switch() {
        let source = r#"
            switch(x) { 
                case 1, 2: { y = 1; }
                else { y = 10; }
            }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Switch(
            KwLang::Eng,
            (Ident("x".to_string()), span(20..21)),
            vec![
                (
                    Some(vec![
                        (Value(Num(1.0)), span(47..48)),
                        (Value(Num(2.0)), span(50..51)),
                    ]),
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y".to_string(),
                        Some((Value(Num(1.0)), span(59..60))),
                    )])),
                ),
                (
                    None,
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y".to_string(),
                        Some((Value(Num(10.0)), span(91..93))),
                    )])),
                ),
            ],
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_switch_without_braces() {
        let source = r#"
            switch(x) { 
                case 1, 2: 
                    y = 1;
                    y = 2;
                else y = 10;
            }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Switch(
            KwLang::Eng,
            (Ident("x".to_string()), span(20..21)),
            vec![
                (
                    Some(vec![
                        (Value(Num(1.0)), span(47..48)),
                        (Value(Num(2.0)), span(50..51)),
                    ]),
                    Box::new(Stmt::Block(vec![
                        Stmt::Var(None, "y".to_string(), Some((Value(Num(1.0)), span(78..79)))),
                        Stmt::Var(
                            None,
                            "y".to_string(),
                            Some((Value(Num(2.0)), span(105..106))),
                        ),
                    ])),
                ),
                (
                    None,
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y".to_string(),
                        Some((Value(Num(10.0)), span(133..135))),
                    )])),
                ),
            ],
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_try() {
        let source = r#"try{ x = x / 0; } catch(e){ throw 1; } finally{ x = 0; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::TryCatch(
            KwLang::Eng,
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x".to_string(),
                Some((
                    Binary(
                        Box::new((Ident("x".to_string()), span(9..10))),
                        Div,
                        Box::new((Value(Num(0.0)), span(13..14))),
                    ),
                    span(9..14),
                )),
            )])),
            Some((
                Some((Ident("e".to_string()), span(24..25))),
                Box::new(Stmt::Block(vec![Stmt::Throw(
                    KwLang::Eng,
                    Some((Value(Num(1.0)), span(34..35))),
                )])),
            )),
            Some(Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x".to_string(),
                Some((Value(Num(0.0)), span(52..53))),
            )]))),
        ));

        assert_eq!(parsed, expected);
    }
}
