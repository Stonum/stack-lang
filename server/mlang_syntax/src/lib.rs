#[macro_use]
mod generated;
pub mod binary_like_expression;
pub mod expression_left_side;
mod ext;
mod file_source;
pub mod parentheses;
mod static_value;
mod syntax_node;

use biome_rowan::RawSyntaxKind;
pub use biome_rowan::{
    AstNode, Language, SendNode, SyntaxNode, TextLen, TextRange, TextSize, TokenAtOffset,
    TokenText, TriviaPieceKind, WalkEvent,
};

pub use crate::ext::*;
pub use crate::file_source::*;
use crate::generated::kind::MSyntaxKind::*;
pub use crate::generated::*;
pub use crate::parentheses::*;
pub use crate::syntax_node::*;

impl From<u16> for MSyntaxKind {
    fn from(d: u16) -> MSyntaxKind {
        assert!(d <= (MSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, MSyntaxKind>(d) }
    }
}

impl From<MSyntaxKind> for u16 {
    fn from(k: MSyntaxKind) -> u16 {
        k as u16
    }
}

impl MSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (MSyntaxKind::SET_KW as u16)
            && (self as u16) >= (MSyntaxKind::BREAK_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for MSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            M_BOGUS
                | M_BOGUS_STATEMENT
                | M_BOGUS_PARAMETER
                | M_BOGUS_BINDING
                | M_BOGUS_MEMBER
                | M_BOGUS_EXPRESSION
                | M_BOGUS_ASSIGNMENT
        )
    }

    fn to_bogus(&self) -> MSyntaxKind {
        match self {
            kind if AnyMExpression::can_cast(*kind) => M_BOGUS_EXPRESSION,
            kind if AnyMBinding::can_cast(*kind) => M_BOGUS_BINDING,
            kind if AnyMClassMember::can_cast(*kind) || AnyMObjectMember::can_cast(*kind) => {
                M_BOGUS_MEMBER
            }
            kind if AnyMAssignment::can_cast(*kind) => M_BOGUS_ASSIGNMENT,
            kind if AnyMParameter::can_cast(*kind) => M_BOGUS_PARAMETER,
            _ => M_BOGUS,
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
        AnyMRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        MSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            MSyntaxKind::NEWLINE | MSyntaxKind::WHITESPACE | MSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        MSyntaxKind::to_string(self)
    }
}

impl TryFrom<MSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: MSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            MSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
            MSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
            MSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
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
    // `new` without arguments list
    NewWithoutArguments = 18,
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
    pub fn try_from_binary_operator(kind: MSyntaxKind) -> Option<OperatorPrecedence> {
        Some(match kind {
            T![||] | T![or] => OperatorPrecedence::LogicalOr,
            T![&&] | T![and] => OperatorPrecedence::LogicalAnd,
            T![|] => OperatorPrecedence::BitwiseOr,
            T![^] => OperatorPrecedence::BitwiseXor,
            T![&] => OperatorPrecedence::BitwiseAnd,
            T![==] | T![!=] => OperatorPrecedence::Equality,
            T![<] | T![>] | T![<=] | T![>=] | T![in] | T![include] => {
                OperatorPrecedence::Relational
            }
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

pub fn inner_string_text(token: &MSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if matches!(token.kind(), MSyntaxKind::M_STRING_LITERAL) {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
