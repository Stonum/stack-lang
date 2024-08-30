use chumsky::{input::ValueInput, prelude::*};

use super::expr::{parser_expr, Expr};
use super::stmt::{parser_stmt, Stmt};
use super::{KwLang, Token};
use super::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub struct Params {
    pub identifier: String,
    pub question_mark: bool,
    pub initializer: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Func {
        lang: KwLang,
        identifier: String,
        params: Spanned<Vec<Params>>,
        body: Spanned<Vec<Stmt>>,
        descr: Option<Vec<String>>,
        doc_string: Option<String>,
    },
}

pub(crate) fn parser_decl<'source, I>(
) -> impl Parser<'source, I, Vec<Decl>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let kw = select! {
        Token::Function(KwLang::Eng) => KwLang::Eng,
        Token::Function(KwLang::Ru) => KwLang::Ru,
    };

    let comment = select! { Token::CommentLine(comment) => comment.to_string() }
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>();

    let doc_string = select! { Token::LongString(comment) => comment.to_string() };

    let identifier = select! { Token::Identifier(ident) => ident.to_string() };

    let params = identifier
        .then(just(Token::QuestionMark).or_not())
        .map(|(identifier, question_mark)| Params {
            identifier,
            question_mark: question_mark.is_some(),
            initializer: None,
        })
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
        .map_with(|body, e| (body, e.span()));

    let body = parser_stmt()
        .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
        .map_with(|body, e| (body, e.span()));

    let fn_ = comment
        .or_not()
        .then(kw)
        .then(identifier)
        .then(params)
        .then(doc_string.or_not())
        .then(body)
        .map(
            |(((((descr, lang), identifier), params), doc_string), body)| {
                vec![Decl::Func {
                    lang,
                    identifier,
                    params,
                    body,
                    descr,
                    doc_string,
                }]
            },
        );

    fn_
}

#[cfg(test)]
mod tests {
    use super::super::expr::{Expr::*, Value::*};
    use super::super::stmt::Stmt::*;
    use super::super::token_stream_from_str;
    use super::*;

    #[test]
    fn test_parse_simple_fn() {
        let source = r#"func test(z){ var x = z; return x; }"#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_decl().parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Func {
            lang: KwLang::Eng,
            identifier: "test".to_string(),
            params: (
                vec![Params {
                    identifier: "z".to_string(),
                    question_mark: false,
                    initializer: None,
                }],
                SimpleSpan::from(9..12),
            ),
            body: (
                vec![
                    Var(
                        Some(KwLang::Eng),
                        "x".to_string(),
                        Some(Box::new(Expr((
                            Ident("z".to_string()),
                            SimpleSpan::from(22..23),
                        )))),
                    ),
                    Ret(
                        KwLang::Eng,
                        Some(Box::new(Expr((
                            Ident("x".to_string()),
                            SimpleSpan::from(32..33),
                        )))),
                    ),
                ],
                SimpleSpan::from(12..36),
            ),
            descr: None,
            doc_string: None,
        }]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_def_fn() {
        let source = r#"
            # description
            # another one
            func test(z?)
            ` 
                ret: 10 
            `
            {
                return 10;
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_decl().parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Func {
            lang: KwLang::Eng,
            identifier: "test".to_string(),
            params: (
                vec![Params {
                    identifier: "z".to_string(),
                    question_mark: true,
                    initializer: None,
                }],
                SimpleSpan::from(74..78),
            ),
            body: (
                vec![Ret(
                    KwLang::Eng,
                    Some(Box::new(Expr((
                        Value(Num(10.0)),
                        SimpleSpan::from(170..172),
                    )))),
                )],
                SimpleSpan::from(145..187),
            ),
            descr: Some(vec![
                " description\n".to_string(),
                " another one\n".to_string(),
            ]),
            doc_string: Some(" \n                ret: 10 \n            ".to_string()),
        }]);
        assert_eq!(parsed, expected);
    }
}
