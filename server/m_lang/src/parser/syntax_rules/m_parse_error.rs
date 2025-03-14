//! Provides factory function to create common diagnostics for the JavaScript syntax

use super::span::Span;
use super::MParser;

use biome_parser::diagnostic::{expected_any, expected_node};
use biome_parser::prelude::*;
use biome_rowan::TextRange;

pub fn expected_function_body(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("function body", range, p)
}

pub fn expected_class_member_name(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "string literal"], range, p)
}

pub fn expected_object_member(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "shorthand property"], range, p)
}
pub fn expected_array_element(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression"], range, p)
}

pub fn expected_block_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range, p)
}

pub fn expected_annotation_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("annotation statement", range, p)
}

pub fn expected_catch_clause(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("catch clause", range, p)
}

pub fn expected_parameter(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parameter", range, p)
}

pub fn expected_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("parenthesis '('", range, p)
}

pub fn expected_case_or_default(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["default", "case"], range, p)
}

pub fn expected_case(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("case", range, p)
}

pub fn expected_simple_assignment_target(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "member expression"], range, p)
}

pub fn expected_identifier(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range, p)
}

pub fn expected_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range, p)
}

pub fn expected_binding(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "array pattern", "object pattern"], range, p)
}

pub fn expected_class_member(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range, p)
}

pub fn expected_class_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class parameters", range, p)
}

pub fn expected_constructor_parameters(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("constructor parameters", range, p)
}

pub fn expected_class_method_body(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("class method body", range, p)
}

pub fn expected_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub fn expected_literal_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("literal expression", range, p)
}

pub fn expected_expression_assignment(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range, p)
}

pub fn expected_declaration(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function", "class", "variable declaration"], range, p)
}

pub fn invalid_assignment_error(p: &MParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid assignment to `{}`", p.text(range.as_range()),),
        range,
    )
    .with_hint("This expression cannot be assigned to")
}
