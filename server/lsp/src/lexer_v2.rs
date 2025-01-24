use super::{MLangSyntaxKind, Token, T};
use biome_rowan::TextRange;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq, Copy, Clone)]
pub enum LogosToken<'source> {
    // keywords
    #[regex("(?i)(var|перем)")]
    Var(&'source str),
    #[regex("(?i)(func|функция)")]
    Function(&'source str),
    #[regex("(?i)(class|класс)")]
    Class(&'source str),
    #[regex("(?i)(extends|расширяет)")]
    Extends(&'source str),
    #[regex("(?i)(get|получить)")]
    Get(&'source str),
    #[regex("(?i)(set|установить)")]
    Set(&'source str),
    #[regex("(?i)(return|вернуть)")]
    Return(&'source str),
    #[regex("(?i)(for|для)")]
    For(&'source str),
    #[regex("(?i)(forall|длявсех)")]
    ForAll(&'source str),
    #[regex("(?i)(in|в)")]
    In(&'source str),
    #[regex("(?i)(while|пока)")]
    While(&'source str),
    #[regex("(?i)(if|если)")]
    If(&'source str),
    #[regex("(?i)(else|иначе)")]
    Else(&'source str),
    #[regex("(?i)(switch|выборпо)")]
    Switch(&'source str),
    #[regex("(?i)(case|выбор)")]
    Case(&'source str),
    #[regex("(?i)(try|попытка)")]
    Try(&'source str),
    #[regex("(?i)(catch|исключение|перехват)")]
    Catch(&'source str),
    #[regex("(?i)(finally|заключение)")]
    Finally(&'source str),
    #[regex("(?i)(\\|\\||или|or)")]
    Or(&'source str),
    #[regex("(?i)(и|&&|and)")]
    And(&'source str),
    #[regex("(?i)(throw|вызватьисключение)")]
    Throw(&'source str),
    #[regex("(?i)(break|прервать)")]
    Break(&'source str),
    #[regex("(?i)(continue|продолжить)")]
    Continue(&'source str),

    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("@")]
    At,

    #[token("=")]
    Eq,

    // Condition operators
    #[token("==")]
    Eqq,
    #[token("!=")]
    Neq,
    #[token("!")]
    Bang,
    #[token(">")]
    Gt,
    #[token(">=")]
    GtEq,
    #[token("<")]
    Lt,
    #[token("<=")]
    LtEq,

    // Operators
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEq,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEq,
    #[token("*")]
    Star,
    #[token("*=")]
    MulEq,
    #[token("/")]
    Div,
    #[token("/=")]
    DivEq,
    #[token("%")]
    Mod,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("++")]
    Plus2,
    #[token("--")]
    Minus2,

    // Delimiters
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(";")]
    SemiColon,
    #[token(":")]
    Colon,
    #[token("?")]
    QuestionMark,
    #[token("...")]
    Spread,

    #[regex("(?i)(null|nil|нуль)")]
    Null(&'source str),
    #[regex("(?i)(true|false|истина|ложь)")]
    Bool(&'source str),
    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", priority = 1)]
    Number(&'source str),
    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    String(&'source str),
    #[regex(r#"`([^`\\]*(\\.[^`\\]*)*)`"#)]
    LongString(&'source str),
    #[regex(r"'([a-zA-ZА-Яа-яёЁ0-9_@. ()@%$\-\\>\/]+)'", priority = 1)]
    #[regex(r"[a-zA-ZА-Яа-яёЁ0-9_@$]+", priority = 0)]
    Identifier(&'source str),

    #[regex(r"#.+")]
    CommentLine(&'source str),

    #[regex(r":\[([^\[\]]*)\]")]
    Annotation(&'source str),

    #[regex(r"[ \t\r\f]+")]
    WS,

    #[regex(r"\n+")]
    NewLine,

    Error,
}

impl<'source> LogosToken<'source> {
    pub fn kind(&self) -> MLangSyntaxKind {
        use LogosToken::*;
        match self {
            // Keywords
            Var(_) => MLangSyntaxKind::Var,
            Function(_) => MLangSyntaxKind::Function,
            Class(_) => MLangSyntaxKind::Class,
            Extends(_) => MLangSyntaxKind::Extends,
            Get(_) => MLangSyntaxKind::Get,
            Set(_) => MLangSyntaxKind::Set,
            Return(_) => MLangSyntaxKind::Return,
            ForAll(_) => MLangSyntaxKind::ForAll,
            For(_) => MLangSyntaxKind::For,
            In(_) => MLangSyntaxKind::In,
            While(_) => MLangSyntaxKind::While,
            If(_) => MLangSyntaxKind::If,
            Else(_) => MLangSyntaxKind::Else,
            Switch(_) => MLangSyntaxKind::Switch,
            Case(_) => MLangSyntaxKind::Case,
            Try(_) => MLangSyntaxKind::Try,
            Catch(_) => MLangSyntaxKind::Catch,
            Finally(_) => MLangSyntaxKind::Finally,
            Or(_) => MLangSyntaxKind::Or,
            And(_) => MLangSyntaxKind::And,
            Break(_) => MLangSyntaxKind::Break,
            Continue(_) => MLangSyntaxKind::Continue,
            Throw(_) => MLangSyntaxKind::Throw,

            // Symbols
            LParen => MLangSyntaxKind::LParen,
            RParen => MLangSyntaxKind::RParen,
            LCurly => MLangSyntaxKind::LCurly,
            RCurly => MLangSyntaxKind::RCurly,
            LBracket => MLangSyntaxKind::LBracket,
            RBracket => MLangSyntaxKind::RBracket,
            At => MLangSyntaxKind::At,
            Eq => MLangSyntaxKind::Eq,
            Eqq => MLangSyntaxKind::Eqq,
            Neq => MLangSyntaxKind::Neq,
            Bang => MLangSyntaxKind::Bang,
            Gt => MLangSyntaxKind::Gt,
            GtEq => MLangSyntaxKind::GtEq,
            Lt => MLangSyntaxKind::Lt,
            LtEq => MLangSyntaxKind::LtEq,

            // Operators
            Plus => MLangSyntaxKind::Plus,
            PlusEq => MLangSyntaxKind::PlusEq,
            Minus => MLangSyntaxKind::Minus,
            MinusEq => MLangSyntaxKind::MinusEq,
            Star => MLangSyntaxKind::Star,
            MulEq => MLangSyntaxKind::MulEq,
            Div => MLangSyntaxKind::Div,
            DivEq => MLangSyntaxKind::DivEq,
            Mod => MLangSyntaxKind::Mod,
            BitAnd => MLangSyntaxKind::BitAnd,
            BitOr => MLangSyntaxKind::BitOr,
            Plus2 => MLangSyntaxKind::Plus2,
            Minus2 => MLangSyntaxKind::Minus2,

            // Delimiters
            Comma => MLangSyntaxKind::Comma,
            Dot => MLangSyntaxKind::Dot,
            SemiColon => MLangSyntaxKind::SemiColon,
            Colon => MLangSyntaxKind::Colon,
            QuestionMark => MLangSyntaxKind::QuestionMark,
            Spread => MLangSyntaxKind::Spread,

            // Literals
            Null(_) => MLangSyntaxKind::Null,
            Bool(_) => MLangSyntaxKind::Bool,
            Number(_) => MLangSyntaxKind::Number,
            String(_) => MLangSyntaxKind::String,
            LongString(_) => MLangSyntaxKind::LongString,
            Identifier(_) => MLangSyntaxKind::Identifier,
            CommentLine(_) => MLangSyntaxKind::CommentLine,
            Annotation(_) => MLangSyntaxKind::Annotation,
            WS => MLangSyntaxKind::WS,
            NewLine => MLangSyntaxKind::NewLine,
            Error => MLangSyntaxKind::Error,
        }
    }
}

impl<'source> From<LogosToken<'source>> for &'source str {
    fn from(value: LogosToken<'source>) -> Self {
        use LogosToken::*;
        match value {
            // Keywords
            Var(value) => value,
            Function(value) => value,
            Class(value) => value,
            Extends(value) => value,
            Get(value) => value,
            Set(value) => value,
            Return(value) => value,
            ForAll(value) => value,
            For(value) => value,
            In(value) => value,
            While(value) => value,
            If(value) => value,
            Else(value) => value,
            Switch(value) => value,
            Case(value) => value,
            Try(value) => value,
            Catch(value) => value,
            Finally(value) => value,
            Or(value) => value,
            And(value) => value,
            Break(value) => value,
            Continue(value) => value,
            Throw(value) => value,

            // Symbols
            LParen => "(",
            RParen => ")",
            LCurly => "{",
            RCurly => "}",
            LBracket => "[",
            RBracket => "]",

            Eqq => "==",
            Neq => "!=",
            Bang => "!",
            Gt => ">",
            GtEq => ">=",
            Lt => "<",
            LtEq => "<=",

            Plus => "+",
            PlusEq => "+=",
            Minus => "-",
            MinusEq => "-=",
            Star => "*",
            MulEq => "*=",
            Div => "/",
            DivEq => "/=",
            Mod => "%",
            BitAnd => "&",
            BitOr => "|",
            Plus2 => "++",
            Minus2 => "--",

            At => "@",
            Eq => "=",
            Comma => ",",
            Dot => ".",
            SemiColon => ";",
            Colon => ":",
            QuestionMark => "?",
            Spread => "...",

            Identifier(value) => value,
            Number(value) => value,
            String(value) => value,
            LongString(value) => value,
            Bool(value) => value,
            Null(value) => value,
            CommentLine(value) => value,

            Annotation(value) => value,

            NewLine => "\n",
            WS => " ",

            Error => "Error parsing",
        }
    }
}

impl<'a> std::fmt::Display for LogosToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token: &str = (*self).into();
        write!(f, "{token}")
    }
}

pub struct LogosLexer<'source> {
    generated: logos::SpannedIter<'source, LogosToken<'source>>,
    source: &'source str,
    eof: bool,
}

impl<'source> LogosLexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            generated: LogosToken::lexer(source).spanned(),
            source,
            eof: false,
        }
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.take(100).collect()
    }
}

impl<'source> Iterator for LogosLexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.generated.next();
            match token {
                Some((Ok(token), span)) => {
                    return Some(Token {
                        kind: token.kind(),
                        range: TextRange::new(
                            span.start
                                .try_into()
                                .expect("failed to convert usize to u32"),
                            span.end.try_into().expect("failed to convert usize to u32"),
                        ),
                    });
                }
                Some((Err(_), span)) => {
                    return Some(Token {
                        kind: T![error],
                        range: TextRange::new(
                            span.start
                                .try_into()
                                .expect("failed to convert usize to u32"),
                            span.end.try_into().expect("failed to convert usize to u32"),
                        ),
                    })
                }
                None if self.eof => return None,
                None => {
                    self.eof = true;
                    let len = self.source.len() as u32;
                    return Some(Token {
                        kind: T![EOF],
                        range: TextRange::new(len.into(), len.into()),
                    });
                }
            }
        }
    }
}

pub type Lexer<'source> = LogosLexer<'source>;
