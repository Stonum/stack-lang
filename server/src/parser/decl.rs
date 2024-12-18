use chumsky::{input::ValueInput, prelude::*};

use super::cst::Span;
use super::cst::{Decl, Method, MethodType, Parameter};
use super::expr::parser_expr;
use super::stmt::parser_stmt;
use super::{KwLang, Token};

pub(crate) fn parser_decl<'source, I>(
    skip_parse_body: bool,
) -> impl Parser<'source, I, Vec<Decl<'source>>, extra::Err<Rich<'source, Token<'source>, Span>>> + Clone
where
    I: ValueInput<'source, Token = Token<'source>, Span = SimpleSpan>,
{
    let error = just(Token::Error).map(|_| Decl::Error);

    let newline = just(Token::NewLine).repeated().or_not();

    let kw = select! {
        Token::Function(KwLang::Eng) => KwLang::Eng,
        Token::Function(KwLang::Ru) => KwLang::Ru,
    };

    let comment = select! { Token::CommentLine(comment) => comment }
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .padded_by(newline.clone());

    let annotation = select! { Token::Annotation(comment) => comment }.padded_by(newline.clone());

    let doc_string = select! {
        Token::LongString(comment) => comment,
        Token::String(comment) => comment,
    };
    let doc_string = doc_string
        .map(|s| vec![s])
        .or(comment)
        .padded_by(newline.clone());

    let identifier = select! { Token::Identifier(ident) => ident }.labelled("identifier");

    let decl_identifier = select! { Token::Identifier(ident) => ident }
        .separated_by(just(Token::Dot))
        .at_least(1)
        .collect::<Vec<_>>()
        .map_with(|ident, e| (ident, e.span()))
        .labelled("identifier");

    let param = {
        let param = identifier.then(just(Token::QuestionMark).or_not()).map(
            |(identifier, question_mark)| Parameter {
                identifier,
                question_mark: question_mark.is_some(),
                ..Default::default()
            },
        );

        let param_init = identifier
            .then_ignore(just(Token::Equals))
            .then(parser_expr())
            .map(|(identifier, (expr, _))| Parameter {
                identifier,
                initializer: Some(expr),
                ..Default::default()
            });

        let spread = select! { Token::Spread => "..." }.map(|identifier| Parameter {
            identifier,
            ..Default::default()
        });

        param_init.or(param).or(spread)
    };

    let params = param
        .padded_by(newline.clone())
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
            (params, None)
        })
        .recover_with(via_parser(nested_delimiters(
            Token::Ctrl("("),
            Token::Ctrl(")"),
            [
                (Token::Ctrl("{"), Token::Ctrl("}")),
                (Token::Ctrl("["), Token::Ctrl("]")),
                (Token::Ctrl("("), Token::Ctrl(")")),
            ],
            |_| (vec![], Some("Error parsing arguments")),
        )))
        .map_with(|(params, error), e| (params, e.span(), error))
        .padded_by(newline.clone())
        .labelled("args");

    let body = if skip_parse_body {
        parser_stmt().ignored().repeated().map(|_| vec![]).boxed()
    } else {
        parser_stmt().repeated().collect::<Vec<_>>().boxed()
    }
    .padded_by(newline.clone())
    .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
    .recover_with(via_parser(nested_delimiters(
        Token::Ctrl("{"),
        Token::Ctrl("}"),
        [
            (Token::Ctrl("{"), Token::Ctrl("}")),
            (Token::Ctrl("["), Token::Ctrl("]")),
            (Token::Ctrl("("), Token::Ctrl(")")),
        ],
        |span| {
            vec![super::cst::Stmt::Error((
                "Error parsing function body",
                span,
            ))]
        },
    )))
    .map_with(|body, e| (body, e.span()))
    .padded_by(newline.clone());

    let fn_ = comment
        .or_not()
        .then(kw)
        .then(decl_identifier.labelled("function name"))
        .then(params.clone())
        .then(doc_string.or_not())
        .then(body.clone())
        .then_ignore(just(Token::SemiColon).or_not())
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
        .padded_by(newline.clone())
        .labelled("function");

    let kw = select! {
        Token::Get(KwLang::Eng) | Token::Get(KwLang::Ru) => MethodType::Getter,
        Token::Set(KwLang::Eng) | Token::Set(KwLang::Ru) => MethodType::Setter,
    };

    let method = comment
        .or_not()
        .then(kw.or_not())
        .then(decl_identifier.labelled("method name"))
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
        .padded_by(newline.clone())
        .labelled("method");

    let kw_class = select! {
        Token::Class(KwLang::Eng) => KwLang::Eng,
        Token::Class(KwLang::Ru) => KwLang::Ru,
    };

    let kw_ext = select! {
        Token::Extends(KwLang::Eng) => (),
        Token::Extends(KwLang::Ru) => (),
    };

    let class = annotation
        .or_not()
        .ignore_then(comment.or_not())
        .then(kw_class)
        .then(decl_identifier.labelled("class name"))
        .then(kw_ext.ignore_then(identifier).or_not())
        .padded_by(newline.clone())
        .then(doc_string.or_not())
        .then(
            method
                .repeated()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Ctrl("{")), just(Token::Ctrl("}")))
                .map_with(|methods, e| (methods, e.span())),
        )
        .then_ignore(just(Token::SemiColon).or_not())
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
        .padded_by(newline.clone())
        .labelled("class");

    let stmt = parser_stmt()
        .map_with(|stmt, e| Decl::Stmt((stmt, e.span())))
        .labelled("statement");

    fn_.or(class)
        .or(stmt)
        .or(error)
        .repeated()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::super::cst::{self, Decl, Expr::*, SimpleSpan, Stmt::*, Value::*};
    use super::super::token_stream_from_str;
    use super::*;

    #[inline]
    fn span(range: std::ops::Range<usize>) -> SimpleSpan {
        SimpleSpan::from(range)
    }

    #[test]
    fn test_parse_simple_fn() {
        let source = r#"
            func test(z) { 
                var x = z; 
                return x; 
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let parsed = parser_decl(false).parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Func {
            lang: KwLang::Eng,
            identifier: (vec!["test"], span(18..22)),
            params: (
                vec![Parameter {
                    identifier: "z",
                    ..Default::default()
                }],
                span(22..25),
                None,
            ),
            body: (
                vec![
                    Var(Some(KwLang::Eng), "x", Some((Ident("z"), span(53..54)))),
                    Ret(KwLang::Eng, Some((Ident("x"), span(80..81)))),
                ],
                span(26..97),
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
        let parsed = parser_decl(false).parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Func {
            lang: KwLang::Eng,
            identifier: (vec!["test"], span(70..74)),
            params: (
                vec![
                    Parameter {
                        identifier: "x",
                        ..Default::default()
                    },
                    Parameter {
                        identifier: "y",
                        initializer: Some(Value(Num("1"))),
                        ..Default::default()
                    },
                    Parameter {
                        identifier: "z",
                        question_mark: true,
                        ..Default::default()
                    },
                ],
                span(74..88),
                None,
            ),
            body: (
                vec![Ret(KwLang::Eng, Some((Value(Num("10")), span(180..182))))],
                span(155..197),
            ),
            descr: Some(vec![" description\n", " another one\n"]),
            doc_string: Some(vec![" \n                ret: 10 \n            "]),
        }]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_fn_with_errors() {
        let source = r#"
            func test() { 
                @[ let x = 10;]
            }
            func test2() { 
                @{ let x = 10;} 
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let (parsed, _errs) = parser_decl(false).parse(token_stream).into_output_errors();
        let expected = Some(vec![
            Decl::Func {
                lang: KwLang::Eng,
                identifier: (vec!["test"], span(18..22)),
                params: (vec![], span(22..24), None),
                body: (
                    vec![cst::Stmt::Expr((cst::Expr::Error, span(44..60)))],
                    span(25..73),
                ),
                descr: None,
                doc_string: None,
            },
            Decl::Func {
                lang: KwLang::Eng,
                identifier: (vec!["test2"], span(91..96)),
                params: (vec![], span(96..98), None),
                body: (
                    vec![cst::Stmt::Expr((cst::Expr::Error, span(118..135)))],
                    span(99..148),
                ),
                descr: None,
                doc_string: None,
            },
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_fn_with_arg_errors() {
        let source = r#"
            func test( 
                x, # comment
                y # comment 2
            ) {}
        "#;
        let token_stream = token_stream_from_str(source);
        let (parsed, _errs) = parser_decl(false).parse(token_stream).into_output_errors();
        let expected = Some(vec![Decl::Func {
            lang: KwLang::Eng,
            identifier: (vec!["test"], span(18..22)),
            params: (vec![], span(22..97), Some("Error parsing arguments")),
            body: (vec![], span(98..100)),
            descr: None,
            doc_string: None,
        }]);
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
        let parsed = parser_decl(false).parse(token_stream).into_result();
        let expected = Ok(vec![Decl::Class {
            lang: KwLang::Eng,
            identifier: (vec!["Test"], span(19..23)),
            extends: Some("Base"),
            methods: (
                vec![
                    Method {
                        m_type: MethodType::Func,
                        identifier: (vec!["constructor"], span(67..78)),
                        params: (vec![], span(78..80), None),
                        body: (vec![], span(81..83)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Getter,
                        identifier: (vec!["x"], span(105..106)),
                        params: (vec![], span(106..108), None),
                        body: (vec![], span(109..111)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Setter,
                        identifier: (vec!["x"], span(133..134)),
                        params: (vec![], span(134..136), None),
                        body: (vec![], span(137..139)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Func,
                        identifier: (vec!["sum"], span(157..160)),
                        params: (
                            vec![
                                Parameter {
                                    identifier: "a",
                                    ..Default::default()
                                },
                                Parameter {
                                    identifier: "b",
                                    ..Default::default()
                                },
                            ],
                            span(160..166),
                            None,
                        ),
                        body: (vec![], span(167..169)),
                        descr: None,
                        doc_string: None,
                    },
                ],
                span(49..183),
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

                sum(a, b) { 
                    @{ let x = 10;} 
                }
            }
        "#;
        let token_stream = token_stream_from_str(source);
        let (parsed, _errs) = parser_decl(false).parse(token_stream).into_output_errors();
        let expected = Some(vec![Decl::Class {
            lang: KwLang::Eng,
            identifier: (vec!["Test"], span(19..23)),
            extends: Some("Base"),
            methods: (
                vec![
                    Method {
                        m_type: MethodType::Func,
                        identifier: (vec!["constructor"], span(67..78)),
                        params: (vec![], span(78..80), None),
                        body: (vec![], span(81..83)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Getter,
                        identifier: (vec!["x"], span(105..106)),
                        params: (vec![], span(106..108), None),
                        body: (vec![], span(109..111)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Setter,
                        identifier: (vec!["x"], span(133..134)),
                        params: (vec![], span(134..136), None),
                        body: (vec![], span(137..139)),
                        descr: None,
                        doc_string: None,
                    },
                    Method {
                        m_type: MethodType::Func,
                        identifier: (vec!["sum"], span(157..160)),
                        params: (
                            vec![
                                Parameter {
                                    identifier: "a",
                                    ..Default::default()
                                },
                                Parameter {
                                    identifier: "b",
                                    ..Default::default()
                                },
                            ],
                            span(160..166),
                            None,
                        ),
                        body: (
                            vec![cst::Stmt::Expr((cst::Expr::Error, span(190..207)))],
                            span(167..224),
                        ),
                        descr: None,
                        doc_string: None,
                    },
                ],
                span(49..238),
            ),
            descr: None,
            doc_string: None,
        }]);
        assert_eq!(parsed, expected);
    }
}
