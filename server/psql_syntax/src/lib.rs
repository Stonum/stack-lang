#[macro_use]
mod generated;
mod syntax_node;

use biome_rowan::RawSyntaxKind;
pub use biome_rowan::{
    AstNode, Language, SendNode, SyntaxNode, TextLen, TextRange, TextSize, TokenAtOffset,
    TokenText, TriviaPieceKind, WalkEvent,
};

use crate::generated::kind::PsqlSyntaxKind::*;
pub use crate::generated::*;
pub use crate::syntax_node::*;

impl From<u16> for PsqlSyntaxKind {
    fn from(d: u16) -> PsqlSyntaxKind {
        assert!(d <= (PsqlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, PsqlSyntaxKind>(d) }
    }
}

impl From<PsqlSyntaxKind> for u16 {
    fn from(k: PsqlSyntaxKind) -> u16 {
        k as u16
    }
}

impl PsqlSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (PsqlSyntaxKind::FALSE_KW as u16)
            && (self as u16) >= (PsqlSyntaxKind::INTEGER_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for PsqlSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            PSQL_BOGUS
                | PSQL_BOGUS_STATEMENT
                | PSQL_BOGUS_PARAMETER
                | PSQL_BOGUS_BINDING
                | PSQL_BOGUS_MEMBER
                | PSQL_BOGUS_EXPRESSION
                | PSQL_BOGUS_ASSIGNMENT
        )
    }

    fn to_bogus(&self) -> PsqlSyntaxKind {
        match self {
            kind if AnyPsqlExpression::can_cast(*kind) => PSQL_BOGUS_EXPRESSION,
            _ => PSQL_BOGUS,
        }
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
        PsqlRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        PsqlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            PsqlSyntaxKind::NEWLINE | PsqlSyntaxKind::WHITESPACE | PsqlSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        PsqlSyntaxKind::to_string(self)
    }
}

impl TryFrom<PsqlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: PsqlSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            PsqlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
            PsqlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
            PsqlSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub enum OperatorPrecedence {
    Comma = 0,
    Assignment = 2,
    Conditional = 3,
    LogicalOr = 5,
    LogicalAnd = 6,
    BitwiseOr = 7,
    BitwiseXor = 8,
    BitwiseAnd = 9,
    Equality = 10,
    Relational = 11,
    Additive = 13,
    Multiplicative = 14,
    Unary = 16,
    Update = 17,
    LeftHandSide = 19,
    Member = 20,
    Primary = 21,
    Group = 22,
}

impl OperatorPrecedence {
    /// Returns the operator with the lowest precedence
    pub fn lowest() -> Self {
        OperatorPrecedence::Comma
    }

    /// Returns the operator with the highest precedence
    #[allow(dead_code)]
    pub fn highest() -> Self {
        OperatorPrecedence::Primary
    }

    /// Returns `true` if this operator has right to left associativity
    pub fn is_right_to_left(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::Assignment
                | OperatorPrecedence::Conditional
                | OperatorPrecedence::Update
        )
    }

    /// Returns the precedence for a binary operator token or [None] if the token isn't a binary operator
    pub fn try_from_binary_operator(kind: PsqlSyntaxKind) -> Option<OperatorPrecedence> {
        Some(match kind {
            T![|] => OperatorPrecedence::BitwiseOr,
            T![^] => OperatorPrecedence::BitwiseXor,
            T![&] => OperatorPrecedence::BitwiseAnd,
            T![<] | T![>] | T![<=] | T![>=] => OperatorPrecedence::Relational,
            T![+] | T![-] => OperatorPrecedence::Additive,
            T![*] | T![/] | T![%] => OperatorPrecedence::Multiplicative,
            _ => return None,
        })
    }

    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::BitwiseAnd
                | OperatorPrecedence::BitwiseOr
                | OperatorPrecedence::BitwiseXor
        )
    }

    pub const fn is_additive(&self) -> bool {
        matches!(self, OperatorPrecedence::Additive)
    }

    pub const fn is_equality(&self) -> bool {
        matches!(self, OperatorPrecedence::Equality)
    }

    pub const fn is_multiplicative(&self) -> bool {
        matches!(self, OperatorPrecedence::Multiplicative)
    }
}

pub fn inner_string_text(token: &PsqlSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if matches!(token.kind(), PsqlSyntaxKind::PSQL_STRING_LITERAL) {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
