use crate::syntax::{inner_string_text, MDirective};

use biome_rowan::{SyntaxResult, TokenText};

impl MDirective {
    /// Get the inner text of a string not including the quotes
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}
