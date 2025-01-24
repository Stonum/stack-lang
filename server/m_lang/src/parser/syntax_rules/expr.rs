//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.

use std::ops::{BitOr, BitOrAssign, Sub};

use crate::lexer::MReLexContext;

use super::rewrite::rewrite_events;
use super::rewrite::RewriteParseEvents;
use super::rewrite_parser::{RewriteMarker, RewriteParser};
use super::{MParserCheckpoint, RecoveryResult};

use super::assignment::parse_assignment;
use super::assignment::AssignmentExprPrecedence;
use super::assignment::{expression_to_assignment, expression_to_assignment_pattern};

use super::function::parse_function_expression;
use super::m_parse_error;
use super::m_parse_error::{
    expected_expression, expected_identifier, expected_simple_assignment_target,
    invalid_assignment_error,
};
use super::object::{parse_hashmap_expression, parse_object_expression};
use super::stmt::STMT_RECOVERY_SET;

use super::syntax::{MSyntaxKind::*, *};
use super::ParsedSyntax::{Absent, Present};
use super::{MParser, ParseRecoveryTokenSet, ParsedSyntax};

use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::*;
use biome_parser::ParserProgress;

use enumflags2::{bitflags, make_bitflags, BitFlags};

pub const EXPR_RECOVERY_SET: TokenSet<MSyntaxKind> =
    token_set![VAR_KW, R_PAREN, L_PAREN, L_BRACK, R_BRACK];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct ExpressionContext(ExpressionContextFlags);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ExpressionContextFlag {
    IncludeIn = 1 << 0,
    AllowObjectExpression = 1 << 1,
    InDecorator = 1 << 2,
    AllowTSTypeAssertion = 1 << 3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ExpressionContextFlags(BitFlags<ExpressionContextFlag>);

impl ExpressionContextFlags {
    /// Whether `in` should be counted in a binary expression.
    /// This is for `for...in` statements to prevent ambiguity.
    /// Corresponds to `[+In]` in the EcmaScript spec if true
    const INCLUDE_IN: Self = Self(make_bitflags!(ExpressionContextFlag::{IncludeIn}));

    /// If false, object expressions are not allowed to be parsed
    /// inside an expression.
    ///
    /// Also applies for object patterns
    const ALLOW_OBJECT_EXPRESSION: Self =
        Self(make_bitflags!(ExpressionContextFlag::{AllowObjectExpression}));

    /// If `true` then, don't parse computed member expressions because they can as well indicate
    /// the start of a computed class member.
    const IN_DECORATOR: Self = Self(make_bitflags!(ExpressionContextFlag::{InDecorator}));

    /// If `true` allows a typescript type assertion.
    /// Currently disabled on "new" expressions.
    const ALLOW_TS_TYPE_ASSERTION: Self =
        Self(make_bitflags!(ExpressionContextFlag::{AllowTSTypeAssertion}));

    pub fn contains(&self, other: impl Into<ExpressionContextFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl BitOr for ExpressionContextFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ExpressionContextFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for ExpressionContextFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Sub for ExpressionContextFlags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl ExpressionContext {
    pub(super) fn and_include_in(self, include: bool) -> Self {
        self.and(ExpressionContextFlags::INCLUDE_IN, include)
    }

    pub(super) fn and_object_expression_allowed(self, allowed: bool) -> Self {
        self.and(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION, allowed)
    }

    pub(super) fn and_in_decorator(self, in_decorator: bool) -> Self {
        self.and(ExpressionContextFlags::IN_DECORATOR, in_decorator)
    }

    pub(super) fn and_ts_type_assertion_allowed(self, allowed: bool) -> Self {
        self.and(ExpressionContextFlags::ALLOW_TS_TYPE_ASSERTION, allowed)
    }

    /// Returns true if object expressions or object patterns are valid in this context
    pub(super) fn is_object_expression_allowed(&self) -> bool {
        self.0
            .contains(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION)
    }

    /// Returns `true` if the expression parsing includes binary in expressions.
    pub(super) fn is_in_included(&self) -> bool {
        self.0.contains(ExpressionContextFlags::INCLUDE_IN)
    }

    /// Returns `true` if currently parsing a decorator expression `@<expr>`.
    pub(super) fn is_in_decorator(&self) -> bool {
        self.0.contains(ExpressionContextFlags::IN_DECORATOR)
    }

    /// Adds the `flag` if `set` is `true`, otherwise removes the `flag`
    fn and(self, flag: ExpressionContextFlags, set: bool) -> Self {
        ExpressionContext(if set { self.0 | flag } else { self.0 - flag })
    }
}

/// Sets the default flags for a context that parses a new root expression (for example, the condition of an if statement)
/// or sub-expression of another expression (the alternate branch of a condition expression).
impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext(
            ExpressionContextFlags::INCLUDE_IN
                | ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION
                | ExpressionContextFlags::ALLOW_TS_TYPE_ASSERTION,
        )
    }
}

/// Parses an expression or recovers to the point of where the next statement starts
pub(super) fn parse_expression_or_recover_to_next_statement(
    p: &mut MParser,
    assign: bool,
    context: ExpressionContext,
) -> RecoveryResult {
    let func = if assign {
        parse_assignment_expression_or_higher
    } else {
        parse_expression
    };

    func(p, context).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(
            MSyntaxKind::M_BOGUS_EXPRESSION,
            STMT_RECOVERY_SET.union(token_set![T!['}']]),
        )
        .enable_recovery_on_line_break(),
        expected_expression,
    )
}

/// A literal expression.
///
/// `TRUE | FALSE | NUMBER | STRING | NULL`
// test js literals
// 5
// true
// false
// 5n
// "foo"
// 'bar'
// null
// 0, 0.0, 0n, 0e00
// "test\
// new-line";
// /^[يفمئامئ‍ئاسۆند]/i; //regex with unicode
// /[\p{Control}--[\t\n]]/v;
// /\’/; // regex with escaped non-ascii chars (issue #1941)

// test_err js literals
// 00, 012, 08, 091, 0789 // parser errors
// 01n, 0_0, 01.2 // lexer errors
// "test
// continues" // unterminated string literal

// test_err js regex
// /[\p{Control}--[\t\n]]/vv;
// /[\p{Control}--[\t\n]]/uv;
pub(super) fn parse_literal_expression(p: &mut MParser) -> ParsedSyntax {
    let literal_kind = match p.cur() {
        MSyntaxKind::M_NUMBER_LITERAL => {
            return parse_number_literal_expression(p)
                .or_else(|| parse_big_int_literal_expression(p));
        }
        MSyntaxKind::M_STRING_LITERAL => MSyntaxKind::M_STRING_LITERAL_EXPRESSION,
        MSyntaxKind::NULL_KW => MSyntaxKind::M_NULL_LITERAL_EXPRESSION,
        MSyntaxKind::TRUE_KW | MSyntaxKind::FALSE_KW => MSyntaxKind::M_BOOLEAN_LITERAL_EXPRESSION,
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, literal_kind))
}

pub(super) fn parse_big_int_literal_expression(p: &mut MParser) -> ParsedSyntax {
    if !p.at(M_NUMBER_LITERAL) || !p.cur_text().ends_with('n') {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(MSyntaxKind::M_BIGINT_LITERAL);
    Present(m.complete(p, M_BIGINT_LITERAL_EXPRESSION))
}

pub(super) fn parse_number_literal_expression(p: &mut MParser) -> ParsedSyntax {
    let cur_src = p.cur_text();
    if !p.at(M_NUMBER_LITERAL) || cur_src.ends_with('n') {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, M_NUMBER_LITERAL_EXPRESSION))
}

/// Parses an assignment expression or any higher expression
/// https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-AssignmentExpression
pub(super) fn parse_assignment_expression_or_higher(
    p: &mut MParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    parse_assignment_expression_or_higher_base(p, context)
}

fn parse_assignment_expression_or_higher_base(
    p: &mut MParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    parse_conditional_expr(p, context)
        .and_then(|target| parse_assign_expr_recursive(p, target, checkpoint, context))
}

// test js assign_expr
// foo += bar = b ??= 3;
// foo -= bar;
// (foo = bar);
// [foo, bar] = baz;
// [foo, bar = "default", ...rest] = baz;
// [,,,foo,bar] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});

// test_err js assign_expr_right
// (foo = );

// test_err js assign_expr_left
// ( = foo);

// test js assign_eval_member_or_computed_expr
// eval.foo = 10
// arguments[1] = "baz"
// eval[2] = "Chungking Express"

// test_err js assign_eval_or_arguments
// eval = 0
// eval ??= 2
// eval *= 4
// arguments = "foo"
// arguments ||= "baz"
// ({ eval } = o)
// ({ foo: { eval }}) = o
fn parse_assign_expr_recursive(
    p: &mut MParser,
    mut target: CompletedMarker,
    checkpoint: MParserCheckpoint,
    context: ExpressionContext,
) -> ParsedSyntax {
    let assign_operator = p.cur();
    if is_assign_token(assign_operator) {
        let target = if matches!(target.kind(p), M_BINARY_EXPRESSION) {
            // Special handling for binary expressions and type assertions to avoid having to deal with `a as string = ...`
            // inside of the `ReparseAssignment` implementation because not using parentheses is valid
            // in for heads `for (a as any in []) {}`
            p.error(invalid_assignment_error(p, target.range(p)));
            target.change_kind(p, M_BOGUS_ASSIGNMENT);
            target
        } else {
            expression_to_assignment_pattern(p, target, checkpoint)
        };

        let m = target.precede(p);
        p.expect(assign_operator);

        parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true))
            .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);
        Present(m.complete(p, M_ASSIGNMENT_EXPRESSION))
    } else {
        Present(target)
    }
}

fn is_assign_token(kind: MSyntaxKind) -> bool {
    matches!(kind, T![=] | T![+=] | T![-=] | T![*=] | T![/=] | T![%=])
}

/// A conditional expression such as `foo ? bar : baz`
// test js conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz

pub(super) fn parse_conditional_expr(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    // test_err js conditional_expr_err
    // foo ? bar baz
    // foo ? bar baz ? foo : bar
    // foo ? bar :
    let lhs = parse_binary_or_logical_expression(p, OperatorPrecedence::lowest(), context);

    if p.at(T![?]) {
        lhs.map(|marker| {
            let m = marker.precede(p);
            p.bump(T![?]);

            parse_conditional_expr_consequent(p, ExpressionContext::default())
                .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);

            p.expect(T![:]);

            parse_assignment_expression_or_higher(p, context)
                .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);
            m.complete(p, M_CONDITIONAL_EXPRESSION)
        })
    } else {
        lhs
    }
}

/// Specialized version of [parse_assignment_expression_or_higher].
/// We need to make sure that on a successful arrow expression parse that
/// the next token is `:`.
// test js arrow_expr_in_alternate
// a ? (b) : a => {};

// test ts ts_arrow_exrp_in_alternate
// a ? (b) : a => {};

// test jsx jsx_arrow_exrp_in_alternate
// bar ? (foo) : (<a>{() => {}}</a>);
fn parse_conditional_expr_consequent(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();

    p.rewind(checkpoint);
    parse_assignment_expression_or_higher_base(p, context)
}

pub(super) fn is_at_binary_operator(p: &MParser, context: ExpressionContext) -> bool {
    let cur_kind = p.cur();

    match cur_kind {
        T![in] => context.is_in_included(),
        kind => OperatorPrecedence::try_from_binary_operator(kind).is_some(),
    }
}

/// A binary expression such as `2 + 2` or `foo * bar + 2` or a logical expression 'a || b'
fn parse_binary_or_logical_expression(
    p: &mut MParser,
    left_precedence: OperatorPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    // test js private_name_presence_check
    // class A {
    // 	#prop;
    // 	test() {
    //    #prop in this
    //  }
    // }
    let left = parse_unary_expr(p, context);

    parse_binary_or_logical_expression_recursive(p, left, left_precedence, context)
}

// test js binary_expressions
// 5 * 5
// 6 ** 6 ** 7
// 1 + 2 * 3
// (1 + 2) * 3
// 1 / 2
// 74 in foo
// foo instanceof Array
// foo ?? bar
// a >> b
// a >>> b
// 1 + 1 + 1 + 1
// 5 + 6 - 1 * 2 / 1 ** 6
// class Test { #name; test() { true && #name in {} } }

// test_err js binary_expressions_err
// foo(foo +);
// foo + * 2;
// !foo * bar;
fn parse_binary_or_logical_expression_recursive(
    p: &mut MParser,
    mut left: ParsedSyntax,
    left_precedence: OperatorPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    // Use a loop to eat all binary expressions with the same precedence.
    // At first, the algorithm makes the impression that it recurse for every right-hand side expression.
    // This is true, but `parse_binary_or_logical_expression` immediately returns if the
    // current operator has the same or a lower precedence than the left-hand side expression. Thus,
    // the algorithm goes at most `count(OperatorPrecedence)` levels deep.
    loop {
        // test_err js js_right_shift_comments
        // 1 >> /* a comment */ > 2;
        let op = p.re_lex(MReLexContext::BinaryOperator);

        let new_precedence = match OperatorPrecedence::try_from_binary_operator(op) {
            Some(precedence) => precedence,
            // Not a binary operator
            None => break,
        };

        let stop_at_current_operator = if new_precedence.is_right_to_left() {
            new_precedence < left_precedence
        } else {
            new_precedence <= left_precedence
        };

        if stop_at_current_operator {
            break;
        }

        let op_range = p.cur_range();

        let mut is_bogus = false;
        if let Present(left) = &mut left {
        } else {
            let err = p
                .err_builder(
                    format!(
                        "Expected an expression for the left hand side of the `{}` operator.",
                        p.text(op_range),
                    ),
                    op_range,
                )
                .with_hint("This operator requires a left hand side value");
            p.error(err);
        }

        let m = left.precede(p);
        p.bump(op);

        parse_binary_or_logical_expression(p, new_precedence, context)
            .or_add_diagnostic(p, expected_expression);

        let expression_kind = if is_bogus {
            M_BOGUS_EXPRESSION
        } else {
            match op {
                // test js logical_expressions
                // foo ?? bar
                // a || b
                // a && b
                //
                // test_err js logical_expressions_err
                // foo ?? * 2;
                // !foo && bar;
                // foo(foo ||)
                T![||] | T![&&] => M_LOGICAL_EXPRESSION,
                T![in] => M_IN_EXPRESSION,
                _ => M_BINARY_EXPRESSION,
            }
        };

        left = Present(m.complete(p, expression_kind));
    }

    left
}

/// A member or new expression with subscripts. e.g. `new foo`, `new Foo()`, `foo`, or `foo().bar[5]`
// test js new_exprs
// new Foo()
// new foo;
// new.target
// new new new new Foo();
// new Foo(bar, baz, 6 + 6, foo[bar] + ((foo) => {}) * foo?.bar)

// test_err js new_exprs
// new;
fn parse_member_expression_or_higher(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    parse_primary_expression(p, context)
        .map(|lhs| parse_member_expression_rest(p, lhs, context, true, &mut false))
}

// test_err js subscripts_err
// foo()?.baz[].;
// BAR`b
fn parse_member_expression_rest(
    p: &mut MParser,
    lhs: CompletedMarker,
    context: ExpressionContext,
    allow_optional_chain: bool,
    in_optional_chain: &mut bool,
) -> CompletedMarker {
    let mut progress = ParserProgress::default();
    let mut lhs = lhs;

    while !p.at(EOF) {
        progress.assert_progressing(p);
        lhs = match p.cur() {
            T![.] => parse_static_member_expression(p, lhs, T![.]).unwrap(),
            // Don't parse out `[` as a member expression because it may as well be the start of a computed class member
            T!['['] if !context.is_in_decorator() => {
                parse_computed_member_expression(p, lhs, false).unwrap()
            }
            _ => {
                break;
            }
        };
    }

    lhs
}

// test_err ts ts_new_operator
// new A<test><test>();

// test ts ts_new_operator
// var c2 = new T<string>;  // Ok
// var x1 = new SS<number>(); // OK
// var x3 = new SS();         // OK
// var x4 = new SS;           // OK
fn parse_new_expr(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![new]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![new]);

    parse_primary_expression(p, context)
        .or_add_diagnostic(p, expected_expression)
        .map(|expr| parse_member_expression_rest(p, expr, context, false, &mut false));

    // test ts ts_new_with_type_arguments
    // class Test<A, B, C> {}
    // new Test<A, B, C>();

    if p.at(T!['(']) {
        parse_call_arguments(p).unwrap();
    }

    Present(m.complete(p, M_NEW_EXPRESSION))
}

// test js super_expression
// class Test extends B {
//   constructor() {
//     super();
//   }
//   test() {
//     super.test(a, b);
//     super[1];
//   }
// }
//
// test_err js super_expression_err
// class Test extends B {
//   test() {
//     super();
//     super?.test();
//   }
// }
// super();
fn parse_super_expression(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![super]) {
        return Absent;
    }
    let super_marker = p.start();
    p.expect(T![super]);
    let mut super_expression = super_marker.complete(p, M_SUPER_EXPRESSION);

    if p.at(T!['(']) && !p.state().in_constructor() {
        p.error(p.err_builder(
            "`super` is only valid inside of a class constructor of a subclass.",
            super_expression.range(p),
        ));
        super_expression.change_kind(p, M_BOGUS_EXPRESSION);
    }

    match p.cur() {
        T![.] | T!['['] | T!['('] => Present(super_expression),
        _ => parse_static_member_expression(p, super_expression, T![.]),
    }
}

// test js subscripts
// foo`bar`
// foo(bar)(baz)(baz)[bar]

/// A static member expression for accessing a property
// test js static_member_expression
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
// class Test {
//   #bar
//   test(other) {
//     this.#bar;
//     this?.#bar;
//     other.#bar;
//     other?.#bar;
//   }
// }
fn parse_static_member_expression(
    p: &mut MParser,
    lhs: CompletedMarker,
    operator: MSyntaxKind,
) -> ParsedSyntax {
    let m = lhs.precede(p);
    p.expect(operator);

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, M_STATIC_MEMBER_EXPRESSION))
}

pub(super) fn parse_any_name(p: &mut MParser) -> ParsedSyntax {
    parse_name(p)
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test js computed_member_expression
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
fn parse_computed_member_expression(
    p: &mut MParser,
    lhs: CompletedMarker,
    optional_chain: bool,
) -> ParsedSyntax {
    // test_err js bracket_expr_err
    // foo[]
    // foo?.[]
    // foo[
    let m = lhs.precede(p);

    p.expect(T!['[']);
    // test js computed_member_in
    // for ({}["x" in {}];;) {}
    parse_expression(p, ExpressionContext::default()).or_add_diagnostic(p, expected_expression);

    p.expect(T![']']);

    Present(m.complete(p, M_COMPUTED_MEMBER_EXPRESSION))
}

/// An identifier name, either an ident or a keyword
pub(super) fn parse_name(p: &mut MParser) -> ParsedSyntax {
    if is_at_name(p) {
        let m = p.start();
        p.bump_remap(T![ident]);
        Present(m.complete(p, M_NAME))
    } else {
        Absent
    }
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`

// test js call_arguments
// function foo(...args) {}
// let a, b, c, d;
// foo(a);
// foo(a, b,);
// foo(a, b, ...c);
// foo(...a, ...b, c, ...d,);
//
// test_err js invalid_arg_list
// function foo(...args) {}
// let a, b, c;
// foo(a,b;
// foo(a,b var;
// foo (,,b);
// foo (a, ...);
fn parse_call_arguments(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    // test js in_expr_in_arguments
    // function foo() {}
    // for (foo("call" in foo);;) {}

    let m = p.start();
    p.bump(T!['(']);
    let args_list = p.start();
    let mut first = true;
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

        let argument = if p.at(T![...]) {
            // already do a check on "..." so it's safe to unwrap
            parse_spread_element(p, ExpressionContext::default())
        } else {
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
        };

        if argument.is_absent() && p.at(T![,]) {
            argument.or_add_diagnostic(p, m_parse_error::expected_expression);
            // missing element
            continue;
        }

        if argument
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    M_BOGUS_EXPRESSION,
                    EXPR_RECOVERY_SET.union(token_set!(T![')'], T![;], T![...])),
                )
                .enable_recovery_on_line_break(),
                m_parse_error::expected_expression,
            )
            .is_err()
        {
            break;
        }
    }

    args_list.complete(p, M_CALL_ARGUMENT_LIST);
    p.expect(T![')']);
    Present(m.complete(p, M_CALL_ARGUMENTS))
}

// test js parenthesized_sequence_expression
// (a, b);
// (a, b, c);
// (a, b, c, d, e, f);
// (a, b, c, d, e, f)
// (a, b, c)

// test_err js incomplete_parenthesized_sequence_expression
// (a,;
// (a, b, c;

// test js m_parenthesized_expression
// ((foo))
// (foo)

fn parse_parenthesized_expression(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    // test js for_with_in_in_parenthesized_expression
    // for((true,"selectionStart"in true);;) {}
    if p.at(T![')']) {
        // test_err js empty_parenthesized_expression
        // ();
        p.error(
            p.err_builder(
                "Parenthesized expression didnt contain anything",
                p.cur_range(),
            )
            .with_hint("Expected an expression here"),
        );
    } else {
        let first = parse_assignment_expression_or_higher(p, ExpressionContext::default());

        if p.at(T![,]) {
            parse_sequence_expression_recursive(p, first, ExpressionContext::default())
                .or_add_diagnostic(p, expected_expression);
        }
    }

    p.expect(T![')']);
    Present(m.complete(p, M_PARENTHESIZED_EXPRESSION))
}

/// A general expression.
pub(super) fn parse_expression(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    let first = parse_assignment_expression_or_higher(p, context);

    if p.at(T![,]) {
        parse_sequence_expression_recursive(p, first, context)
    } else {
        first
    }
}

// test js sequence_expr
// 1, 2, 3, 4, 5

// test_err js sequence_expr
// 1, 2, , 4
fn parse_sequence_expression_recursive(
    p: &mut MParser,
    left: ParsedSyntax,
    context: ExpressionContext,
) -> ParsedSyntax {
    if !p.at(T![,]) {
        return left;
    }

    let mut left = left;

    while p.at(T![,]) {
        let sequence_expr_marker =
            left.precede_or_add_diagnostic(p, m_parse_error::expected_expression);
        p.bump(T![,]);
        parse_assignment_expression_or_higher(p, context).or_add_diagnostic(p, expected_expression);

        left = Present(sequence_expr_marker.complete(p, M_SEQUENCE_EXPRESSION))
    }

    left
}

#[inline]
pub(super) fn is_at_expression(p: &mut MParser) -> bool {
    is_nth_at_expression(p, 0)
}

pub(super) fn is_nth_at_expression(p: &mut MParser, n: usize) -> bool {
    match p.nth(n) {
        T![!]
        | T!['(']
        | T!['[']
        | T!['{']
        | T![++]
        | T![--]
        | T![~]
        | T![+]
        | T![-]
        | T![throw]
        | T![new]
        | T![delete]
        | T![ident]
        | T![...]
        | T![this]
        | T![function]
        | T![class]
        | T![super]
        | T![<]
        | T![/]
        | T![/=]
        | TRUE_KW
        | FALSE_KW
        | M_NUMBER_LITERAL
        | M_BIGINT_LITERAL
        | M_STRING_LITERAL
        | NULL_KW => true,
        _ => false,
    }
}

/// A primary expression such as a literal, an object, an array, or `this`.
fn parse_primary_expression(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    let parsed_literal_expression = parse_literal_expression(p);
    if parsed_literal_expression.is_present() {
        return parsed_literal_expression;
    }

    let complete = match p.cur() {
        T![this] => {
            // test js this_expr
            // this
            // this.foo
            let m = p.start();
            p.expect(T![this]);
            m.complete(p, M_THIS_EXPRESSION)
        }
        T![function] => {
            // test js function_expr
            // let a = function() {}
            // let b = function foo() {}

            parse_function_expression(p).unwrap()
        }
        // test js grouping_expr
        // ((foo))
        // (foo)
        T!['('] => parse_parenthesized_expression(p).unwrap(),
        T![@] => match p.nth(1) {
            T!['['] => parse_array_expr(p).unwrap(),
            T!['{'] => parse_object_expression(p).unwrap(),
            T!['('] => parse_hashmap_expression(p).unwrap(),
            _ => {
                let m = p.start();
                p.bump(T![@]);
                m.complete(p, M_BOGUS)
            }
        },
        T!['['] => parse_array_expr(p).unwrap(),
        T!['{'] if context.is_object_expression_allowed() => parse_object_expression(p).unwrap(),

        // test_err js import_keyword_in_expression_position
        // let a = import;
        T![new] => parse_new_expr(p, context).unwrap(),

        ERROR_TOKEN => {
            let m = p.start();
            p.bump_any();
            m.complete(p, M_BOGUS)
        }
        T![ident] => parse_identifier_expression(p).unwrap(),

        _ => {
            return Absent;
        }
    };

    Present(complete)
}

fn parse_identifier_expression(p: &mut MParser) -> ParsedSyntax {
    parse_reference_identifier(p)
        .map(|identifier| identifier.precede(p).complete(p, M_IDENTIFIER_EXPRESSION))
}

// test_err js identifier
// yield;
// await;
pub(super) fn parse_reference_identifier(p: &mut MParser) -> ParsedSyntax {
    parse_identifier(p, M_REFERENCE_IDENTIFIER)
}

pub(super) fn is_nth_at_reference_identifier(p: &mut MParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
}

// test js identifier_loose_mode
// // SCRIPT
// foo;
// yield;
// await;
//
// test js identifier
// foo;
// let accessor = 5;
//
// test_err js identifier_err
// yield;
// await;
// async function test(await) {}
// function* test(yield) {}
// enum;
// implements;
// interface;

/// Parses an identifier if it is valid in this context or returns `Invalid` if the context isn't valid in this context.
/// An identifier is invalid if:
/// * It is named `await` inside of an async function
/// * It is named `yield` inside of a generator function or in strict mode
pub(super) fn parse_identifier(p: &mut MParser, kind: MSyntaxKind) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![ident]);
    let mut identifier = m.complete(p, kind);

    Present(identifier)
}

#[inline]
pub(super) fn is_at_identifier(p: &mut MParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(super) fn is_nth_at_identifier(p: &mut MParser, n: usize) -> bool {
    p.nth_at(n, T![ident])
}

#[inline]
pub(super) fn is_nth_at_identifier_or_keyword(p: &mut MParser, n: usize) -> bool {
    p.nth(n).is_keyword() || is_nth_at_identifier(p, n)
}

struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    type Kind = MSyntaxKind;
    type Parser<'a> = MParser<'a>;
    const LIST_KIND: MSyntaxKind = M_ARRAY_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        match p.cur() {
            T![...] => parse_spread_element(p, ExpressionContext::default()),
            T![,] => Present(p.start().complete(p, M_ARRAY_HOLE)),
            _ => parse_assignment_expression_or_higher(p, ExpressionContext::default()),
        }
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                M_BOGUS_EXPRESSION,
                EXPR_RECOVERY_SET.union(token_set!(T![']'])),
            ),
            m_parse_error::expected_array_element,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An array literal such as `[foo, bar, ...baz]`.
// test js array_expr
// @[foo, bar];
// @[foo];
// @[,foo];
// @[foo,];
// @[,,,,,foo,,,,];
// @[...a, ...b];

// test_err js array_expr_incomplete
// let a = [
fn parse_array_expr(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![@]) && !p.nth_at(1, T!['[']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);
    p.bump(T!['[']);

    // test js array_element_in_expr
    // for(["a" in {}];;) {}
    ArrayElementsList.parse_list(p);

    p.expect(T![']']);
    Present(m.complete(p, M_ARRAY_EXPRESSION))
}

// test_err js spread
// [...]
/// A spread element consisting of three dots and an assignment expression such as `...foo`
pub(super) fn parse_spread_element(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![...]);
    parse_assignment_expression_or_higher(p, context)
        .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);
    Present(m.complete(p, M_SPREAD))
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub(super) fn parse_lhs_expr(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    // super.foo and super[bar]
    // test js super_property_access
    // super.foo
    // super[bar]
    // super[foo][bar]
    let lhs = if p.at(T![super]) {
        parse_super_expression(p)
    } else {
        parse_member_expression_or_higher(p, context)
    };

    lhs.map(|lhs_marker| parse_call_expression_rest(p, lhs_marker, context))
}

fn parse_call_expression_rest(
    p: &mut MParser,
    lhs: CompletedMarker,
    context: ExpressionContext,
) -> CompletedMarker {
    let mut lhs = lhs;
    let mut in_optional_chain = false;
    loop {
        lhs = parse_member_expression_rest(p, lhs, context, true, &mut in_optional_chain);

        if !matches!(p.cur(), T![<] | T!['(']) {
            break lhs;
        }

        // Cloning here is necessary because parsing out the type arguments may rewind in which
        // case we want to return the `lhs`.
        let m = lhs.clone().precede(p);

        let start_pos = p.source().position();

        // test ts ts_call_expr_with_type_arguments
        // function a<A, B, C>() {}
        // a<A, B, C>();
        // (() => { a }).a<A, B, C>()
        // (() => a)<A, B, C>();
        // type A<T> = T;
        // a<<T>(arg: T) => number, number, string>();

        if p.at(T!['(']) {
            parse_call_arguments(p)
                .or_add_diagnostic(p, |p, _| expected_token(T!['(']).into_diagnostic(p));
            lhs = m.complete(p, M_CALL_EXPRESSION);
        } else {
            break {
                // test ts optional_chain_call_less_than
                // String(item)?.b < 0;
                // String(item)?.b <aBcd;

                // Safety:
                // * The method initially checks if the parsers at a '<', '(', or '?.' token.
                // * if the parser is at '?.': It takes the branch right above, ensuring that no token was consumed
                // * if the parser is at '<': `parse_ts_type_arguments_in_expression` rewinds if what follows aren't  valid type arguments and this is the only way we can reach this branch
                // * if the parser is at '(': This always parses out as valid arguments.
                debug_assert_eq!(p.source().position(), start_pos);
                m.abandon(p);
                lhs
            };
        }
    }
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test js postfix_expr
// foo++
// foo--
fn parse_postfix_expr(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let lhs = parse_lhs_expr(p, context);
    lhs.map(|marker| {
        if !p.has_preceding_line_break() {
            // test js post_update_expr
            // foo++
            // foo--
            match p.cur() {
                T![++] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![++]);
                    m.complete(p, M_POST_UPDATE_EXPRESSION)
                }
                T![--] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![--]);
                    m.complete(p, M_POST_UPDATE_EXPRESSION)
                }
                _ => marker,
            }
        } else {
            marker
        }
    })
}

/// A unary expression such as `!foo` or `++bar`
pub(super) fn parse_unary_expr(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    const UNARY_SINGLE: TokenSet<MSyntaxKind> = token_set![T![delete], T![+], T![-], T![!]];

    // test js pre_update_expr
    // ++foo
    // --foo
    if p.at(T![++]) {
        let m = p.start();
        p.bump(T![++]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, M_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }
    if p.at(T![--]) {
        let m = p.start();
        p.bump(T![--]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, M_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }

    // test js m_unary_expressions
    // delete a['test'];
    // void a;
    // typeof a;
    // +1;
    // -1;
    // ~1;
    // !true;
    // -a + -b + +a;

    // test_err js unary_expr
    // ++ ;
    // -- ;
    // -;

    if p.at_ts(UNARY_SINGLE) {
        let m = p.start();
        let op = p.cur();

        let is_delete = op == T![delete];

        if is_delete {
            p.expect(T![delete]);
        } else {
            p.bump_any();
        }

        // test js unary_delete
        // delete obj.key;
        // delete (obj).key;
        // delete obj.#member.key;
        // delete (obj.#member).key;
        // delete func().#member.key;
        // delete (func().#member).key;
        // delete obj?.#member.key;
        // delete (obj?.#member).key;
        // delete obj?.inner.#member.key;
        // delete (obj?.inner.#member).key;
        // delete obj[key];
        // delete (obj)[key];
        // delete obj.#member[key];
        // delete (obj.#member)[key];
        // delete func().#member[key];
        // delete (func().#member)[key];
        // delete obj?.#member[key];
        // delete (obj?.#member)[key];
        // delete obj?.inner.#member[key];
        // delete (obj?.inner.#member)[key];
        // delete (obj.#key, obj.key);
        // delete (#key in obj);
        // delete (obj.key);
        // delete (console.log(1));
        // delete (() => {});

        // test js unary_delete_nested
        // class TestClass { #member = true; method() { delete func(this.#member) } }
        // class TestClass { #member = true; method() { delete [this.#member] } }
        // class TestClass { #member = true; method() { delete { key: this.#member } } }
        // class TestClass { #member = true; method() { delete (() => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (param => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (async () => { this.#member; }) } }

        // test_err js unary_delete
        // delete ident;
        // delete obj.#member;
        // delete func().#member;
        // delete obj?.#member;
        // delete obj?.inner.#member;

        // test_err js unary_delete_parenthesized
        // delete (ident);
        // delete ((ident));
        // delete (obj.key, ident);
        // delete (obj.#member);
        // delete (func().#member);
        // delete (obj?.#member);
        // delete (obj?.inner.#member);
        // delete (obj.key, obj.#key);

        let kind = M_UNARY_EXPRESSION;

        if is_delete {
            let checkpoint = p.checkpoint();
            parse_unary_expr(p, context).ok();

            let mut rewriter = DeleteExpressionRewriter::default();
            rewrite_events(&mut rewriter, checkpoint, p);

            rewriter.result.take()
        } else {
            parse_unary_expr(p, context).ok()
        };

        return Present(m.complete(p, kind));
    }

    parse_postfix_expr(p, context)
}

#[derive(Default)]
struct DeleteExpressionRewriter {
    stack: Vec<(RewriteMarker, MSyntaxKind)>,
    result: Option<CompletedMarker>,
    /// Set to true immediately after the rewriter exits an identifier expression
    exited_ident_expr: Option<TextRange>,
    /// Set to true immediately after the rewriter exits a private name
    exited_private_name: bool,
    /// Set to true immediately after the rewriter exits a member expresison with a private name
    exited_private_member_expr: Option<TextRange>,
}

impl RewriteParseEvents for DeleteExpressionRewriter {
    fn start_node(&mut self, kind: MSyntaxKind, p: &mut RewriteParser) {
        self.stack.push((p.start(), kind));
        self.exited_ident_expr.take();
        self.exited_private_name = false;
        self.exited_private_member_expr.take();
    }

    fn finish_node(&mut self, p: &mut RewriteParser) {
        let (m, kind) = self.stack.pop().expect("stack depth mismatch");
        let node = m.complete(p, kind);

        if kind != M_PARENTHESIZED_EXPRESSION && kind != M_SEQUENCE_EXPRESSION {
            self.exited_private_member_expr =
                if self.exited_private_name && kind == M_STATIC_MEMBER_EXPRESSION {
                    Some(node.range(p))
                } else {
                    None
                };

            self.exited_ident_expr = if kind == M_IDENTIFIER_EXPRESSION {
                Some(node.range(p))
            } else {
                None
            };

            self.exited_private_name = kind == M_PRIVATE_NAME;
        }

        self.result = Some(node.into());
    }
}

pub(super) fn is_at_name(p: &mut MParser) -> bool {
    is_nth_at_name(p, 0)
}

pub(super) fn is_nth_at_name(p: &mut MParser, offset: usize) -> bool {
    p.nth_at(offset, T![ident]) || p.nth(offset).is_keyword()
}
