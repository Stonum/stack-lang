use crate::prelude::*;
use mlang_syntax::MSyntaxToken;

use biome_formatter::trivia::format_skipped_token_trivia;
use biome_formatter::{format_args, write};

use crate::MFormatContext;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSyntaxToken;
impl_format!(MSyntaxToken, FormatMSyntaxToken);

impl FormatRule<MSyntaxToken> for FormatMSyntaxToken {
    type Context = MFormatContext;

    fn fmt(&self, token: &MSyntaxToken, f: &mut MFormatter) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(f, [format_skipped_token_trivia(token),])?;

        if token.kind().is_keyword() {
            if let Some(normilized_token_text) = self.normalize_keyword(token) {
                let start = token.text_trimmed_range().start();
                return write!(
                    f,
                    [&format_args!(dynamic_text(normilized_token_text, start))]
                );
            }
        }
        write!(f, [format_trimmed_token(token)])
    }
}

impl FormatMSyntaxToken {
    fn normalize_keyword(&self, token: &MSyntaxToken) -> Option<&'static str> {
        if matches!(token.text_trimmed(), "&&" | "||") {
            return None;
        }

        match token.text_trimmed().is_ascii() {
            true => token.kind().to_en_keyword(),
            false => token.kind().to_ru_keyword(),
        }
    }
}
