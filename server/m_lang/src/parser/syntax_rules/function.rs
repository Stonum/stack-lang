use super::binding::{is_at_identifier_binding, parse_binding, parse_binding_pattern};
use super::class::parse_initializer_clause;
use super::expr::ExpressionContext;
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

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test js function_decl
// function foo1() {}
// function *foo2() {}
// async function *foo3() {}
// async function foo4() {}
// function *foo5() {
//   yield foo;
// }
//
// test js function_declaration_script
// // SCRIPT
// function test(await) {}
//
// test_err js function_decl_err
// function() {}
// function foo {}
// function {}
// function *() {}
// async function() {}
// async function *() {}
// function *foo2() {}
// yield foo3;
// function test2(): number {}
// function foo4(await) {}
// function foo5(yield) {}
//
// test_err js function_broken
// function foo())})}{{{  {}
//
// test ts ts_function_statement
// function test(a: string, b?: number, c="default") {}
// function test2<A, B extends A, C = A>(a: A, b: B, c: C) {}
pub(super) fn parse_function_declaration(
    p: &mut MParser,
    context: StatementContext,
) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    let mut function = {
        parse_function(
            p,
            m,
            FunctionKind::Declaration {
                single_statement_context: context.is_single_statement(),
            },
        )
    };

    Present(function)
}

pub(super) fn parse_function_expression(p: &mut MParser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    Present(parse_function(p, m, FunctionKind::Expression))
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum AmbientFunctionKind {
    Declaration,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
    Declaration {
        // https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-functiondeclarations-in-ifstatement-statement-clauses
        single_statement_context: bool,
    },
    Expression,
}

impl FunctionKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, FunctionKind::Expression)
    }

    fn is_expression(&self) -> bool {
        matches!(self, FunctionKind::Expression)
    }

    fn is_in_single_statement_context(&self) -> bool {
        matches!(
            self,
            FunctionKind::Declaration {
                single_statement_context: true
            }
        )
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
    let mut flags = SignatureFlags::empty();

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

    let parameter_context = if !kind.is_expression() {
        // It isn't known at this point if this is a function overload definition (body is missing)
        // or a regular function implementation.
        // Let's go with the laxer of the two. Ideally, these verifications should be part of
        // a second compiler pass.
        ParameterContext::Declaration
    } else {
        ParameterContext::Implementation
    };

    parse_parameter_list(p, parameter_context, flags)
        .or_add_diagnostic(p, m_parse_error::expected_parameters);

    let body = parse_function_body(p, flags);

    // test ts ts_function_overload
    // function test(a: string): void;
    // function test(a: string | undefined): void {}
    // function no_semi(a: string)
    // function no_semi(a: string) {}
    // async function async_overload(a: string)
    // async function async_overload(a: string) {}
    {
        body.or_add_diagnostic(p, m_parse_error::expected_function_body);

        let mut function = m.complete(p, kind.into());

        function
    }
}

// test_err js break_in_nested_function
// while (true) {
//   function helper() {
//     break;
//   }
// }
pub(super) fn parse_function_body(p: &mut MParser, flags: SignatureFlags) -> ParsedSyntax {
    p.with_state(EnterFunction(flags), |p| {
        parse_block_impl(p, M_FUNCTION_BODY)
    })
}

fn parse_function_id(p: &mut MParser, kind: FunctionKind, flags: SignatureFlags) -> ParsedSyntax {
    match kind {
        // Takes the async and generator restriction from the expression
        FunctionKind::Expression => {
            // test js function_expression_id
            // // SCRIPT
            // (function await() {});
            // (function yield() {});
            // (async function yield() {});
            // (function* await() {})
            //
            // test_err js function_expression_id_err
            // (async function await() {});
            // (function* yield() {});
            // function* test() { function yield() {} }
            p.with_state(EnterFunction(flags), parse_binding)
        }
        // Inherits the async and generator from the parent
        _ => {
            // test js function_id
            // // SCRIPT
            // function test() {}
            // function await(test) {}
            // async function await(test) {}
            // function yield(test) {}
            // function* yield(test) {}
            //
            //
            // test_err js function_id_err
            // function* test() {
            //   function yield(test) {}
            // }
            parse_binding(p)
        }
    }
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq, Eq)]
pub(crate) enum LineBreak {
    // check line breaks
    DoCheck,
    // do not check line break
    DoNotCheck,
}

/// There are cases where the parser must speculatively parse a syntax. For example,
/// parsing `<string>(test)` very much looks like an arrow expression *except* that it isn't followed
/// by a `=>`. This enum tells a parse function if ambiguity should be tolerated or if it should stop if it is not.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Ambiguity {
    /// Ambiguity is allowed. A parse method should continue even if an expected character is missing.
    Allowed,

    /// Ambiguity isn't allowed. A parse method should stop parsing if an expected character is missing
    /// and let the caller decide what to do in this case.
    Disallowed,
}

impl Ambiguity {
    fn is_disallowed(&self) -> bool {
        matches!(self, Ambiguity::Disallowed)
    }
}

pub(crate) fn parse_any_parameter(
    p: &mut MParser,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    let parameter = match p.cur() {
        T![...] => parse_rest_parameter(p, expression_context),
        _ => parse_formal_parameter(p, parameter_context, expression_context),
    };

    parameter
}

pub(crate) fn parse_rest_parameter(
    p: &mut MParser,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);
    parse_binding_pattern(p, expression_context).or_add_diagnostic(p, expected_binding);

    let mut valid = true;

    if let Present(initializer) = parse_initializer_clause(p, ExpressionContext::default()) {
        // test_err js arrow_rest_in_expr_in_initializer
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ParameterContext {
    /// Regular parameter in a class method implementation: `class A { method(a) {} }`
    ClassImplementation,

    /// Regular parameter in a function / method implementation: `function x(a) {}`
    Implementation,

    /// Parameter of a function/method declaration: `declare function x(a);`
    Declaration,

    /// Parameter of a setter function: `set a(b: string)`
    Setter,

    /// Parameter of a class setter method: `class A { set a(b: string) }`
    ClassSetter,

    /// Parameter of an arrow function
    Arrow,

    /// Parameter inside a TS property parameter: `constructor(private a)`
    ParameterProperty,
}

impl ParameterContext {
    pub fn is_any_setter(&self) -> bool {
        self.is_setter() || self.is_class_setter()
    }

    pub fn is_setter(&self) -> bool {
        self == &ParameterContext::Setter
    }

    pub fn is_class_setter(&self) -> bool {
        self == &ParameterContext::ClassSetter
    }

    pub fn is_class_method_implementation(&self) -> bool {
        self == &ParameterContext::ClassImplementation
    }

    pub fn is_any_class_method(&self) -> bool {
        self.is_class_method_implementation() || self.is_class_setter()
    }

    pub fn is_parameter_property(&self) -> bool {
        self == &ParameterContext::ParameterProperty
    }

    pub fn is_arrow_function(&self) -> bool {
        self == &ParameterContext::Arrow
    }
}

// test ts ts_formal_parameter
// function a(x) {}
// function b({ x, y } = {}) {}
// function c(x: string, y?: number, z: string = "test") {}
//
// test_err ts ts_formal_parameter_error
// function a(x?: string = "test") {}
// function b(...rest: string[] = "init") {}
// function c(...rest, b: string) {}
//
// test_err js m_formal_parameter_error
// function a(x: string) {}
// function b(x?) {}
pub(crate) fn parse_formal_parameter(
    p: &mut MParser,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
) -> ParsedSyntax {
    // test ts ts_formal_parameter_decorator { "parse_class_parameter_decorators": true }
    // class Foo {
    //    constructor(@dec x) {}
    //    method(@dec x) {}
    // }

    // test_err ts ts_formal_parameter_decorator_option
    // class Foo {
    //    constructor(@dec x) {}
    //    method(@dec x) {}
    // }

    // test_err ts ts_formal_parameter_decorator { "parse_class_parameter_decorators": true }
    // function a(@dec x) {}

    // we use a checkpoint to avoid bogus nodes if the binding pattern fails to parse.
    let checkpoint = p.checkpoint();

    let m = p.start();

    if let Present(binding) = parse_binding_pattern(p, expression_context) {
        let binding_kind = binding.kind(p);
        let binding_range = binding.range(p);

        let mut valid = true;

        // test ts ts_parameter_option_binding_pattern
        // declare namespace A {
        //   export class Ajv {
        //     errorsText(errors?: string[] | null | undefined, { separator, dataVar }?: ErrorsTextOptions): string;
        //   }
        // }
        // if valid
        //     && matches!(
        //         binding_kind,
        //         M_OBJECT_BINDING_PATTERN | M_ARRAY_BINDING_PATTERN
        //     )
        //     && parameter_context.is_parameter_property()
        // {
        //     valid = false;
        //     p.error(p.err_builder(
        //         "A parameter property may not be declared using a binding pattern.",
        //         binding_range,
        //     ));
        // }

        if let Present(initializer) = parse_initializer_clause(p, expression_context) {}

        let mut parameter = m.complete(p, M_FORMAL_PARAMETER);

        if !valid {
            parameter.change_to_bogus(p);
        }

        Present(parameter)
    } else {
        m.abandon(p);
        p.rewind(checkpoint);
        Absent
    }
}

/// Skips over the binding token of a parameter. Useful in the context of lookaheads to determine
/// if any typescript specific syntax like `:` is present after the parameter name.
/// Returns `true` if the function skipped over a valid binding, returns false if the parser
/// is not positioned at a binding.
pub(super) fn skip_parameter_start(p: &mut MParser) -> bool {
    if is_at_identifier_binding(p) || p.at(T![this]) {
        p.bump_any();
        return true;
    }

    if p.at(T!['[']) || p.at(T!['{']) {
        // Array or object pattern. Try to parse it and return true if there were no parsing errors
        let previous_error_count = p.context().diagnostics().len();
        let pattern = parse_binding_pattern(p, ExpressionContext::default());
        pattern.is_present() && p.context().diagnostics().len() == previous_error_count
    } else {
        false
    }
}

// test js parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(
    p: &mut MParser,
    parameter_context: ParameterContext,
    flags: SignatureFlags,
) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    parse_parameters_list(
        p,
        flags,
        |p, expression_context| parse_any_parameter(p, parameter_context, expression_context),
        M_PARAMETER_LIST,
    );

    Present(m.complete(p, M_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
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

            // test_err js formal_params_no_binding_element
            // function foo(true) {}

            // test_err js formal_params_invalid
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
