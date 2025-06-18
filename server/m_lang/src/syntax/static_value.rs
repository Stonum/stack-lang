use biome_rowan::TextRange;

use super::{MSyntaxKind, MSyntaxToken};

#[derive(Debug, Clone, Eq, PartialEq)]
/// static values defined in mlang expressions
pub enum StaticValue {
    Boolean(MSyntaxToken),
    Null(MSyntaxToken),
    Number(MSyntaxToken),
    // The string can be empty.
    String(MSyntaxToken),
    /// This is used to represent the empty template string.
    EmptyString(TextRange),
    Date(MSyntaxToken),
    Time(MSyntaxToken),
}

impl StaticValue {
    /// Return `true` if the value is falsy
    pub fn is_falsy(&self) -> bool {
        match self {
            StaticValue::Boolean(token) => token.text_trimmed() == "false",
            StaticValue::Null(_) | StaticValue::EmptyString(_) => true,
            StaticValue::Number(token) => token.text_trimmed() == "0",
            StaticValue::String(_) => self.text().is_empty(),
            StaticValue::Date(token) => token.text_trimmed() == "00.00.0000",
            StaticValue::Time(token) => {
                token.text_trimmed() == "00:00:00" || token.text_trimmed() == "00:00"
            }
        }
    }

    /// Return a string of the static value
    pub fn text(&self) -> &str {
        match self {
            StaticValue::Boolean(token)
            | StaticValue::Null(token)
            | StaticValue::Number(token)
            | StaticValue::String(token)
            | StaticValue::Date(token)
            | StaticValue::Time(token) => {
                let text = token.text_trimmed();
                if matches!(
                    token.kind(),
                    MSyntaxKind::M_STRING_LITERAL | MSyntaxKind::M_LONG_STRING_LITERAL
                ) {
                    // SAFETY: string literal token have a delimiters at the start and the end of the string
                    return &text[1..text.len() - 1];
                }
                text
            }
            StaticValue::EmptyString(_) => "",
        }
    }

    /// Return teh range of the static value.
    pub fn range(&self) -> TextRange {
        match self {
            StaticValue::Boolean(token)
            | StaticValue::Null(token)
            | StaticValue::Number(token)
            | StaticValue::String(token)
            | StaticValue::Date(token)
            | StaticValue::Time(token) => token.text_trimmed_range(),
            StaticValue::EmptyString(range) => *range,
        }
    }

    /// Return `true` if the static value doesn't match the given string value and it is
    pub fn is_not_string_constant(&self, text: &str) -> bool {
        match self {
            StaticValue::String(_) | StaticValue::EmptyString(_) => self.text() != text,
            _ => false,
        }
    }

    /// Return a string if the static value is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    pub fn as_string_constant(&self) -> Option<&str> {
        match self {
            StaticValue::String(_) | StaticValue::EmptyString(_) => Some(self.text()),
            _ => None,
        }
    }

    /// Return `true` if the value is null
    pub fn is_null_or_undefined(&self) -> bool {
        matches!(self, StaticValue::Null(_))
    }
}

impl AsRef<str> for StaticValue {
    fn as_ref(&self) -> &str {
        self.text()
    }
}
