use thiserror::Error;
use tower_lsp::lsp_types::Range;

use crate::fmt::Formatter;
use crate::lexer::{Lexer, Token};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token")]
    UnexpectedToken(String, Range),
}

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

    pub fn parse_definitions(&mut self) -> Result<Vec<SDefinition>, ParseError> {
        let mut def = vec![];
        let mut comments = vec![];
        while let Some(token) = self.next() {
            match token {
                Token::CommentLine(_) => comments.push(token),
                Token::Function(_) => {
                    let func = SFunctionDef::parse(self, std::mem::take(&mut comments))?;
                    def.push(SDefinition::Function(func));
                }
                Token::Class(_) => {
                    let class = SClassDef::parse(self)?;
                    def.push(SDefinition::Class(class));
                }
                _ => {
                    comments.clear();
                    continue;
                }
            }
        }
        Ok(def)
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
            Self::Token(x) => format!("{x}"),
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
        let stmt = vec![];

        while let Some(token) = parser.peek() {
            match token {
                Token::Ctrl("(") => {
                    parameters = SFunction::parse_parameters(parser);
                }
                Token::Ctrl("{") => {
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
                Token::Ctrl("(") => {
                    parser.next();
                    if has_paren {
                        paren_count += 1;
                        continue;
                    }
                    has_paren = true;
                }
                Token::Ctrl(")") => {
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
                Token::Ctrl("{") => {
                    parser.next();
                    if lbrace {
                        brace_count += 1;
                        continue;
                    }
                    lbrace = true;
                }
                Token::Ctrl("}") => {
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
        result.push_str(&(self.identifier.to_string()));
        if self.question_mark {
            result.push('?');
        }
        if let Some(initializer) = &self.initializer {
            result.push_str(" = ");
            result.push_str(&(initializer.to_string()));
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
                Token::Ctrl("]") => {
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
                Token::Ctrl("}") => {
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
            result.push_str(&(identifier.to_string()));
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

#[derive(Debug)]
pub enum SDefinition {
    Function(SFunctionDef),
    Class(SClassDef),
}

impl SDefinition {
    pub fn get_identifier(&self) -> String {
        match self {
            SDefinition::Function(x) => x.identifier.clone(),
            SDefinition::Class(x) => x.identifier.clone(),
        }
    }

    pub fn get_position(&self) -> Range {
        match self {
            SDefinition::Function(x) => x.position,
            SDefinition::Class(x) => x.position,
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            SDefinition::Function(x) => x.description.clone().unwrap_or_default(),
            SDefinition::Class(x) => x.description.clone().unwrap_or_default(),
        }
    }

    pub fn get_doc_string(&self) -> String {
        match self {
            SDefinition::Function(x) => x.doc_string.clone().unwrap_or_default(),
            SDefinition::Class(x) => x.doc_string.clone().unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct SFunctionDef {
    pub identifier: String,
    pub position: Range,
    pub description: Option<String>,
    pub doc_string: Option<String>,
}

impl SFunctionDef {
    fn parse(parser: &mut Parser, comments: Vec<Token<'_>>) -> Result<Self, ParseError> {
        match parser.next() {
            Some(Token::Identifier(id)) => {
                let identifier = id.replace("'", "").into();
                let position = parser.position();

                let mut description = None;
                if !comments.is_empty() {
                    description = Some(
                        comments
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                }
                let mut doc_string = None;

                let mut paren_count = 0;
                let mut has_paren = false;
                let mut brace_count = 0;
                let mut has_brace = false;
                while let Some(token) = parser.next() {
                    match token {
                        Token::Ctrl("(") => {
                            paren_count += 1;
                            has_paren = true;
                        }
                        Token::SetLbracket => {
                            paren_count += 1;
                        }
                        Token::Ctrl(")") => {
                            if paren_count == 0 {
                                return Err(ParseError::UnexpectedToken(
                                    token.to_string(),
                                    parser.position(),
                                ));
                            }
                            paren_count -= 1;
                        }
                        Token::LongString(s) if has_paren && paren_count == 0 => {
                            doc_string = Some(s.into())
                        }
                        Token::Ctrl("{") => {
                            brace_count += 1;
                            has_brace = true;
                        }
                        Token::ObjectLbracket => {
                            brace_count += 1;
                        }
                        Token::Ctrl("}") => {
                            if brace_count == 0 {
                                return Err(ParseError::UnexpectedToken(
                                    token.to_string(),
                                    parser.position(),
                                ));
                            }
                            brace_count -= 1;
                            if has_brace && brace_count == 0 {
                                break;
                            }
                        }
                        _ => continue,
                    }
                }

                Ok(Self {
                    identifier,
                    position,
                    description,
                    doc_string,
                })
            }
            Some(token) => Err(ParseError::UnexpectedToken(
                token.to_string(),
                parser.position(),
            )),
            None => Err(ParseError::UnexpectedToken(
                String::from("EOF"),
                parser.position(),
            )),
        }
    }
}

#[derive(Debug)]
pub struct SClassDef {
    pub identifier: String,
    pub position: Range,
    pub methods: Vec<SFunctionDef>,
    pub description: Option<String>,
    pub doc_string: Option<String>,
}

impl SClassDef {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        match parser.next() {
            Some(Token::Identifier(id)) => {
                let identifier = id.replace("'", "").into();
                let position = parser.position();
                let mut methods = vec![];

                let mut brace_count = 0;

                let mut comments = vec![];
                while let Some(token) = parser.peek() {
                    match token {
                        Token::Extends(_) => {
                            parser.next();
                            if let Some(token) = parser.peek() {
                                match token {
                                    Token::Identifier(_) => {
                                        parser.next();
                                    }
                                    _ => {
                                        return Err(ParseError::UnexpectedToken(
                                            token.to_string(),
                                            parser.position(),
                                        ));
                                    }
                                }
                            }
                        }
                        Token::CommentLine(_) => {
                            comments.push(parser.next().unwrap());
                        }
                        Token::Ctrl("{") | Token::ObjectLbracket => {
                            brace_count += 1;
                            parser.next();
                        }
                        Token::Ctrl("}") => {
                            if brace_count == 0 {
                                return Err(ParseError::UnexpectedToken(
                                    token.to_string(),
                                    parser.position(),
                                ));
                            }
                            brace_count -= 1;
                            parser.next();
                            if brace_count == 0 {
                                break;
                            }
                        }
                        Token::Identifier(_) => {
                            if brace_count > 0 {
                                methods.push(SFunctionDef::parse(
                                    parser,
                                    std::mem::take(&mut comments),
                                )?);
                            }
                        }
                        _ => {
                            parser.next();
                            continue;
                        }
                    }
                }

                Ok(Self {
                    identifier,
                    position,
                    methods,
                    description: None,
                    doc_string: None,
                })
            }
            Some(token) => Err(ParseError::UnexpectedToken(
                token.to_string(),
                parser.position(),
            )),
            None => Err(ParseError::UnexpectedToken(
                String::from("EOF"),
                parser.position(),
            )),
        }
    }
}
