use super::FormatLiteralStringToken;
use super::StringLiteralParentKind;

use crate::formatter::prelude::*;
use crate::syntax::MSyntaxKind::M_STRING_LITERAL;
use crate::syntax::{AnyMClassMemberName, AnyMObjectMemberName};
use biome_formatter::write;
use biome_rowan::{declare_node_union, AstNode};

declare_node_union! {
    pub(crate) AnyMMemberName = AnyMObjectMemberName | AnyMClassMemberName
}

impl Format<MFormatContext> for AnyMMemberName {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            Self::AnyMObjectMemberName(name) => name.format().fmt(f),
            Self::AnyMClassMemberName(name) => name.format().fmt(f),
        }
    }
}

pub(crate) fn write_member_name(name: &AnyMMemberName, f: &mut MFormatter) -> FormatResult<usize> {
    match name {
        name @ (AnyMMemberName::AnyMClassMemberName(AnyMClassMemberName::MLiteralMemberName(
            literal,
        ))
        | AnyMMemberName::AnyMObjectMemberName(AnyMObjectMemberName::MLiteralMemberName(
            literal,
        ))) => {
            let value = literal.value()?;

            if value.kind() == M_STRING_LITERAL {
                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text();

                write!(
                    f,
                    [
                        format_leading_comments(name.syntax()),
                        cleaned,
                        format_trailing_comments(name.syntax())
                    ]
                )?;

                Ok(cleaned.width())
            } else {
                write!(f, [name])?;

                Ok(value.text_trimmed().len())
            }
        }
        name => {
            write!(f, [&name])?;
            Ok(name.text().len())
        }
    }
}
