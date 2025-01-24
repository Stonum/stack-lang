use biome_rowan::TriviaPieceKind;
use biome_rowan::{RawSyntaxKind, SyntaxKind, TextRange};
use std::{
    fmt,
    ops::{Index, Range},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u16)]
pub enum MLangSyntaxKind {
    TOMBSTONE,
    EOF,
    // keywords
    Var,
    Function,
    Class,
    Extends,
    Get,
    Set,
    Return,
    For,
    ForAll,
    In,
    While,
    If,
    Else,
    Switch,
    Case,
    Try,
    Catch,
    Finally,
    Or,
    And,
    Throw,
    Break,
    Continue,

    // Symbols
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,

    At,

    Eq,

    // Condition operators
    Eqq,
    Neq,
    Bang,
    Gt,
    GtEq,
    Lt,
    LtEq,

    // Operators
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Star,
    MulEq,
    Div,
    DivEq,
    Mod,
    BitAnd,
    BitOr,
    Plus2,
    Minus2,

    // Delimiters
    Comma,
    Dot,
    SemiColon,
    Colon,
    QuestionMark,
    Spread,

    Null,
    Bool,
    Number,
    String,
    LongString,
    Identifier,

    CommentLine,

    Annotation,

    WS,
    NewLine,

    Error,

    // Pseudo tokens
    MLANG_ROOT,
    MLANG_UNARY_EXPRESSION,
    MLANG_BINARY_EXPRESSION,
    MLANG_NUMBER_LITERAL_EXPRESSION,
}

#[macro_export]
macro_rules! TT {
    [var] => {$crate::MLangSyntaxKind::Var};
    [function] => {$crate::MLangSyntaxKind::Function};
    [class] => {$crate::MLangSyntaxKind::Class};
    [extends] => {$crate::MLangSyntaxKind::Extends};
    [get] => {$crate::MLangSyntaxKind::Get};
    [set] => {$crate::MLangSyntaxKind::Set};
    [return] => {$crate::MLangSyntaxKind::Return};
    [for] => {$crate::MLangSyntaxKind::For};
    [forall] => {$crate::MLangSyntaxKind::ForAll};
    [in] => {$crate::MLangSyntaxKind::In};
    [while] => {$crate::MLangSyntaxKind::While};
    [if] => {$crate::MLangSyntaxKind::If};
    [else] => {$crate::MLangSyntaxKind::Else};
    [switch] => {$crate::MLangSyntaxKind::Switch};
    [case] => {$crate::MLangSyntaxKind::Case};
    [try] => {$crate::MLangSyntaxKind::Try};
    [catch] => {$crate::MLangSyntaxKind::Catch};
    [finally] => {$crate::MLangSyntaxKind::Finally};
    [or] => {$crate::MLangSyntaxKind::Or};
    [and] => {$crate::MLangSyntaxKind::And};
    [throw] => {$crate::MLangSyntaxKind::Throw};
    [break] => {$crate::MLangSyntaxKind::Break};
    [continue] => {$crate::MLangSyntaxKind::Continue};

    ['(']=>{$crate::MLangSyntaxKind::LParen};
    [')']=>{$crate::MLangSyntaxKind::RParen};
    ['{']=>{$crate::MLangSyntaxKind::LCurly};
    ['}']=>{$crate::MLangSyntaxKind::RCurly};
    ['[']=>{$crate::MLangSyntaxKind::LBracket};
    [']']=>{$crate::MLangSyntaxKind::RBracket};

    [@]=>{$crate::MLangSyntaxKind::At};

    [=]=>{$crate::MLangSyntaxKind::Eq};

    [==]=>{$crate::MLangSyntaxKind::Eqq};
    [!=]=>{$crate::MLangSyntaxKind::Neq};
    [!]=>{$crate::MLangSyntaxKind::Bang};
    [>]=>{$crate::MLangSyntaxKind::Gt};
    [>=]=>{$crate::MLangSyntaxKind::GtEq};
    [<]=>{$crate::MLangSyntaxKind::Lt};
    [<=]=>{$crate::MLangSyntaxKind::LtEq};

    [+]=>{$crate::MLangSyntaxKind::Plus};
    [+=]=>{$crate::MLangSyntaxKind::PlusEq};
    [-]=>{$crate::MLangSyntaxKind::Minus};
    [-=]=>{$crate::MLangSyntaxKind::MinusEq};
    [*]=>{$crate::MLangSyntaxKind::Star};
    [*=]=>{$crate::MLangSyntaxKind::MulEq};
    [/]=>{$crate::MLangSyntaxKind::Div};
    [/=]=>{$crate::MLangSyntaxKind::DivEq};
    [%]=>{$crate::MLangSyntaxKind::Mod};
    [&]=>{$crate::MLangSyntaxKind::BitAnd};
    [|]=>{$crate::MLangSyntaxKind::BitOr};
    [++]=>{$crate::MLangSyntaxKind::Plus2};
    [--]=>{$crate::MLangSyntaxKind::Minus2};

    // Delimiters
    [,]=>{$crate::MLangSyntaxKind::Comma};
    [.]=>{$crate::MLangSyntaxKind::Dot};
    [;]=>{$crate::MLangSyntaxKind::SemiColon};
    [:]=>{$crate::MLangSyntaxKind::Colon};
    [?]=>{$crate::MLangSyntaxKind::QuestionMark};
    [...]=>{$crate::MLangSyntaxKind::Spread};

    [null]=>{$crate::MLangSyntaxKind::Null};
    [bool]=>{$crate::MLangSyntaxKind::Bool};
    [num]=>{$crate::MLangSyntaxKind::Number};
    [str]=>{$crate::MLangSyntaxKind::String};
    [longstr]=>{$crate::MLangSyntaxKind::LongString};
    [ident]=>{$crate::MLangSyntaxKind::Identifier};

    [comment]=>{$crate::MLangSyntaxKind::CommentLine};

    [annotation]=>{$crate::MLangSyntaxKind::Annotation};

    [ws]=>{$crate::MLangSyntaxKind::WS};
    [newline]=>{$crate::MLangSyntaxKind::NewLine};
    [EOF]=>{$crate::MLangSyntaxKind::EOF};
    [TOMBSTONE]=>{$crate::MLangSyntaxKind::TOMBSTONE};

    [error]=>{$crate::MLangSyntaxKind::Error};
}

impl fmt::Display for MLangSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MLangSyntaxKind::TOMBSTONE => "Tombstone",
                T![EOF] => "EOF",
                T![var] => "var",
                T![function] => "function",
                T![class] => "class",
                T![extends] => "extends",
                T![get] => "get",
                T![set] => "set",
                T![return] => "return",
                T![for] => "for",
                T![forall] => "forall",
                T![in] => "in",
                T![while] => "while",
                T![if] => "if",
                T![else] => "else",
                T![switch] => "switch",
                T![case] => "case",
                T![try] => "try",
                T![catch] => "catch",
                T![finally] => "finally",
                T![or] => "or",
                T![and] => "and",
                T![throw] => "throw",
                T![break] => "break",
                T![continue] => "continue",

                T!['('] => "(",
                T![')'] => ")",
                T!['{'] => "{",
                T!['}'] => "}",
                T!['['] => "[",
                T![']'] => "]",

                T![@] => "@",

                T![=] => "=",

                T![==] => "==",
                T![!=] => "!=",
                T![!] => "!",
                T![>] => ">",
                T![>=] => ">=",
                T![<] => "<",
                T![<=] => "<=",

                T![+] => "+",
                T![+=] => "+=",
                T![-] => "-",
                T![-=] => "-=",
                T![*] => "*",
                T![*=] => "*=",
                T![/] => "/",
                T![/=] => "/=",
                T![%] => "%",
                T![&] => "&",
                T![|] => "|",
                T![++] => "++",
                T![--] => "--",

                // Delimiters
                T![,] => ",",
                T![.] => ".",
                T![;] => ";",
                T![:] => ":",
                T![?] => "?",
                T![...] => "...",

                T![null] => "null",
                T![bool] => "Boolean",
                T![num] => "Number",
                T![str] => "String",
                T![longstr] => "Long String",
                T![ident] => "Identifier",

                T![comment] => "// Comment",

                T![annotation] => ":[Annotation]",

                T![ws] => "<ws>",
                T![newline] => "<newline>",
                T![error] => "<error>",
                _ => "<unknown>",
            }
        )
    }
}

impl From<u16> for MLangSyntaxKind {
    fn from(d: u16) -> MLangSyntaxKind {
        assert!(d <= (T![error] as u16));
        unsafe { std::mem::transmute::<u16, MLangSyntaxKind>(d) }
    }
}

impl From<MLangSyntaxKind> for u16 {
    fn from(k: MLangSyntaxKind) -> u16 {
        k as u16
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Token {
    pub kind: MLangSyntaxKind,
    pub range: TextRange,
}

impl Token {
    pub fn text<'source>(&self, input: &'source str) -> &'source str {
        &input[self.range]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - <{:?}, {:?}>",
            self.kind,
            self.range.start(),
            self.range.end()
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl SyntaxKind for MLangSyntaxKind {
    const TOMBSTONE: Self = MLangSyntaxKind::TOMBSTONE;
    const EOF: Self = MLangSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        false
    }

    fn to_bogus(&self) -> Self {
        Self::TOMBSTONE
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        false
    }

    fn is_list(&self) -> bool {
        false
    }

    fn is_trivia(self) -> bool {
        matches!(self, T![ws] | T![newline] | T![comment] | T![annotation])
    }

    fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            T![var] => "var",
            T![function] => "function",
            T![class] => "class",
            T![extends] => "extends",
            T![get] => "get",
            T![set] => "set",
            T![return] => "return",
            T![for] => "for",
            T![forall] => "forall",
            T![in] => "in",
            T![while] => "while",
            T![if] => "if",
            T![else] => "else",
            T![switch] => "switch",
            T![case] => "case",
            T![try] => "try",
            T![catch] => "catch",
            T![finally] => "finally",
            T![or] => "or",
            T![and] => "and",
            T![throw] => "throw",
            T![break] => "break",
            T![continue] => "continue",

            T!['('] => "(",
            T![')'] => ")",
            T!['{'] => "{",
            T!['}'] => "}",
            T!['['] => "[",
            T![']'] => "]",

            T![@] => "@",

            T![=] => "=",

            T![==] => "==",
            T![!=] => "!=",
            T![!] => "!",
            T![>] => ">",
            T![>=] => ">=",
            T![<] => "<",
            T![<=] => "<=",

            T![+] => "+",
            T![+=] => "+=",
            T![-] => "-",
            T![-=] => "-=",
            T![*] => "*",
            T![*=] => "*=",
            T![/] => "/",
            T![/=] => "/=",
            T![%] => "%",
            T![&] => "&",
            T![|] => "|",
            T![++] => "++",
            T![--] => "--",

            // Delimiters
            T![,] => ",",
            T![.] => ".",
            T![;] => ";",
            T![:] => ":",
            T![?] => "?",
            T![...] => "...",

            T![null] => "null",
            T![bool] => "Boolean",
            T![num] => "Number",
            T![str] => "String",
            T![longstr] => "Long String",
            T![ident] => "Identifier",

            T![comment] => "// Comment",

            T![annotation] => ":[Annotation]",

            _ => return None,
        };

        Some(tok)
    }
}

impl TryFrom<MLangSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: MLangSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                MLangSyntaxKind::NewLine => Ok(TriviaPieceKind::Newline),
                MLangSyntaxKind::WS => Ok(TriviaPieceKind::Whitespace),
                MLangSyntaxKind::CommentLine => Ok(TriviaPieceKind::SingleLineComment),
                MLangSyntaxKind::Annotation => Ok(TriviaPieceKind::MultiLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
