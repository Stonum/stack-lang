use chumsky::{input::ValueInput, prelude::*};

use super::expr::{parser_expr, Expr};
use super::{KwLang, Token};
use super::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Error,
    Comment(Spanned<String>),
    Expr(Spanned<Expr>),
    Var(Option<KwLang>, String, Option<Box<Self>>),
    Ret(KwLang, Option<Box<Self>>),
    Block(Vec<Self>),
    If(KwLang, Box<Self>, Box<Self>, Option<Box<Self>>),
}

pub(crate) fn parser_stmt<'source, I>(
) -> impl Parser<'source, I, Vec<Stmt>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let comment = select! { Token::CommentLine(comment) => comment.to_string() }
        .map_with(|comment, e| Stmt::Comment((comment, e.span())))
        .labelled("comment");

    let kw = select! {
        Token::Var(KwLang::Eng) => KwLang::Eng,
        Token::Var(KwLang::Ru) => KwLang::Ru,
    };

    let ident = select! { Token::Identifier(ident) => ident.to_string() }.labelled("identifier");

    let expr = parser_expr()
        .then_ignore(just(Token::SemiColon).or_not())
        .map(|e| Stmt::Expr(e))
        .labelled("expression");

    let var = kw
        .or_not()
        .then(ident)
        .then_ignore(just(Token::SemiColon).or_not())
        .map(|(kw, ident)| Stmt::Var(kw, ident, None));

    let var_eq = kw
        .or_not()
        .then(ident)
        .then_ignore(just(Token::Equals))
        .then(expr.clone())
        .map(|((kw, ident), expr)| Stmt::Var(kw, ident, Some(Box::new(expr))));

    let var = var_eq.or(var).labelled("variable");

    let ret_kw = select! {
        Token::Return(KwLang::Eng) => KwLang::Eng,
        Token::Return(KwLang::Ru) => KwLang::Ru,
    };
    let ret = ret_kw
        .then(expr.clone().or_not())
        .map(|(kw, expr)| Stmt::Ret(kw, expr.map(Box::new)))
        .labelled("return");

    let block = recursive(|block| {
        var.clone()
            .or(ret.clone())
            .or(expr.clone())
            .or(block)
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
            .map(Stmt::Block)
    })
    .labelled("block");

    let if_kw = select! {
        Token::If(KwLang::Eng) => KwLang::Eng,
        Token::If(KwLang::Ru) => KwLang::Ru,
    };

    let else_kw = select! {
        Token::Else(KwLang::Eng) => KwLang::Eng,
        Token::Else(KwLang::Ru) => KwLang::Ru,
    };

    let _if = recursive(|_if| {
        if_kw
            .then(expr.clone())
            .then(block.clone())
            .then(else_kw.ignore_then(block.clone().or(_if)).or_not())
            .map(|(((if_kw, expr), block), else_block)| {
                Stmt::If(
                    if_kw,
                    Box::new(expr),
                    Box::new(block),
                    else_block.map(Box::new),
                )
            })
    });

    expr.or(comment)
        .or(var)
        .or(ret)
        .or(block)
        .or(_if)
        .repeated()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::super::expr::{BinaryOp::*, Expr::*, Value::*};
    use super::super::token_stream_from_str;
    use super::*;

    #[test]
    fn test_parse_simple() {
        let source = r#"перем y = 10.5; var z = "hello";"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![
            Stmt::Var(
                Some(KwLang::Ru),
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Num(10.5)),
                    SimpleSpan::from(15..19),
                )))),
            ),
            Stmt::Var(
                Some(KwLang::Eng),
                "z".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Str("hello".to_string())),
                    SimpleSpan::from(29..36),
                )))),
            ),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_return() {
        let source = r#"return y;"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![Stmt::Ret(
            KwLang::Eng,
            Some(Box::new(Stmt::Expr((
                Ident("y".to_string()),
                SimpleSpan::from(7..8),
            )))),
        )]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block() {
        let source = r#"{ перем y = 10.5 var z = "hello"; x = @{ a: 1 }; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();

        let expected = Ok(vec![Stmt::Block(vec![
            Stmt::Var(
                Some(KwLang::Ru),
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Num(10.5)),
                    SimpleSpan::from(17..21),
                )))),
            ),
            Stmt::Var(
                Some(KwLang::Eng),
                "z".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Str("hello".to_string())),
                    SimpleSpan::from(30..37),
                )))),
            ),
            Stmt::Var(
                None,
                "x".to_string(),
                Some(Box::new(Stmt::Expr((
                    Obj(vec![(
                        "a".to_string(),
                        (Value(Num(1.0)), SimpleSpan::from(49..50)),
                    )]),
                    SimpleSpan::from(43..52),
                )))),
            ),
        ])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_block_in_block() {
        let source = r#"{ var x; {var y = 1;} }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![Stmt::Block(vec![
            Stmt::Var(Some(KwLang::Eng), "x".to_string(), None),
            Stmt::Block(vec![Stmt::Var(
                Some(KwLang::Eng),
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Num(1.0)),
                    SimpleSpan::from(18..19),
                )))),
            )]),
        ])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if() {
        let source = r#"if ( x == 1 ) { y = x; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![Stmt::If(
            KwLang::Eng,
            Box::new(Stmt::Expr((
                Binary(
                    Box::new((Ident("x".to_string()), SimpleSpan::from(5..6))),
                    Eq,
                    Box::new((Value(Num(1.0)), SimpleSpan::from(10..11))),
                ),
                SimpleSpan::from(5..11),
            ))),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Ident("x".to_string()),
                    SimpleSpan::from(20..21),
                )))),
            )])),
            None,
        )]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if_else() {
        let source = r#"if ( x == 1 ) { y = x; } else { y = 10; } "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        let expected = Ok(vec![Stmt::If(
            KwLang::Eng,
            Box::new(Stmt::Expr((
                Binary(
                    Box::new((Ident("x".to_string()), SimpleSpan::from(5..6))),
                    Eq,
                    Box::new((Value(Num(1.0)), SimpleSpan::from(10..11))),
                ),
                SimpleSpan::from(5..11),
            ))),
            Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Ident("x".to_string()),
                    SimpleSpan::from(20..21),
                )))),
            )])),
            Some(Box::new(Stmt::Block(vec![Stmt::Var(
                None,
                "y".to_string(),
                Some(Box::new(Stmt::Expr((
                    Value(Num(10.0)),
                    SimpleSpan::from(36..38),
                )))),
            )]))),
        )]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_if_else_if() {
        let source = r#"if ( x == 1 ) { y = x; } else if (x == 2) { y = x; } else { y = 10; } "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_stmt().parse(token_stream).into_result();
        assert!(parsed.is_ok());
    }
}
