use logos::{Logos, Span};
use ropey::Rope;
use tower_lsp::lsp_types::{Position, Range};

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token<'source>>,
    rope: Rope,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
            rope: Rope::from(source),
        }
    }

    pub fn position(&self) -> Option<Range> {
        let Span { start, end } = self.lexer.span();
        let start = self.get_position_from_offset(start)?;
        let end = self.get_position_from_offset(end)?;
        Some(Range { start, end })
    }

    fn get_position_from_offset(&self, offset: usize) -> Option<Position> {
        let line = self.rope.try_byte_to_line(offset).ok()?;
        let first_char_of_line = self.rope.try_line_to_char(line).ok()?;
        let offset_char = self.rope.try_byte_to_char(offset).ok()?;
        let column = offset_char - first_char_of_line;
        Some(Position::new(line as u32, column as u32))
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeywordLanguage {
    Ru,
    Eng,
}

impl From<&str> for KeywordLanguage {
    fn from(value: &str) -> Self {
        match value
            .chars()
            .all(|char| char.is_ascii_alphabetic() || char.is_ascii_punctuation())
        {
            true => Self::Eng,
            false => Self::Ru,
        }
    }
}

fn to_keyword_language<'s>(token: &logos::Lexer<'s, Token<'s>>) -> KeywordLanguage {
    KeywordLanguage::from(token.slice())
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'source> {
    #[regex("(?i)(var|перем)", to_keyword_language)]
    Var(KeywordLanguage),

    #[regex("(?i)(func|функция)", to_keyword_language)]
    Function(KeywordLanguage),

    #[regex("(?i)(class|класс)", to_keyword_language)]
    Class(KeywordLanguage),

    #[regex("(?i)(extends|расширяет)", to_keyword_language)]
    Extends(KeywordLanguage),

    #[regex("(?i)(return|вернуть)", to_keyword_language)]
    Return(KeywordLanguage),

    #[regex("(?i)(forall|длявсех)", to_keyword_language)]
    ForAll(KeywordLanguage),

    #[regex("(?i)(подобъектов)", to_keyword_language)]
    ObjectsIter(KeywordLanguage),

    #[regex("(?i)(элементов)", to_keyword_language)]
    ElementsIter(KeywordLanguage),

    #[regex("(?i)(if|если)", to_keyword_language)]
    If(KeywordLanguage),

    #[regex("(?i)(else|иначе)", to_keyword_language)]
    Else(KeywordLanguage),

    #[regex("(?i)(switch|выборпо)", to_keyword_language)]
    Switch(KeywordLanguage),

    #[regex("(?i)(case|выбор)", to_keyword_language)]
    Case(KeywordLanguage),

    #[regex("(?i)(exists|есть)", to_keyword_language)]
    Exists(KeywordLanguage),

    #[regex("(?i)(getordefault|извлечь)", to_keyword_language)]
    GetOrDefault(KeywordLanguage),

    #[regex("(?i)(\\|\\||или)", to_keyword_language)]
    Or(KeywordLanguage),

    #[regex("(?i)(и|&)", to_keyword_language)]
    And(KeywordLanguage),

    #[regex("(?i)throw")]
    Throw,

    #[regex("(?i)(break|прервать)", to_keyword_language)]
    Break(KeywordLanguage),

    #[regex("(?i)(continue|продолжить)", to_keyword_language)]
    Continue(KeywordLanguage),

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token("[")]
    Lbracket,

    #[token("]")]
    Rbracket,

    #[token("@[")]
    ArrayLbracket,

    #[token("@{")]
    ObjectLbracket,

    #[token("@(")]
    SetLbracket,

    #[token("=")]
    Equals,

    #[token("==")]
    EqualsEq,

    #[token("!=")]
    NotEq,

    #[token("!")]
    NotEq2,

    #[token("+")]
    Plus,

    #[token("+=")]
    PlusEq,

    #[token("-")]
    Minus,

    #[token("-=")]
    MinusEq,

    #[token("*")]
    Mul,

    #[token("*=")]
    MulEq,

    #[token("/")]
    Div,

    #[token("/=")]
    DivEq,

    #[token("%")]
    Mod,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("'")]
    Quote,

    #[token(";")]
    SemiColon,

    #[token(":")]
    Colon,

    #[token("?")]
    QuestionMark,

    #[token(">")]
    Greater,

    #[token(">=")]
    GreaterEq,

    #[token("<")]
    Less,

    #[token("<=")]
    LessEq,

    #[regex(r"#.+[\r\n]+", |s| &s.slice()[1..])]
    CommentLine(&'source str),

    #[regex("(?i)(true|false|истина|ложь)")]
    Bool(&'source str),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", priority = 1)]
    Number(&'source str),

    #[regex("\"[^\"]*\"", |s| &s.slice()[1..s.slice().len() - 1])]
    String(&'source str),

    #[regex("`[^`]*`", |s| &s.slice()[1..s.slice().len() - 1])]
    LongString(&'source str),

    #[regex(r"'([a-zA-ZА-Яа-я0-9_@. ]+)'", priority = 1)]
    #[regex(r"[a-zA-ZА-Яа-я0-9_@.]+", priority = 0)]
    Identifier(&'source str),

    #[regex(r"[\r\n]+")]
    NewLine,
}

impl<'source> From<Token<'source>> for &'source str {
    fn from(value: Token<'source>) -> Self {
        match value {
            Token::Identifier(value) => value,
            Token::Number(value) => value,
            Token::String(value) => value,
            Token::LongString(value) => value,
            Token::Bool(value) => value,
            Token::CommentLine(value) => value,

            // Keywords
            Token::Var(value) => match value {
                KeywordLanguage::Eng => "var",
                KeywordLanguage::Ru => "перем",
            },
            Token::Function(value) => match value {
                KeywordLanguage::Eng => "func",
                KeywordLanguage::Ru => "Функция",
            },
            Token::Class(value) => match value {
                KeywordLanguage::Eng => "class",
                KeywordLanguage::Ru => "Класс",
            },
            Token::Extends(value) => match value {
                KeywordLanguage::Eng => "extends",
                KeywordLanguage::Ru => "расширяет",
            },
            Token::Return(value) => match value {
                KeywordLanguage::Eng => "return",
                KeywordLanguage::Ru => "Вернуть",
            },
            Token::ForAll(value) => match value {
                KeywordLanguage::Eng => "forall",
                KeywordLanguage::Ru => "ДляВсех",
            },
            Token::ObjectsIter(value) => match value {
                KeywordLanguage::Eng => unimplemented!(),
                KeywordLanguage::Ru => "ПодОбъектов",
            },
            Token::ElementsIter(value) => match value {
                KeywordLanguage::Eng => unimplemented!(),
                KeywordLanguage::Ru => "Элементов",
            },
            Token::If(value) => match value {
                KeywordLanguage::Eng => "if",
                KeywordLanguage::Ru => "Если",
            },
            Token::Else(value) => match value {
                KeywordLanguage::Eng => "else",
                KeywordLanguage::Ru => "Иначе",
            },
            Token::Switch(value) => match value {
                KeywordLanguage::Eng => "switch",
                KeywordLanguage::Ru => "ВыборПо",
            },
            Token::Case(value) => match value {
                KeywordLanguage::Eng => "case",
                KeywordLanguage::Ru => "Выбор",
            },
            Token::Exists(value) => match value {
                KeywordLanguage::Eng => "exists",
                KeywordLanguage::Ru => "Есть",
            },
            Token::GetOrDefault(value) => match value {
                KeywordLanguage::Eng => "getOrDefault",
                KeywordLanguage::Ru => "Извлечь",
            },
            Token::Or(value) => match value {
                KeywordLanguage::Eng => "|",
                KeywordLanguage::Ru => "или",
            },
            Token::And(value) => match value {
                KeywordLanguage::Eng => "&",
                KeywordLanguage::Ru => "и",
            },
            Token::Break(value) => match value {
                KeywordLanguage::Eng => "break",
                KeywordLanguage::Ru => "прервать",
            },
            Token::Continue(value) => match value {
                KeywordLanguage::Eng => "continue",
                KeywordLanguage::Ru => "продолжить",
            },
            Token::Throw => "throw",

            // Symbols
            Token::Lparen => "(",
            Token::Rparen => ")",
            Token::Lbrace => "{",
            Token::Rbrace => "}",
            Token::Lbracket => "[",
            Token::Rbracket => "]",
            Token::ArrayLbracket => "@[",
            Token::ObjectLbracket => "@{",
            Token::SetLbracket => "@(",
            Token::Equals => "=",
            Token::EqualsEq => "==",
            Token::NotEq => "!=",
            Token::NotEq2 => "!=",
            Token::Plus => "+",
            Token::PlusEq => "+=",
            Token::Minus => "-",
            Token::MinusEq => "-=",
            Token::Mul => "*",
            Token::MulEq => "*=",
            Token::Div => "/",
            Token::DivEq => "/=",
            Token::Mod => "%",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Quote => "'",
            Token::SemiColon => ";",
            Token::Colon => ":",
            Token::QuestionMark => "?",
            Token::Greater => ">",
            Token::GreaterEq => ">=",
            Token::Less => "<",
            Token::LessEq => "<=",

            Token::NewLine => "\n",
        }
    }
}

impl<'source> From<Token<'source>> for String {
    fn from(value: Token<'source>) -> Self {
        let s: &str = value.into();
        s.to_string()
    }
}

impl<'source> From<&Token<'source>> for String {
    fn from(value: &Token<'source>) -> Self {
        let s: &str = (*value).into();
        s.to_string()
    }
}
