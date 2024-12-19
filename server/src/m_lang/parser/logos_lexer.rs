use biome_parser::lexer::LexContext;
// use super::{MSyntaxKind, Token, T};
use biome_rowan::TextRange;
use logos::Logos;

use super::token_source::Token;
use super::MSyntaxKind;

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
    // #[regex("(?i)(true|false|истина|ложь)")]
    // Bool(&'source str),
    #[regex("(?i)(true|истина)")]
    True(&'source str),
    #[regex("(?i)(false|ложь)")]
    False(&'source str),
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
    pub fn kind(&self) -> MSyntaxKind {
        use LogosToken::*;
        use MSyntaxKind::*;
        match self {
            // Keywords
            Var(_) => VAR_KW,
            Function(_) => FUNCTION_KW,
            Class(_) => CLASS_KW,
            Extends(_) => EXTENDS_KW,
            Get(_) => GET_KW,
            Set(_) => SET_KW,
            Return(_) => RETURN_KW,
            ForAll(_) => FORALL_KW,
            For(_) => FOR_KW,
            In(_) => IN_KW,
            While(_) => WHILE_KW,
            If(_) => IF_KW,
            Else(_) => ELSE_KW,
            Switch(_) => SWITCH_KW,
            Case(_) => CASE_KW,
            Try(_) => TRY_KW,
            Catch(_) => CATCH_KW,
            Finally(_) => FINALLY_KW,
            Throw(_) => THROW_KW,
            Or(_) => OR_KW,
            And(_) => AND_KW,
            Break(_) => BREAK_KW,
            Continue(_) => CONTINUE_KW,

            // Symbols
            LParen => L_PAREN,
            RParen => R_PAREN,
            LCurly => L_CURLY,
            RCurly => R_CURLY,
            LBracket => L_BRACK,
            RBracket => R_BRACK,
            At => AT,
            Eq => EQ,
            Eqq => EQ2,
            Neq => NEQ,
            Bang => BANG,
            Gt => R_ANGLE,
            GtEq => GTEQ,
            Lt => L_ANGLE,
            LtEq => LTEQ,

            // Operators
            Plus => PLUS,
            PlusEq => PLUS2,
            Minus => MINUS,
            MinusEq => MINUS2,
            Star => STAR,
            MulEq => STAREQ,
            Div => SLASH,
            DivEq => SLASHEQ,
            Mod => PERCENT,
            BitAnd => AMP,
            BitOr => PIPE,
            Plus2 => PLUS2,
            Minus2 => MINUS2,

            // Delimiters
            Comma => COMMA,
            Dot => DOT,
            SemiColon => SEMICOLON,
            Colon => COLON,
            QuestionMark => QUESTION,
            Spread => DOT3,

            // Literals
            Null(_) => NULL_KW,
            // Bool(_) => M_BOOLEAN_LITERAL_EXPRESSION, // TODO mb M_BOOLEAN_LITERAL
            True(_) => TRUE_KW,
            False(_) => FALSE_KW,
            Number(_) => M_NUMBER_LITERAL,
            String(_) => M_STRING_LITERAL,
            LongString(_) => M_LONG_STRING_LITERAL,
            Identifier(_) => IDENT,
            CommentLine(_) => COMMENT,
            Annotation(_) => COMMENT, // TODO: should be ANNOTATION
            WS => WHITESPACE,
            NewLine => NEWLINE,
            Error => ERROR_TOKEN,
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
            True(value) => value,
            False(value) => value,
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
    current_kind: MSyntaxKind,
    eof: bool,
}

impl<'source> LogosLexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            generated: LogosToken::lexer(source).spanned(),
            source,
            current_kind: MSyntaxKind::TOMBSTONE,
            eof: false,
        }
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }
}

impl<'source> Iterator for LogosLexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.generated.next();
            match token {
                Some((Ok(token), span)) => {
                    self.current_kind = token.kind();
                    return Some(Token {
                        kind: self.current_kind,
                        range: TextRange::new(
                            span.start
                                .try_into()
                                .expect("failed to convert usize to u32"),
                            span.end.try_into().expect("failed to convert usize to u32"),
                        ),
                    });
                }
                Some((Err(_), span)) => {
                    self.current_kind = MSyntaxKind::ERROR_TOKEN;
                    return Some(Token {
                        kind: self.current_kind,
                        range: TextRange::new(
                            span.start
                                .try_into()
                                .expect("failed to convert usize to u32"),
                            span.end.try_into().expect("failed to convert usize to u32"),
                        ),
                    });
                }
                None if self.eof => return None,
                None => {
                    self.eof = true;
                    let len = self.source.len() as u32;
                    self.current_kind = MSyntaxKind::EOF;
                    return Some(Token {
                        kind: self.current_kind,
                        range: TextRange::new(len.into(), len.into()),
                    });
                }
            }
        }
    }
}

pub type Lexer<'source> = LogosLexer<'source>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MLexContext {
    /// Default context for if the lexer isn't in any specific other context
    #[default]
    Regular,
}

impl LexContext for MLexContext {
    fn is_regular(&self) -> bool {
        true
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MReLexContext {
    /// Re-lexes
    /// * `> >` as `>>`
    /// * `> > >` as `>>>`,
    /// * `> =` as '>='
    /// * `> > =` as '>>='
    /// * `> > > =` as `>>>=`
    BinaryOperator,
}

impl<'source> biome_parser::lexer::Lexer<'source> for LogosLexer<'source> {
    const NEWLINE: Self::Kind = MSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = MSyntaxKind::WHITESPACE;

    type Kind = MSyntaxKind;
    type LexContext = MLexContext;
    type ReLexContext = MReLexContext;

    fn source(&self) -> &'source str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.next();
        self.current_kind
    }

    fn current_start(&self) -> biome_rowan::TextSize {
        todo!()
    }

    fn has_preceding_line_break(&self) -> bool {
        todo!()
    }

    fn has_unicode_escape(&self) -> bool {
        todo!()
    }

    fn rewind(&mut self, checkpoint: biome_parser::lexer::LexerCheckpoint<Self::Kind>) {
        todo!()
    }

    fn finish(self) -> Vec<biome_parser::prelude::ParseDiagnostic> {
        todo!()
    }

    fn position(&self) -> usize {
        todo!()
    }

    fn push_diagnostic(&mut self, diagnostic: biome_parser::prelude::ParseDiagnostic) {
        todo!()
    }

    fn advance_char_unchecked(&mut self) {
        todo!()
    }

    fn advance(&mut self, n: usize) {
        todo!()
    }
}
