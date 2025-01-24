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

pub fn expected_arrow_body(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function body", "expression"], range, p)
}

pub fn expected_object_member(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "shorthand property"], range, p)
}
pub fn expected_array_element(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression"], range, p)
}

pub fn expected_object_member_name(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "identifier",
            "string literal",
            "number literal",
            "computed property",
        ],
        range,
        p,
    )
}

pub fn expected_block_statement(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range, p)
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

pub fn expected_assignment_target(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "assignment target"], range, p)
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

pub fn expected_module_source(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("string literal", range, p)
}

pub fn duplicate_assertion_keys_error(
    p: &MParser,
    key: &str,
    first_use: TextRange,
    duplicate_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Duplicate assertion keys are not allowed", first_use)
        .with_detail(first_use, format!("First use of the key `{key}`"))
        .with_detail(duplicate_range, "second use here")
}

pub fn expected_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range, p)
}

pub fn expected_expression_assignment(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range, p)
}

pub fn expected_unary_expression(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_node("unary expression", range, p)
}

pub fn expected_property_or_signature(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "signature"], range, p)
}

pub fn expected_declaration(p: &MParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function", "class", "variable declaration"], range, p)
}

pub fn unexpected_body_inside_ambient_context(p: &MParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "members inside ambient contexts should not have a body",
        range,
    )
}

pub fn private_names_only_allowed_on_left_side_of_in_expression(
    p: &MParser,
    private_name_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Private names are only allowed on the left side of a 'in' expression",
        private_name_range,
    )
}

pub fn invalid_assignment_error(p: &MParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        format!("Invalid assignment to `{}`", p.text(range.as_range()),),
        range,
    )
    .with_hint("This expression cannot be assigned to")
}

pub fn modifier_already_seen(
    p: &MParser,
    second_range: TextRange,
    first_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(second_range);
    p.err_builder(format!("'{modifier}' already seen"), second_range)
        .with_detail(second_range, "duplicate modifier")
        .with_detail(first_range, "first seen here")
}

pub fn modifier_cannot_be_used_with_modifier(
    p: &MParser,
    range: TextRange,
    other_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.text(range);
    let other_modifier = p.text(other_modifier_range);

    p.err_builder(
        format!("'{modifier}' cannot be used with '{other_modifier}' modifier."),
        range,
    )
    .with_detail(range, format!("'{modifier}' modifier"))
    .with_detail(other_modifier_range, format!("'{other_modifier}' modifier"))
}

pub fn modifier_must_precede_modifier(
    p: &MParser,
    range: TextRange,
    to_precede_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier_name = p.text(range);
    let to_precede_name = p.text(to_precede_modifier_range);

    p.err_builder(
        format!("'{modifier_name}' must precede '{to_precede_name}'",),
        range,
    )
    .with_detail(range, "move this modifier")
    .with_detail(to_precede_modifier_range, "before this modifier")
}
