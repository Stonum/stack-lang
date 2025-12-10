use biome_rowan::SyntaxNode;
use line_index::LineIndex;
use mlang_syntax::{MLanguage, MSyntaxKind, T};
use tower_lsp::lsp_types::{Range, SemanticToken, SemanticTokenModifier, SemanticTokenType};

pub const SEMANTIC_TOKEN_MAP: [SemanticTokenType; 1] = [SemanticTokenType::KEYWORD];

pub const SEMANTIC_TOKEN_MODIFIERS: [SemanticTokenModifier; 0] = [];

#[inline]
fn kind_to_type(token: MSyntaxKind) -> Option<(u32, u32)> {
    match token {
        // Don't override tm grammar
        T![function] | T![class] | T![get] | T![set] => None,
        T![var] => None,
        T![new] | T![this] | T![super] | T![extends] => None,
        T![true] | T![false] | T![null] => None,

        // keywords
        _ if token.is_keyword() => Some((0, 0)),
        _ => None,
    }
}

pub fn semantic_tokens(
    syntax: SyntaxNode<MLanguage>,
    line_index: &LineIndex,
    tokens_range: Option<Range>,
) -> Vec<SemanticToken> {
    let (mut prev_line, mut prev_col) = (0, 0);

    syntax
        .descendants_tokens(biome_rowan::Direction::Next)
        .filter_map(|token| {
            let (token_type, token_modifiers) = kind_to_type(token.kind())?;
            let range = token.text_trimmed_range();
            let range = line_index.line_col_range(range)?;

            // filter for requested range
            if let Some(tokens_range) = tokens_range
                && (tokens_range.start.line > range.start.line
                    || tokens_range.end.line < range.end.line)
                {
                    return None;
                }

            Some((token_type, token_modifiers, range))
        })
        .map(|(token_type, token_modifiers, range)| {
            let (line, col) = (range.start.line, range.start.col);
            let length = range.end.col - col; // we has not supported multiline tokens

            let delta_line = line.saturating_sub(prev_line);
            let delta_start = if line == prev_line {
                col.saturating_sub(prev_col)
            } else {
                col
            };

            (prev_line, prev_col) = (line, col);
            SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type,
                token_modifiers_bitset: token_modifiers,
            }
        })
        .collect()
}
