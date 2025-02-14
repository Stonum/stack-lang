use super::binding::{parse_binding, parse_binding_pattern};
use super::class::parse_initializer_clause;
use super::expr::{parse_doc_string_expression, ExpressionContext};
use super::m_parse_error;
use super::m_parse_error::{expected_binding, expected_parameter};
use super::state::{EnterFunction, EnterParameters, SignatureFlags};
use super::stmt::{parse_block_impl, StatementContext};
use super::ParsedSyntax::{Absent, Present};
use super::{MParser, ParseRecoveryTokenSet, ParsedSyntax};

use super::syntax::MSyntaxKind::*;
use super::syntax::{MSyntaxKind, T};

use biome_parser::prelude::*;
use biome_parser::ParserProgress;

/// A function declaration. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test function_decl
// function foo1() {}
// async function foo4() {}
//
// test function_declaration_script
// // SCRIPT
// function test(await) {}
//
// test_err function_decl_err
// function() {}
// function foo {}
// function {}
//
// test_err function_broken
// function foo())})}{{{  {}
pub(crate) fn parse_function_declaration(
    p: &mut MParser,
    context: StatementContext,
) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    let function = parse_function(
        p,
        m,
        FunctionKind::Declaration {
            single_statement_context: context.is_single_statement(),
        },
    );

    Present(function)
}

pub(crate) fn parse_function_expression(p: &mut MParser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    Present(parse_function(p, m, FunctionKind::Expression))
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
    Declaration { single_statement_context: bool },
    Expression,
}

impl FunctionKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, FunctionKind::Expression)
    }
}

impl From<FunctionKind> for MSyntaxKind {
    fn from(kind: FunctionKind) -> Self {
        match kind {
            FunctionKind::Declaration { .. } => M_FUNCTION_DECLARATION,
            FunctionKind::Expression => M_FUNCTION_EXPRESSION,
        }
    }
}

fn is_at_function(p: &mut MParser) -> bool {
    p.at_ts(token_set![T![function]])
}

#[inline]
fn parse_function(p: &mut MParser, m: Marker, kind: FunctionKind) -> CompletedMarker {
    let flags = SignatureFlags::empty();

    p.expect(T![function]);

    let id = parse_function_id(p, kind, flags);

    if !kind.is_id_optional() {
        id.or_add_diagnostic(p, |p, range| {
            p.err_builder(
                "expected a name for the function in a function declaration, but found none",
                range,
            )
        });
    }

    parse_parameter_list(p, flags).or_add_diagnostic(p, m_parse_error::expected_parameters);

    if let Present(expr) = parse_doc_string_expression(p) {
        match kind {
            FunctionKind::Expression { .. } => {
                p.error(p.err_builder(
                    "expected function body, but found string expression",
                    expr.range(p),
                ));
            }
            _ => (),
        }
    }

    let body = parse_function_body(p, flags);

    {
        body.or_add_diagnostic(p, m_parse_error::expected_function_body);

        let function = m.complete(p, kind.into());

        function
    }
}

// test_err break_in_nested_function
// while (true) {
//   function helper() {
//     break;
//   }
// }
pub(crate) fn parse_function_body(p: &mut MParser, flags: SignatureFlags) -> ParsedSyntax {
    p.with_state(EnterFunction(flags), |p| {
        parse_block_impl(p, M_FUNCTION_BODY)
    })
}

fn parse_function_id(p: &mut MParser, kind: FunctionKind, flags: SignatureFlags) -> ParsedSyntax {
    match kind {
        FunctionKind::Expression => p.with_state(EnterFunction(flags), parse_binding),
        _ => parse_binding(p),
    }
}

pub(crate) fn parse_any_parameter(
    p: &mut MParser,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    let parameter = match p.cur() {
        T![...] => parse_rest_parameter(p),
        _ => parse_formal_parameter(p, expression_context),
    };

    parameter
}

pub(crate) fn parse_rest_parameter(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);
    parse_binding_pattern(p).or_add_diagnostic(p, expected_binding);

    let mut valid = true;

    if let Present(initializer) = parse_initializer_clause(p, ExpressionContext::default()) {
        // test_err arrow_rest_in_expr_in_initializer
        // for ((...a = "b" in {}) => {};;) {}
        let err = p.err_builder(
            "rest elements may not have default initializers",
            initializer.range(p),
        );

        p.error(err);
        valid = false;
    }

    let mut rest_parameter = m.complete(p, M_REST_PARAMETER);

    if p.at(T![,]) {
        let err = p.err_builder(
            "rest elements may not have trailing commas",
            rest_parameter.range(p),
        );

        p.error(err);
        valid = false;
    }

    if !valid {
        rest_parameter.change_to_bogus(p);
    }

    Present(rest_parameter)
}

pub(crate) fn parse_formal_parameter(
    p: &mut MParser,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    // we use a checkpoint to avoid bogus nodes if the binding pattern fails to parse.
    let checkpoint = p.checkpoint();

    let m = p.start();

    if let Present(_) = parse_binding_pattern(p) {
        parse_initializer_clause(p, expression_context).ok();

        let parameter = m.complete(p, M_FORMAL_PARAMETER);

        Present(parameter)
    } else {
        m.abandon(p);
        p.rewind(checkpoint);
        Absent
    }
}

// test parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(crate) fn parse_parameter_list(p: &mut MParser, flags: SignatureFlags) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    parse_parameters_list(
        p,
        flags,
        |p, expression_context| parse_any_parameter(p, expression_context),
        M_PARAMETER_LIST,
    );

    Present(m.complete(p, M_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(crate) fn parse_parameters_list(
    p: &mut MParser,
    flags: SignatureFlags,
    parse_parameter: impl Fn(&mut MParser, ExpressionContext) -> ParsedSyntax,
    list_kind: MSyntaxKind,
) {
    let mut first = true;
    let has_l_paren = p.expect(T!['(']);

    p.with_state(EnterParameters(flags), |p| {
        let parameters_list = p.start();
        let mut progress = ParserProgress::default();

        while !p.at(EOF) && !p.at(T![')']) {
            if first {
                first = false;
            } else {
                p.expect(T![,]);
            }

            if p.at(T![')']) {
                break;
            }

            progress.assert_progressing(p);

            let parameter = parse_parameter(
                p,
                ExpressionContext::default().and_object_expression_allowed(!first || has_l_paren),
            );

            if parameter.is_absent() && p.at(T![,]) {
                // a missing parameter,
                parameter.or_add_diagnostic(p, expected_parameter);
                continue;
            }

            // test_err formal_params_no_binding_element
            // function foo(true) {}

            // test_err formal_params_invalid
            // function (a++, c) {}
            let recovered_result = parameter.or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    M_BOGUS_PARAMETER,
                    token_set![T![ident], T![,], T!['['], T![...], T!['{'], T![')'], T![;],],
                )
                .enable_recovery_on_line_break(),
                m_parse_error::expected_parameter,
            );

            if recovered_result.is_err() {
                break;
            }
        }

        parameters_list.complete(p, list_kind);
    });

    p.expect(T![')']);
}
