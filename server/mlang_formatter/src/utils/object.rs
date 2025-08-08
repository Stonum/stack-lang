use super::FormatLiteralStringToken;
use super::StringLiteralParentKind;

use crate::prelude::*;
use mlang_syntax::AnyMObjectMemberName;
use mlang_syntax::MSyntaxKind::M_STRING_LITERAL;
use biome_formatter::write;
use biome_rowan::AstNode;

impl Format<MFormatContext> for AnyMObjectMemberName {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        self.format().fmt(f)
    }
}

pub(crate) fn write_member_name(
    name: &AnyMObjectMemberName,
    f: &mut MFormatter,
) -> FormatResult<usize> {
    match name {
        AnyMObjectMemberName::MLiteralMemberName(literal) => {
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
