use biome_parser::prelude::*;

pub fn invalid_digits_after_escape_sequence(start: usize, end: usize) -> ParseDiagnostic {
    ParseDiagnostic::new("invalid digits after escape sequence", start..end)
        .with_hint("expected valid escape sequence")
}
