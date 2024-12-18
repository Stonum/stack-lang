use logos::{Logos, Span};

pub struct Lexer<'source> {
    lexer: logos::Lexer<'source, Token<'source>>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
        }
    }

    pub fn span(&self) -> Span {
        self.lexer.span()
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Token<'source>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KwLang {
    Eng,
    Ru,
}

impl From<&str> for KwLang {
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

fn to_keyword_language<'s>(token: &logos::Lexer<'s, Token<'s>>) -> KwLang {
    KwLang::from(token.slice())
}

#[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
#[logos(skip r"[ \r\t\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'source> {
    #[regex("(?i)(var|перем)", to_keyword_language)]
    Var(KwLang),

    #[regex("(?i)(func|функция)", to_keyword_language)]
    Function(KwLang),

    #[regex("(?i)(class|класс)", to_keyword_language)]
    Class(KwLang),

    #[regex("(?i)(extends|расширяет)", to_keyword_language)]
    Extends(KwLang),

    #[regex("(?i)(get|получить)", to_keyword_language)]
    Get(KwLang),

    #[regex("(?i)(set|установить)", to_keyword_language)]
    Set(KwLang),

    #[regex("(?i)(return|вернуть)", to_keyword_language)]
    Return(KwLang),

    #[regex("(?i)(for|для)", to_keyword_language)]
    For(KwLang),

    #[regex("(?i)(forall|длявсех)", to_keyword_language)]
    ForAll(KwLang),

    #[regex("(?i)(in|в)", to_keyword_language)]
    In(KwLang),

    #[regex("(?i)(while|пока)", to_keyword_language)]
    While(KwLang),

    #[regex("(?i)(if|если)", to_keyword_language)]
    If(KwLang),

    #[regex("(?i)(else|иначе)", to_keyword_language)]
    Else(KwLang),

    #[regex("(?i)(switch|выборпо)", to_keyword_language)]
    Switch(KwLang),

    #[regex("(?i)(case|выбор)", to_keyword_language)]
    Case(KwLang),

    #[regex("(?i)(try|попытка)", to_keyword_language)]
    Try(KwLang),

    #[regex("(?i)(catch|исключение|перехват)", to_keyword_language)]
    Catch(KwLang),

    #[regex("(?i)(finally|заключение)", to_keyword_language)]
    Finally(KwLang),

    #[regex("(?i)(\\|\\||или|or)")]
    Or(&'source str),

    #[regex("(?i)(и|&&|and)")]
    And(&'source str),

    #[regex("(?i)(throw|вызватьисключение)", to_keyword_language)]
    Throw(KwLang),

    #[regex("(?i)(break|прервать)", to_keyword_language)]
    Break(KwLang),

    #[regex("(?i)(continue|продолжить)", to_keyword_language)]
    Continue(KwLang),

    #[token("(")]
    #[token(")")]
    #[token("{")]
    #[token("}")]
    #[token("[")]
    #[token("]")]
    Ctrl(&'source str),

    #[token("@")]
    At,

    #[token("=")]
    Equals,

    #[token("==")]
    #[token("!=")]
    #[token("!")]
    #[token(">")]
    #[token(">=")]
    #[token("<")]
    #[token("<=")]
    CondOp(&'source str),

    #[token("+")]
    #[token("+=")]
    #[token("-")]
    #[token("-=")]
    #[token("*")]
    #[token("*=")]
    #[token("/")]
    #[token("/=")]
    #[token("%")]
    #[token("&")]
    #[token("|")]
    #[token("++")]
    #[token("--")]
    Op(&'source str),

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

    #[token("...")]
    Spread,

    #[regex(r"#.+[\r\n]*", |s| &s.slice()[1..])]
    CommentLine(&'source str),

    #[regex("(?i)(null|nil|нуль)")]
    Null(&'source str),

    #[regex("(?i)(true|false|истина|ложь)")]
    Bool(&'source str),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", priority = 1)]
    Number(&'source str),

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#, |s| &s.slice()[1..s.slice().len() - 1])]
    String(&'source str),

    #[regex(r#"`([^`\\]*(\\.[^`\\]*)*)`"#, |s| &s.slice()[1..s.slice().len() - 1])]
    LongString(&'source str),

    #[regex(r"'([a-zA-ZА-Яа-яёЁ0-9_@. ()@%$\-\\>\/]+)'", priority = 1)]
    #[regex(r"[a-zA-ZА-Яа-яёЁ0-9_@$]+", priority = 0)]
    Identifier(&'source str),

    #[regex(r":\[([^\[\]]*)\]")]
    Annotation(&'source str),

    #[regex(r"\n+")]
    NewLine,

    Error,
}

impl<'source> From<Token<'source>> for &'source str {
    fn from(value: Token<'source>) -> Self {
        match value {
            Token::Identifier(value) => value,
            Token::Number(value) => value,
            Token::String(value) => value,
            Token::LongString(value) => value,
            Token::Bool(value) => value,
            Token::Null(value) => value,
            Token::CommentLine(value) => value,

            // Keywords
            Token::Var(value) => match value {
                KwLang::Eng => "var",
                KwLang::Ru => "перем",
            },
            Token::Function(value) => match value {
                KwLang::Eng => "func",
                KwLang::Ru => "Функция",
            },
            Token::Class(value) => match value {
                KwLang::Eng => "class",
                KwLang::Ru => "Класс",
            },
            Token::Extends(value) => match value {
                KwLang::Eng => "extends",
                KwLang::Ru => "расширяет",
            },
            Token::Get(value) => match value {
                KwLang::Eng => "get",
                KwLang::Ru => "получить",
            },
            Token::Set(value) => match value {
                KwLang::Eng => "set",
                KwLang::Ru => "установить",
            },
            Token::Return(value) => match value {
                KwLang::Eng => "return",
                KwLang::Ru => "Вернуть",
            },
            Token::ForAll(value) => match value {
                KwLang::Eng => "forall",
                KwLang::Ru => "ДляВсех",
            },
            Token::For(value) => match value {
                KwLang::Eng => "for",
                KwLang::Ru => "Для",
            },
            Token::In(value) => match value {
                KwLang::Eng => "in",
                KwLang::Ru => "в",
            },
            Token::While(value) => match value {
                KwLang::Eng => "while",
                KwLang::Ru => "Пока",
            },
            Token::If(value) => match value {
                KwLang::Eng => "if",
                KwLang::Ru => "Если",
            },
            Token::Else(value) => match value {
                KwLang::Eng => "else",
                KwLang::Ru => "Иначе",
            },
            Token::Switch(value) => match value {
                KwLang::Eng => "switch",
                KwLang::Ru => "ВыборПо",
            },
            Token::Case(value) => match value {
                KwLang::Eng => "case",
                KwLang::Ru => "Выбор",
            },
            Token::Try(value) => match value {
                KwLang::Eng => "try",
                KwLang::Ru => "Попытка",
            },
            Token::Catch(value) => match value {
                KwLang::Eng => "catch",
                KwLang::Ru => "Исключение",
            },
            Token::Finally(value) => match value {
                KwLang::Eng => "finally",
                KwLang::Ru => "Заключение",
            },
            Token::Or(value) => value,
            Token::And(value) => value,
            Token::Break(value) => match value {
                KwLang::Eng => "break",
                KwLang::Ru => "прервать",
            },
            Token::Continue(value) => match value {
                KwLang::Eng => "continue",
                KwLang::Ru => "продолжить",
            },
            Token::Throw(value) => match value {
                KwLang::Eng => "throw",
                KwLang::Ru => "вызватьисключение",
            },

            // Symbols
            Token::Ctrl(value) => value,
            Token::At => "@",
            Token::Equals => "=",
            Token::Op(value) => value,
            Token::CondOp(value) => value,
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Quote => "'",
            Token::SemiColon => ";",
            Token::Colon => ":",
            Token::QuestionMark => "?",
            Token::Spread => "...",

            Token::Annotation(value) => value,

            Token::NewLine => "\n",

            Token::Error => "Error parsing",
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token: &str = (*self).into();
        write!(f, "{token}")
    }
}
