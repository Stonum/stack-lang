use tower_lsp::lsp_types::Range;

use crate::fmt::Formatter;
use crate::lexer::{Lexer, Token};

use log::debug;

pub struct Parser<'source> {
    lexer: Lexer<'source>,
    peeked: Option<Option<Token<'source>>>,
    current: Option<Token<'source>>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Lexer::new(source),
            peeked: None,
            current: None,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmt = vec![];

        while self.peek().is_some() {
            stmt.push(Stmt::parse(self));
        }
        stmt
    }

    pub fn parse_definitions(&mut self) -> Vec<(String, Range)> {
        let mut def = vec![];
        while let Some(token) = self.next() {
            match token {
                Token::Function(_) => {
                    if let Some(&Token::Identifier(_)) = self.peek() {
                        let id = self.next().unwrap().into();
                        let pos = self.position();
                        def.push((id, pos));
                    }
                }
                _ => continue,
            }
        }
        def
    }

    fn peek(&mut self) -> Option<&Token<'source>> {
        let cb = || {
            while let Some(result) = self.lexer.next() {
                match result {
                    Ok(token) => return Some(token),
                    Err(_) => continue,
                };
            }
            None
        };

        self.peeked.get_or_insert_with(cb).as_ref()
    }

    fn next(&mut self) -> Option<Token<'source>> {
        self.current = match self.peeked.take() {
            Some(token) => token,
            None => match self.lexer.next() {
                Some(Ok(token)) => Some(token),
                Some(Err(_)) => self.next(),
                _ => None,
            },
        };
        self.current
    }

    fn current(&self) -> Option<Token<'source>> {
        self.current.clone()
    }

    fn position(&self) -> Range {
        self.lexer.position().unwrap()
    }
}

#[derive(Debug)]
pub enum Stmt<'source> {
    Function(SFunction<'source>),
    Array(SArray<'source>),
    Object(SObject<'source>),
    Expression(SExpression<'source>),
    Token(Token<'source>),
    EmptyLine,
    Eof,
}

impl<'source> Stmt<'source> {
    pub fn parse(parser: &mut Parser<'source>) -> Self {
        let current_token = parser.current();
        let next_token = parser.peek();

        match next_token {
            Some(Token::Function(_)) => Self::Function(SFunction::parse(parser)),
            Some(Token::ArrayLbracket) => Self::Array(SArray::parse(parser)),
            Some(Token::ObjectLbracket) => Self::Object(SObject::parse(parser)),
            Some(Token::Var(_)) => Self::Expression(SExpression::parse(parser)),
            Some(Token::Identifier(_)) => Self::Expression(SExpression::parse(parser)),
            Some(Token::NewLine) if current_token == Some(Token::NewLine) => {
                parser.next();
                Self::EmptyLine
            }
            Some(_) => Self::Token(parser.next().expect("get peeked token")),
            None => Self::Eof,
        }
    }

    pub fn fmt(&self, f: &mut Formatter) -> String {
        match self {
            Self::Function(x) => x.fmt(f),
            Self::Array(x) => x.fmt(f),
            Self::Object(x) => x.fmt(f),
            Self::Expression(x) => x.fmt(f),
            Self::Token(x) => x.into(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SFunction<'source> {
    pub token: Token<'source>,
    pub identifier: Token<'source>,
    pub position: Range,
    pub parameters: Vec<SFunctionParameter<'source>>,
    pub body: Vec<Stmt<'source>>,
    pub stmt: Vec<Stmt<'source>>,
}

impl<'source> SFunction<'source> {
    fn parse(parser: &mut Parser<'source>) -> Self {
        let token = parser.next().unwrap();
        let identifier = parser.next().unwrap();
        let range = parser.position();
        let mut parameters = vec![];
        let mut body = vec![];
        let mut stmt = vec![];

        while let Some(token) = parser.peek() {
            match token {
                Token::Lparen => {
                    parameters = SFunction::parse_parameters(parser);
                }
                Token::Lbrace => {
                    body = SFunction::parse_body(parser);
                    break;
                }
                _ => {
                    // stmt.push(Stmt::parse(parser));
                    parser.next();
                }
            }
        }
        Self {
            token,
            identifier,
            position: range,
            parameters,
            body,
            stmt,
        }
    }

    fn parse_parameters(parser: &mut Parser<'source>) -> Vec<SFunctionParameter<'source>> {
        let mut has_paren = false;
        let mut parameters = vec![];
        let mut paren_count = 0;
        while let Some(token) = parser.peek() {
            match token {
                Token::Lparen => {
                    parser.next();
                    if has_paren {
                        paren_count += 1;
                        continue;
                    }
                    has_paren = true;
                }
                Token::Rparen => {
                    parser.next();
                    match paren_count {
                        0 | 1 => break,
                        _ => paren_count -= 1,
                    }
                }
                Token::Identifier(_) => {
                    let identifier = parser.next().unwrap();
                    let mut question_mark = false;
                    let mut initializer = Option::None;
                    match parser.peek() {
                        Some(Token::QuestionMark) => {
                            question_mark = true;
                            parser.next();
                        }
                        Some(Token::Equals) => {
                            parser.next();
                            initializer = Some(parser.next().unwrap());
                        }
                        _ => {
                            parser.next();
                        }
                    }

                    parameters.push(SFunctionParameter {
                        identifier,
                        question_mark,
                        initializer,
                    });
                }
                _ => {
                    parser.next();
                }
            }
        }

        parameters
    }

    fn parse_body(parser: &mut Parser<'source>) -> Vec<Stmt<'source>> {
        let mut lbrace = false;
        let mut stmt = vec![];
        let mut brace_count = 0;
        while let Some(token) = parser.peek() {
            match token {
                Token::Lbrace => {
                    parser.next();
                    if lbrace {
                        brace_count += 1;
                        continue;
                    }
                    lbrace = true;
                }
                Token::Rbrace => {
                    parser.next();
                    if brace_count == 0 {
                        break;
                    } else {
                        brace_count -= 1;
                    }
                }
                _ => {
                    stmt.push(Stmt::parse(parser));
                }
            }
        }

        stmt
    }

    pub fn fmt(&self, f: &mut Formatter) -> String {
        f.indent_inc();
        let tab = f.tab();
        let token: &str = self.token.into();
        let ident: &str = self.identifier.into();
        let result = format!(
            "{token} {ident}({})\n{{\n{}\n}}",
            self.parameters
                .iter()
                .map(|x| format!("{}", x.fmt(f)))
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|x| format!("{tab}{}", x.fmt(f)))
                .collect::<Vec<String>>()
                .join(""),
        );
        f.indent_dec();
        result
    }
}

#[derive(Debug)]
pub struct SFunctionParameter<'source> {
    pub identifier: Token<'source>,
    pub question_mark: bool,
    pub initializer: Option<Token<'source>>,
}

impl SFunctionParameter<'_> {
    pub fn fmt(&self, _f: &mut Formatter) -> String {
        let mut result = String::new();
        result.push_str(&String::from(self.identifier));
        if self.question_mark {
            result.push('?');
        }
        if let Some(initializer) = &self.initializer {
            result.push_str(" = ");
            result.push_str(&String::from(initializer));
        }
        result
    }
}

#[derive(Debug)]
pub struct SArray<'source> {
    pub elements: Vec<Stmt<'source>>,
}

impl<'source> SArray<'source> {
    fn parse(parser: &mut Parser<'source>) -> Self {
        let mut elements = vec![];
        parser.next();
        while let Some(token) = parser.peek() {
            match token {
                Token::Rbracket => {
                    parser.next();
                    break;
                }
                Token::Comma | Token::NewLine => {
                    parser.next();
                    continue;
                }
                _ => elements.push(Stmt::parse(parser)),
            }
        }
        Self { elements }
    }

    pub fn fmt(&self, f: &mut Formatter) -> String {
        f.indent_inc();
        let tab = f.tab();
        let result = format!(
            "@[\n{}\n]",
            self.elements
                .iter()
                .map(|x| format!("{}{}", &tab, x.fmt(f)))
                .collect::<Vec<String>>()
                .join(",\n")
        );
        f.indent_dec();
        result
    }
}

#[derive(Debug)]
pub struct SObject<'source> {
    pub elements: Vec<(Stmt<'source>, Stmt<'source>)>,
}

impl<'source> SObject<'source> {
    fn parse(parser: &mut Parser<'source>) -> Self {
        let mut elements = vec![];
        parser.next();
        while let Some(token) = parser.peek() {
            match token {
                Token::Rbrace => {
                    parser.next();
                    break;
                }
                Token::Comma | Token::NewLine => {
                    parser.next();
                    continue;
                }
                Token::Identifier(_) => {
                    let identifier = parser.next().expect("get peeked identifier");
                    if let Some(token) = parser.peek() {
                        if token == &Token::Colon {
                            parser.next();
                            elements.push((Stmt::Token(identifier), Stmt::parse(parser)));
                        }
                    }
                }
                _ => break,
            }
        }
        Self { elements }
    }

    pub fn fmt(&self, f: &mut Formatter) -> String {
        f.indent_inc();
        let tab = f.tab();
        let result = format!(
            "@{{\n{}\n}}",
            self.elements
                .iter()
                .map(|x| format!("{}{}: {}", &tab, x.0.fmt(f), x.1.fmt(f)))
                .collect::<Vec<String>>()
                .join(",\n")
        );
        f.indent_dec();
        result
    }
}

#[derive(Debug)]
pub struct SExpression<'source> {
    pub token: Option<Token<'source>>,
    pub identifier: Option<Token<'source>>,
    pub value: Option<Vec<Stmt<'source>>>,
}

impl<'source> SExpression<'source> {
    fn parse(parser: &mut Parser<'source>) -> Self {
        let mut token = None;
        if let Some(Token::Var(_)) = parser.peek() {
            token = parser.next();
        }

        if let Some(Token::Identifier(_)) = parser.peek() {
            let identifier = Some(parser.next().unwrap());
            let mut value = vec![];
            while let Some(token) = parser.peek() {
                match token {
                    Token::CommentLine(_) | Token::NewLine => {
                        break;
                    }
                    Token::SemiColon => {
                        parser.next();
                        break;
                    }
                    Token::Equals => {
                        parser.next();
                    }
                    _ => {
                        value.push(Stmt::parse(parser));
                    }
                }
            }
            Self {
                token,
                identifier,
                value: Some(value),
            }
        } else {
            Self {
                token,
                identifier: None,
                value: None,
            }
        }
    }

    pub fn fmt(&self, f: &mut Formatter) -> String {
        let mut result = String::new();
        // result.push_str(&String::from(self.token));
        if let Some(identifier) = &self.identifier {
            result.push_str(" ");
            result.push_str(&String::from(identifier));
            result.push_str(" = ");
        }
        if let Some(value) = &self.value {
            for stmt in value {
                result.push_str(&stmt.fmt(f));
            }
        }
        result.push(';');

        result
    }
}
