use super::binding::{parse_dot_binding, parse_identifier_binding};
use super::class::parse_initializer_clause;
use super::expr::{parse_doc_string_expression, ExpressionContext};
use super::m_parse_error;
use super::m_parse_error::expected_parameter;
use super::state::{EnterFunction, EnterParameters, SignatureFlags};
use super::stmt::{parse_block_impl, StatementContext};
use super::ParsedSyntax::{Absent, Present};
use super::{MParser, ParseRecoveryTokenSet, ParsedSyntax};

use super::syntax::MSyntaxKind::*;
use super::syntax::{MSyntaxKind, T};

use biome_parser::prelude::*;
use biome_parser::ParserProgress;

/// A function declaration
// test function_decl
// function foo1() {}
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
    annotation: Option<CompletedMarker>,
) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let a = annotation.unwrap_or_else(|| {
        let a = p.start();
        a.complete(p, M_ANNOTATION_GROUP_LIST)
    });
    let m = a.precede(p);

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
        if let FunctionKind::Expression { .. } = kind {
            p.error(p.err_builder(
                "expected function body, but found string expression",
                expr.range(p),
            ));
        }
    }

    let body = parse_function_body(p, flags);

    {
        body.or_add_diagnostic(p, m_parse_error::expected_function_body);

        m.complete(p, kind.into())
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
        FunctionKind::Expression => p.with_state(EnterFunction(flags), parse_identifier_binding),
        _ => {
            let ident = parse_identifier_binding(p);
            if let Present(lhs) = ident {
                let mut progress = ParserProgress::default();
                let mut lhs = lhs;
                while !p.at(EOF) {
                    progress.assert_progressing(p);
                    lhs = match p.cur() {
                        T![.] => parse_dot_binding(p, lhs, T![.]).unwrap(),
                        _ => {
                            break;
                        }
                    };
                }
                Present(lhs)
            } else {
                Absent
            }
        }
    }
}

pub(crate) fn parse_any_parameter(
    p: &mut MParser,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    match p.cur() {
        T![...] => parse_rest_parameter(p),
        _ => parse_formal_parameter(p, expression_context),
    }
}

pub(crate) fn parse_rest_parameter(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);
    let _parsed = parse_identifier_binding(p);

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

    if let Present(_) = parse_identifier_binding(p) {
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
    parse_parameters_list(p, flags, parse_any_parameter, M_PARAMETER_LIST);

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
