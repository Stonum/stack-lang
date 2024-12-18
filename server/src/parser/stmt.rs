use chumsky::{input::ValueInput, prelude::*};

use super::cst::Span;
use super::cst::Stmt;
use super::expr::parser_expr;
use super::{KwLang, Token};

pub(crate) fn parser_stmt<'source, I>(
) -> impl Parser<'source, I, Stmt<'source>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let newline = just(Token::NewLine).repeated().or_not();

    let empty_line = just(Token::NewLine)
        .then(just(Token::NewLine))
        .map(|_| Stmt::EmptyLine);

    let comment = select! { Token::CommentLine(comment) => comment }
        .map_with(|comment, e| Stmt::Comment((comment, e.span())))
        .labelled("comment");

    let ident = select! { Token::Identifier(ident) => ident }.labelled("identifier");

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

    let inline_expr = comment.or(var).or(ret).or(throw).or(expr).boxed();

    let expr_commented = inline_expr
        .clone()
        .then(
            select! { Token::CommentLine(comment) => comment }
                .map_with(|comment, e| (comment, e.span())),
        )
        .map(|(e, comment)| Stmt::InlineComment(Box::new(e), comment))
        .labelled("expression");

    let inline_expr = expr_commented
        .or(inline_expr)
        .padded_by(newline.clone())
        .or(empty_line)
        .boxed();

    let expr = parser_expr();

    recursive(|block_expr| {
        let block_with_braces = block_expr
            .clone()
            .repeated()
            .collect::<Vec<_>>()
            .padded_by(newline.clone())
            .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
            .then_ignore(just(Token::SemiColon).or_not()) // semicolon may be written after block
            .map(Stmt::Block)
            .recover_with(via_parser(nested_delimiters(
                Token::Ctrl("{"),
                Token::Ctrl("}"),
                [
                    (Token::Ctrl("{"), Token::Ctrl("}")),
                    (Token::Ctrl("["), Token::Ctrl("]")),
                    (Token::Ctrl("("), Token::Ctrl(")")),
                ],
                |span| Stmt::Error(("Error parsing block", span)),
            )))
            .boxed();

        let block = block_with_braces
            .clone()
            .or(block_expr.clone().map(|e| Stmt::Block(vec![e]))) // simple block or inline expression without braces
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
                .padded_by(newline.clone())
                .then(block.clone())
                .then(
                    else_kw
                        .padded_by(newline.clone())
                        .ignore_then(block.clone().or(_if))
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
                .padded_by(newline.clone())
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
                .padded_by(newline.clone())
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
                .padded_by(newline.clone())
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
                .padded_by(newline.clone())
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

            let block = block_with_braces.clone().or(block_without_braces);

            let case = case_kw
                .ignore_then(
                    expr.clone()
                        .separated_by(just(Token::Comma))
                        .collect::<Vec<_>>(),
                )
                .then_ignore(just(Token::Colon))
                .padded_by(newline.clone())
                .then(block.clone())
                .map(|(expr, block)| (Some(expr), Box::new(block)))
                .boxed();

            let default_kw = select! {
                Token::Else(KwLang::Eng) => KwLang::Eng,
                Token::Else(KwLang::Ru) => KwLang::Ru,
            };

            let default = default_kw
                .padded_by(newline.clone())
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
                .padded_by(newline.clone())
                .then(
                    case.padded_by(newline.clone())
                        .repeated()
                        .collect::<Vec<_>>()
                        .then(default.padded_by(newline.clone()).or_not())
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
                        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
                        .or_not(),
                )
                .padded_by(newline.clone())
                .then(block.clone())
                .map(|(expr, block)| (expr, Box::new(block)));

            let finally_kw = select! {
                Token::Finally(KwLang::Eng) => KwLang::Eng,
                Token::Finally(KwLang::Ru) => KwLang::Ru,
            };

            let finally = finally_kw
                .padded_by(newline.clone())
                .ignore_then(block.clone())
                .map(Box::new);

            try_kw
                .padded_by(newline.clone())
                .then(block.clone())
                .then(catch.or_not())
                .then(finally.or_not())
                .map(|(((try_kw, block), catch), finally)| {
                    Stmt::TryCatch(try_kw, Box::new(block), catch, finally)
                })
                .boxed()
        };

        let block_expr = block_with_braces
            .or(_if)
            .or(_while)
            .or(_forall)
            .or(_for)
            .or(_switch)
            .or(_try)
            .padded_by(newline.clone())
            .boxed()
            .labelled("block");

        block_expr.or(inline_expr)
    })
}

#[cfg(test)]
mod tests {
    use super::super::cst::{BinaryOp::*, Expr::*, UnaryOp, Value::*};
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
            "y",
            Some((Value(Num("10.5")), span(15..19))),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_commented_expr() {
        let source = r#"перем y = 10.5; # комментарий"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::InlineComment(
            Box::new(Stmt::Var(
                Some(KwLang::Ru),
                "y",
                Some((Value(Num("10.5")), span(15..19))),
            )),
            (" комментарий", span(21..45)),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_return() {
        let source = r#"return y;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Ret(KwLang::Eng, Some((Ident("y"), span(7..8)))));
        assert_eq!(parsed, expected);

        let source = r#"return;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Ret(KwLang::Eng, None));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block() {
        let source = r#"
            {
                перем y = 10.5 
                var z = "hello";
                x = @{ a: 1 };
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();

        let expected = Ok(Stmt::Block(vec![
            Stmt::Var(
                Some(KwLang::Ru),
                "y",
                Some((Value(Num("10.5")), span(46..52))),
            ),
            Stmt::Var(
                Some(KwLang::Eng),
                "z",
                Some((Value(Str("hello")), span(76..83))),
            ),
            Stmt::Var(
                None,
                "x",
                Some((
                    Obj(vec![("a", (Value(Num("1")), span(111..112)))]),
                    span(105..114),
                )),
            ),
        ]));
        assert_eq!(parsed, expected);

        // empty block
        let source = r#"
            {
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();

        let expected = Ok(Stmt::Block(vec![]));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block_in_block() {
        let source = r#"
            { 
                var x; 
                {
                    var y = 1;
                } 
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Block(vec![
            Stmt::Var(Some(KwLang::Eng), "x", None),
            Stmt::Block(vec![Stmt::Var(
                Some(KwLang::Eng),
                "y",
                Some((Value(Num("1")), span(86..87))),
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
                    Box::new((Ident("x"), span(17..18))),
                    Eq,
                    Box::new((Value(Num("1")), span(22..23))),
                ),
                span(17..23),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y",
                Some((Ident("x"), span(48..49))),
            )])),
            None,
        ));

        let source = r#"
            if( x == 1 ) {
                y = x;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);

        let source = r#"
            if( x == 1 )   
               y = x;
        "#;
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
                    Box::new((Ident("x"), span(17..18))),
                    Eq,
                    Box::new((Value(Num("1")), span(22..23))),
                ),
                span(17..23),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y",
                Some((Ident("x"), span(47..48))),
            )])),
            Some(Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y",
                Some((Value(Num("10")), span(90..92))),
            )]))),
        ));

        let source = r#"
            if( x == 1 ) {
               y = x;
            } else {
               y = 10;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);

        let source = r#"
            if( x == 1 )  
               y = x;
            else    
               y = 10;
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if_else_if() {
        let source = r#"
            if( x == 1 ) {
               y = x;
            } else if(x == 2) {
               y = x;
            } else {
               y = 10;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert!(parsed.is_ok());

        let source = r#"
            if( x == 1 )  
               y = x; 
            else if(x == 2)  
               y = x;  
            else y = 10;
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_while() {
        let source = r#"
            while( x < 1 ) {
               x = 1;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::While(
            KwLang::Eng,
            (
                Binary(
                    Box::new((Ident("x"), span(20..21))),
                    Lt,
                    Box::new((Value(Num("1")), span(24..25))),
                ),
                span(20..25),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x",
                Some((Value(Num("1")), span(49..50))),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_forall() {
        let source = r#"
            forall( i in @[1,2,3] ) {
               x = x + i
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::ForAll(
            KwLang::Eng,
            "i",
            (
                Arr(vec![
                    (Value(Num("1")), span(28..29)),
                    (Value(Num("2")), span(30..31)),
                    (Value(Num("3")), span(32..33)),
                ]),
                span(26..34),
            ),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x",
                Some((
                    Binary(
                        Box::new((Ident("x"), span(58..59))),
                        Add,
                        Box::new((Ident("i"), span(62..64))),
                    ),
                    span(58..64),
                )),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_forall_fabric() {
        let source = r#"
            ДляВсех( Элементов(м, инд) ) 
            {
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::ForAll2(
            KwLang::Ru,
            "Элементов",
            (Ident("м"), span(48..50)),
            "инд",
            Box::new(Stmt::Block(vec![])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_for() {
        let source = r#"
            for( i = 0; i < 10; i = i + 1 ) 
            { 
                x = x + i
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::For(
            KwLang::Eng,
            Box::new(Stmt::Var(None, "i", Some((Value(Num("0")), span(22..23))))),
            (
                Binary(
                    Box::new((Ident("i"), span(25..26))),
                    Lt,
                    Box::new((Value(Num("10")), span(29..31))),
                ),
                span(25..31),
            ),
            Box::new(Stmt::Var(
                None,
                "i",
                Some((
                    Binary(
                        Box::new((Ident("i"), span(37..38))),
                        Add,
                        Box::new((Value(Num("1")), span(41..42))),
                    ),
                    span(37..42),
                )),
            )),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x",
                Some((
                    Binary(
                        Box::new((Ident("x"), span(81..82))),
                        Add,
                        Box::new((Ident("i"), span(85..87))),
                    ),
                    span(81..87),
                )),
            )])),
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_switch() {
        let source = r#"
            switch(x) {
                case 1, 2: {
                    y = 1;
                }
                else {
                     y = 10; 
                }
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Switch(
            KwLang::Eng,
            (Ident("x"), span(20..21)),
            vec![
                (
                    Some(vec![
                        (Value(Num("1")), span(46..47)),
                        (Value(Num("2")), span(49..50)),
                    ]),
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y",
                        Some((Value(Num("1")), span(78..79))),
                    )])),
                ),
                (
                    None,
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y",
                        Some((Value(Num("10")), span(147..149))),
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
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::Switch(
            KwLang::Eng,
            (Ident("x"), span(20..21)),
            vec![
                (
                    Some(vec![
                        (Value(Num("1")), span(46..47)),
                        (Value(Num("2")), span(49..50)),
                    ]),
                    Box::new(Stmt::Block(vec![
                        Stmt::Var(None, "y", Some((Value(Num("1")), span(76..77)))),
                        Stmt::Var(None, "y", Some((Value(Num("2")), span(103..104)))),
                    ])),
                ),
                (
                    None,
                    Box::new(Stmt::Block(vec![Stmt::Var(
                        None,
                        "y",
                        Some((Value(Num("10")), span(131..133))),
                    )])),
                ),
            ],
        ));
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_try() {
        let source = r#"
            try{
                x = x / 0;
            } catch(e){
                throw 1;
            } finally{
                x = 0;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::TryCatch(
            KwLang::Eng,
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x",
                Some((
                    Binary(
                        Box::new((Ident("x"), span(38..39))),
                        Div,
                        Box::new((Value(Num("0")), span(42..43))),
                    ),
                    span(38..43),
                )),
            )])),
            Some((
                Some((Ident("e"), span(65..66))),
                Box::new(Stmt::Block(vec![Stmt::Throw(
                    KwLang::Eng,
                    Some((Value(Num("1")), span(91..92))),
                )])),
            )),
            Some(Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "x",
                Some((Value(Num("0")), span(137..138))),
            )]))),
        ));

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_chain_statements() {
        let source = r#"
            if( x == 1 ) 
               while( x < 10)
                  forall( i in @[1,2,3] )
                     x++;
        "#;

        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(Stmt::If(
            KwLang::Eng,
            (
                Binary(
                    Box::new((Ident("x"), span(17..18))),
                    Eq,
                    Box::new((Value(Num("1")), span(22..23))),
                ),
                span(17..23),
            ),
            Box::new(Stmt::Block(vec![Stmt::While(
                KwLang::Eng,
                (
                    Binary(
                        Box::new((Ident("x"), span(49..50))),
                        Lt,
                        Box::new((Value(Num("10")), span(53..55))),
                    ),
                    span(49..55),
                ),
                Box::new(Stmt::Block(vec![Stmt::ForAll(
                    KwLang::Eng,
                    "i",
                    (
                        Arr(vec![
                            (Value(Num("1")), span(90..91)),
                            (Value(Num("2")), span(92..93)),
                            (Value(Num("3")), span(94..95)),
                        ]),
                        span(88..96),
                    ),
                    Box::new(Stmt::Block(vec![Stmt::Expr((
                        UnaryRight(Box::new((Ident("x"), span(120..123))), UnaryOp::Add),
                        span(120..123),
                    ))])),
                )])),
            )])),
            None,
        ));

        assert_eq!(parsed, expected);
    }
}
