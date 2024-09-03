use chumsky::{input::ValueInput, prelude::*};

use super::expr::{parser_expr, Expr};
use super::stmt::{parser_stmt, Stmt};
use super::{KwLang, Token};
use super::{Span, Spanned};

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub question_mark: bool,
    pub initializer: Option<Expr>,
}

#[derive(Debug, PartialEq, Default)]
pub enum MethodType {
    #[default]
    Func,
    Getter,
    Setter,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    m_type: MethodType,
    identifier: String,
    params: Spanned<Vec<Parameter>>,
    body: Spanned<Vec<Stmt>>,
    descr: Option<Vec<String>>,
    doc_string: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum Decl {
    Error,
    Func {
        lang: KwLang,
        identifier: String,
        params: Spanned<Vec<Parameter>>,
        body: Spanned<Vec<Stmt>>,
        descr: Option<Vec<String>>,
        doc_string: Option<String>,
    },
    Class {
        lang: KwLang,
        identifier: String,
        extends: Option<String>,
        methods: Spanned<Vec<Method>>,
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

    let identifier =
        select! { Token::Identifier(ident) => ident.to_string() }.labelled("identifier");

    let param = {
        let param = identifier.then(just(Token::QuestionMark).or_not()).map(
            |(identifier, question_mark)| Parameter {
                identifier,
                question_mark: question_mark.is_some(),
                initializer: None,
            },
        );

        let param_init = identifier
            .then_ignore(just(Token::Equals))
            .then(parser_expr())
            .map(|(identifier, (expr, _))| Parameter {
                identifier,
                question_mark: false,
                initializer: Some(expr),
            });

        let spread = select! { Token::Spread => String::from("...") }.map(|identifier| Parameter {
            identifier,
            question_mark: false,
            initializer: None,
        });

        param_init.or(param).or(spread)
    };

    let params = param
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect::<Vec<_>>()
        .delimited_by(just(Token::Ctrl("(")), just(Token::Ctrl(")")))
        .validate(|params, e, emitter| {
            let count = params
                .iter()
                .filter(|param| param.identifier == "...")
                .count();

            if count > 1 {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("Function must have one `...` parameter at most"),
                ));
            } else if count == 1 && params.last().unwrap().identifier != "..." {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("The function must have the last parameter `...`"),
                ));
            }
            params
        })
        .map_with(|params, e| (params, e.span()))
        .labelled("args");

    let body = parser_stmt()
        .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
        .recover_with(via_parser(nested_delimiters(
            Token::Ctrl("{"),
            Token::Ctrl("}"),
            [
                (Token::ObjectLbracket, Token::Ctrl("}")),
                (Token::ArrayLbracket, Token::Ctrl("]")),
                (Token::Ctrl("["), Token::Ctrl("]")),
                (Token::Ctrl("("), Token::Ctrl(")")),
            ],
            |_| vec![Stmt::Error],
        )))
        .map_with(|body, e| (body, e.span()));

    let fn_ = comment
        .or_not()
        .then(kw)
        .then(identifier.labelled("function name"))
        .then(params.clone())
        .then(doc_string.or_not())
        .then(body.clone())
        .map(
            |(((((descr, lang), identifier), params), doc_string), body)| Decl::Func {
                lang,
                identifier,
                params,
                body,
                descr,
                doc_string,
            },
        )
        .labelled("function");

    let kw = select! {
        Token::Get(KwLang::Eng) => MethodType::Getter,
        Token::Set(KwLang::Eng) => MethodType::Setter,
    };

    let method = comment
        .or_not()
        .then(kw.or_not())
        .then(identifier.labelled("method name"))
        .then(params)
        .then(doc_string.or_not())
        .then(body)
        .map(
            |(((((descr, tp), identifier), params), doc_string), body)| Method {
                m_type: tp.unwrap_or_default(),
                identifier,
                params,
                body,
                descr,
                doc_string,
            },
        )
        .labelled("method");

    let kw_class = select! {
        Token::Class(KwLang::Eng) => KwLang::Eng,
        Token::Class(KwLang::Ru) => KwLang::Ru,
    };

    let kw_ext = select! {
        Token::Extends(KwLang::Eng) => (),
        Token::Extends(KwLang::Ru) => (),
    };

    let class = comment
        .or_not()
        .then(kw_class)
        .then(identifier.labelled("class name"))
        .then(kw_ext.ignore_then(identifier).or_not())
        .then(doc_string.or_not())
        .then(
            method
                .repeated()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
                .map_with(|methods, e| (methods, e.span())),
        )
        .map(
            |(((((descr, lang), identifier), extends), doc_string), methods)| Decl::Class {
                lang,
                identifier,
                extends,
                methods,
                descr,
                doc_string,
            },
        )
        .labelled("class");

    fn_.or(class).repeated().collect::<Vec<_>>()
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
                vec![Parameter {
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
            func test(x, y = 1, z?)
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
                vec![
                    Parameter {
                        identifier: "x".to_string(),
                        question_mark: false,
                        initializer: None,
                    },
                    Parameter {
                        identifier: "y".to_string(),
                        question_mark: false,
                        initializer: Some(Value(Num(1.0))),
                    },
                    Parameter {
                        identifier: "z".to_string(),
                        question_mark: true,
                        initializer: None,
                    },
                ],
                SimpleSpan::from(74..88),
            ),
            body: (
                vec![Ret(
                    KwLang::Eng,
                    Some(Box::new(Expr((
                        Value(Num(10.0)),
                        SimpleSpan::from(180..182),
                    )))),
                )],
                SimpleSpan::from(155..197),
            ),
            descr: Some(vec![
                " description\n".to_string(),
                " another one\n".to_string(),
            ]),
            doc_string: Some(" \n                ret: 10 \n            ".to_string()),
        }]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_fn_with_errors() {
        let source = r#"
            func test() { @[ let x = 10;] }
            func test2() { @{ let x = 10;} }
        "#;
        let token_stream = token_stream_from_str(source);
        let (parsed, _errs) = parser_decl().parse(token_stream).into_output_errors();
        let expected = Some(vec![
            Decl::Func {
                lang: KwLang::Eng,
                identifier: "test".to_string(),
                params: (vec![], SimpleSpan::from(22..24)),
                body: (
                    vec![Stmt::Expr((
                        super::super::expr::Expr::Error,
                        SimpleSpan::from(27..42),
                    ))],
                    SimpleSpan::from(25..44),
                ),
                descr: None,
                doc_string: None,
            },
            Decl::Func {
                lang: KwLang::Eng,
                identifier: "test2".to_string(),
                params: (vec![], SimpleSpan::from(67..69)),
                body: (
                    vec![Stmt::Expr((
                        super::super::expr::Expr::Error,
                        SimpleSpan::from(72..87),
                    ))],
                    SimpleSpan::from(70..89),
                ),
                descr: None,
                doc_string: None,
            },
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_def_class() {
        let source = r#"
            class Test extends Base
            {
                constructor() {}

                get x() {}

                set x() {}

                sum(a, b) {}
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_decl().parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Class {
            lang: KwLang::Eng,
            identifier: "Test".to_string(),
            extends: Some("Base".to_string()),
            methods: (
                vec![
                    Method {
                        m_type: MethodType::Func,
                        identifier: "constructor".to_string(),
                        params: (vec![], SimpleSpan::from(78..80)),
                        body: (vec![], SimpleSpan::from(81..83)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Getter,
                        identifier: "x".to_string(),
                        params: (vec![], SimpleSpan::from(106..108)),
                        body: (vec![], SimpleSpan::from(109..111)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Setter,
                        identifier: "x".to_string(),
                        params: (vec![], SimpleSpan::from(134..136)),
                        body: (vec![], SimpleSpan::from(137..139)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Func,
                        identifier: "sum".to_string(),
                        params: (
                            vec![
                                Parameter {
                                    identifier: "a".to_string(),
                                    question_mark: false,
                                    initializer: None,
                                },
                                Parameter {
                                    identifier: "b".to_string(),
                                    question_mark: false,
                                    initializer: None,
                                },
                            ],
                            SimpleSpan::from(160..166),
                        ),
                        body: (vec![], SimpleSpan::from(167..169)),
                        descr: None,
                        doc_string: None,
                    },
                ],
                SimpleSpan::from(49..183),
            ),
            descr: None,
            doc_string: None,
        }]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_class_with_errors() {
        let source = r#"
            class Test extends Base
            {
                constructor() {}

                get x() {}

                set x() {}

                sum(a, b) { @{ let x = 10;} }
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let (parsed, _errs) = parser_decl().parse(token_stream).into_output_errors();
        let expected = Some(vec![Decl::Class {
            lang: KwLang::Eng,
            identifier: "Test".to_string(),
            extends: Some("Base".to_string()),
            methods: (
                vec![
                    Method {
                        m_type: MethodType::Func,
                        identifier: "constructor".to_string(),
                        params: (vec![], SimpleSpan::from(78..80)),
                        body: (vec![], SimpleSpan::from(81..83)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Getter,
                        identifier: "x".to_string(),
                        params: (vec![], SimpleSpan::from(106..108)),
                        body: (vec![], SimpleSpan::from(109..111)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Setter,
                        identifier: "x".to_string(),
                        params: (vec![], SimpleSpan::from(134..136)),
                        body: (vec![], SimpleSpan::from(137..139)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Func,
                        identifier: "sum".to_string(),
                        params: (
                            vec![
                                Parameter {
                                    identifier: "a".to_string(),
                                    question_mark: false,
                                    initializer: None,
                                },
                                Parameter {
                                    identifier: "b".to_string(),
                                    question_mark: false,
                                    initializer: None,
                                },
                            ],
                            SimpleSpan::from(160..166),
                        ),
                        body: (vec![], SimpleSpan::from(167..169)),
                        descr: None,
                        doc_string: None,
                    },
                ],
                SimpleSpan::from(49..183),
            ),
            descr: None,
            doc_string: None,
        }]);
        assert_eq!(parsed, expected);
    }
}
