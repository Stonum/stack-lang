use crate::formatter::prelude::*;

use crate::syntax::MSyntaxKind::{M_LONG_STRING_LITERAL, M_STRING_LITERAL};
use crate::syntax::MSyntaxToken;
use std::borrow::Cow;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variant to track tokens that are inside an expression
    Expression,
    /// Variant to track tokens that are inside a member
    Member,
    /// Variant used when the string literal is inside a directive. This will apply
    /// a simplified logic of normalisation
    Directive,
}

/// Data structure of convenience to format string literals
pub(crate) struct FormatLiteralStringToken<'token> {
    /// The current token
    token: &'token MSyntaxToken,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token MSyntaxToken, _parent_kind: StringLiteralParentKind) -> Self {
        Self { token }
    }

    fn token(&self) -> &'token MSyntaxToken {
        self.token
    }

    pub fn clean_text(&self) -> CleanedStringLiteralText {
        let token = self.token();
        debug_assert!(
            matches!(token.kind(), M_STRING_LITERAL | M_LONG_STRING_LITERAL),
            "Found kind {:?}",
            token.kind()
        );

        let content: Cow<'token, str> = Cow::Borrowed(self.token().text_trimmed());
        let normalized_text_width = content.len();

        CleanedStringLiteralText {
            text: content,
            width: normalized_text_width,
            token,
        }
    }
}

pub(crate) struct CleanedStringLiteralText<'a> {
    token: &'a MSyntaxToken,
    text: Cow<'a, str>,
    width: usize,
}

impl CleanedStringLiteralText<'_> {
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Format<MFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl Format<MFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let cleaned = self.clean_text();

        cleaned.fmt(f)
    }
}
