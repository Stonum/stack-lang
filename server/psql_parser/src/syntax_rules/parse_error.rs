//! Provides factory function to create common diagnostics for the JavaScript syntax

use crate::PsqlParser;

use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub fn expected_statement(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub fn expected_expression(p: &PsqlParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}
