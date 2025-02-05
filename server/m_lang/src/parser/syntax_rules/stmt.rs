//! Statements, these include `if`, `while`, `for`, `;`, and more.
use crate::parser::syntax_rules::expr::parse_assignment_expression_or_higher;

use super::binding::*;

use super::expr::{parse_expression, parse_identifier_expression};

use super::assignment::expression_to_assignment_pattern;
use super::class::{parse_class_declaration, parse_initializer_clause};
use super::expr::{
    is_at_expression, parse_expression_or_recover_to_next_statement, ExpressionContext,
};
use super::function::parse_function_declaration;
use super::m_parse_error;
use super::m_parse_error::{expected_binding, expected_statement};
use super::state::{BreakableKind, EnterBreakable};

use super::syntax::{MSyntaxKind::*, T, *};
use super::{Absent, MParser, Present};

use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::*;
use biome_parser::ParserProgress;

pub const STMT_RECOVERY_SET: TokenSet<MSyntaxKind> = token_set![
    L_CURLY,
    VAR_KW,
    FUNCTION_KW,
    IF_KW,
    FOR_KW,
    DO_KW,
    WHILE_KW,
    CONTINUE_KW,
    BREAK_KW,
    RETURN_KW,
    SWITCH_KW,
    THROW_KW,
    TRY_KW,
    DEBUG_KW,
    FUNCTION_KW,
    CLASS_KW,
    T![;]
];

/// Consume an explicit semicolon, or try to automatically insert one,
/// or add an error to the parser if there was none and it could not be inserted
// test js semicolons
// let foo = bar;
// let foo2 = b;
// let foo3;
// let foo4
// let foo5
// function foo6() { return true }
pub(crate) fn semi(p: &mut MParser, err_range: TextRange) -> bool {
    // test_err js semicolons_err
    // let foo = bar throw foo

    if !optional_semi(p) {
        let err = p
            .err_builder(
                "Expected a semicolon or an implicit semicolon after a statement, but found none",
                p.cur_range(),
            )
            .with_detail(
                p.cur_range(),
                "An explicit or implicit semicolon is expected here...",
            )
            .with_detail(err_range, "...Which is required to end this statement");

        p.error(err);
        false
    } else {
        true
    }
}

/// Eats a semicolon if present but doesn't add an error none is present and the automatic
/// semicolon insertion rule does not apply.
///
/// Returns false if neither a semicolon was present and the current position doesn't allow an automatic
/// semicolon insertion.
pub(crate) fn optional_semi(p: &mut MParser) -> bool {
    if p.eat(T![;]) {
        return true;
    }

    is_semi(p, 0)
}

pub(crate) fn is_semi(p: &mut MParser, offset: usize) -> bool {
    p.nth_at(offset, T![;])
        || p.nth_at(offset, EOF)
        || p.nth_at(offset, T!['}'])
        || p.has_nth_preceding_line_break(offset)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StatementContext {
    If,
    While,
    With,
    For,
    // Block, Switch consequence, etc.
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(&self) -> bool {
        !matches!(self, StatementContext::StatementList)
    }

    pub(crate) fn is_statement_list(&self) -> bool {
        matches!(self, StatementContext::StatementList)
    }
}

/// A generic statement such as a block, if, while, with, etc
///
/// Error handling and recovering happens inside this function, so the
/// caller has to pass a recovery set.
///
/// If not passed, [STMT_RECOVERY_SET] will be used as recovery set
pub(crate) fn parse_statement(p: &mut MParser, context: StatementContext) -> ParsedSyntax {
    match p.cur() {
        T![;] => parse_empty_statement(p),
        T!['{'] => parse_block_stmt(p),
        T![if] => parse_if_statement(p),
        T![while] => parse_while_statement(p),

        T![var] => parse_variable_statement(p, context),
        T![for] => parse_for_statement(p),
        T![forall] => parse_forall_statement(p),

        T![switch] => parse_switch_statement(p),
        T![try] => parse_try_statement(p),
        T![return] => parse_return_statement(p),
        T![break] => parse_break_statement(p),
        T![continue] => parse_continue_statement(p),
        T![throw] => parse_throw_statement(p),
        T![debug] => parse_debugger_statement(p),
        // function
        T![function] => parse_function_declaration(p, context),

        // class
        T![class] => parse_class_declaration(p, context),

        _ if is_at_expression(p) => parse_expression_statement(p),
        _ => Absent,
    }
}

// test js ts_keyword_assignments
// declare = 1;
// abstract = 2;
// namespace = 3;
// type = 4;
// module = 5;
// global = 6;
//
// test js ts_keywords_assignments_script
// // SCRIPT
// interface = 1;
// private = 2;
// protected = 3;
// public = 4;
// implements = 5;
fn parse_expression_statement(p: &mut MParser) -> ParsedSyntax {
    let start = p.cur_range().start();

    let expr =
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default());

    if let Ok(expr) = expr {
        let m = expr.precede(p);
        semi(p, TextRange::new(start, p.cur_range().end()));
        Present(m.complete(p, M_EXPRESSION_STATEMENT))
    } else {
        Absent
    }
}

// test js debugger_stmt
// debugger;

// test_err js debugger_stmt
// function foo() {
//   debugger {
//     var something = "lorem";
//   }
// }

/// A debugger statement such as `debugger;`
fn parse_debugger_statement(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![debug]) {
        return Absent;
    }
    let m = p.start();
    let range = p.cur_range();
    p.expect(T![debug]); // debug keyword
    semi(p, range);
    Present(m.complete(p, M_DEBUG_STATEMENT))
}

/// A throw statement such as `throw new Error("uh oh");`
// test js throw_stmt
// throw new Error("foo");
// throw "foo"
fn parse_throw_statement(p: &mut MParser) -> ParsedSyntax {
    // test_err js throw_stmt_err
    // throw
    // new Error("oh no :(")
    // throw;
    if !p.at(T![throw]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range().start();
    p.expect(T![throw]); // throw keyword
    if p.has_preceding_line_break() {
        let mut err = p
            .err_builder(
                "Linebreaks between a throw statement and the error to be thrown are not allowed",
                p.cur_range(),
            )
            .with_hint("A linebreak is not allowed here");

        if is_at_expression(p) {
            err = err.with_detail(p.cur_range(), "Help: did you mean to throw this?");
        }

        p.error(err);
    } else {
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default()).ok();
    }

    semi(p, TextRange::new(start, p.cur_range().end()));
    Present(m.complete(p, M_THROW_STATEMENT))
}

// test js break_stmt
// while (true) {
//   break;
//   foo: {
//    break foo;
//   }
// }
// out: while (true) {
//   break out;
// }

// test_err js break_stmt
// function foo() { break; }
// while (true) {
//   break foo;
// }

/// A break statement with an optional label such as `break a;`
fn parse_break_statement(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![break]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range();
    p.expect(T![break]); // break keyword

    let error = if !p.state().break_allowed() {
        Some(p.err_builder("A `break` statement can only be used within an enclosing iteration or switch statement.", start, ))
    } else {
        None
    };

    semi(p, TextRange::new(start.start(), p.cur_range().end()));

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, M_BOGUS_STATEMENT))
    } else {
        Present(m.complete(p, M_BREAK_STATEMENT))
    }
}

// test js continue_stmt
// outer: while(true) {
// while (true) {
//   continue;
//     continue outer;
//    }
//   continue
// }

// test_err js continue_stmt
// function foo() { continue; }
// while (true) {
//   continue foo;
// }
// foo: {
//   continue foo;
// }
/// A continue statement with an optional label such as `continue a;`
fn parse_continue_statement(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![continue]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range();
    p.expect(T![continue]); // continue keyword

    // test js async_continue_stmt
    // async: for(a of b) continue async;
    let error = if !p.state().continue_allowed() {
        Some(p.err_builder( "A `continue` statement can only be used within an enclosing `for`, `while` or 'forall' statement.",start ))
    } else {
        None
    };

    semi(p, TextRange::new(start.start(), p.cur_range().end()));

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, M_BOGUS_STATEMENT))
    } else {
        Present(m.complete(p, M_CONTINUE_STATEMENT))
    }
}

// test js return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
/// A return statement with an optional value such as `return a;`
fn parse_return_statement(p: &mut MParser) -> ParsedSyntax {
    // test_err js return_stmt_err
    // return;
    // return foo;
    if !p.at(T![return]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range().start();
    p.expect(T![return]);
    if !p.has_preceding_line_break() {
        parse_expression(p, ExpressionContext::default()).ok();
    }

    semi(p, TextRange::new(start, p.cur_range().end()));
    let mut complete = m.complete(p, M_RETURN_STATEMENT);

    // The frontmatter of Astro files is executed inside a function during the compilation, so it's safe to have illegal returns
    if !p.state().in_function() {
        let err = p.err_builder(
            "Illegal return statement outside of a function",
            complete.range(p),
        );

        p.error(err);
        complete.change_kind(p, M_BOGUS_STATEMENT);
    }
    Present(complete)
}

// test js empty_stmt
// ;
/// An empty statement denoted by a single semicolon.
fn parse_empty_statement(p: &mut MParser) -> ParsedSyntax {
    if p.at(T![;]) {
        let m = p.start();
        p.bump_any(); // bump ;
        m.complete(p, M_EMPTY_STATEMENT).into()
    } else {
        Absent
    }
}

// test js block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
/// A block statement consisting of statements wrapped in curly brackets.
pub(crate) fn parse_block_stmt(p: &mut MParser) -> ParsedSyntax {
    parse_block_impl(p, M_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn parse_block_impl(p: &mut MParser, block_kind: MSyntaxKind) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    if block_kind == M_FUNCTION_BODY {
        let directives_list = p.start();
        directives_list.complete(p, M_DIRECTIVE_LIST);
    }

    let statement_list = p.start();

    parse_statements(p, true, statement_list);

    p.expect(T!['}']);

    Present(m.complete(p, block_kind))
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn parse_statements(p: &mut MParser, stop_on_r_curly: bool, statement_list: Marker) {
    let mut progress = ParserProgress::default();

    // test_err js statements_closing_curly
    // {
    // "name": "troublesome-lib",
    // "typings": "lib/index.d.ts",
    // "version": "0.0.1"
    // }
    let recovery_set = if stop_on_r_curly {
        // Don't eat over the closing '}'
        STMT_RECOVERY_SET.union(token_set![T!['}']])
    } else {
        STMT_RECOVERY_SET
    };

    while !p.at(EOF) {
        progress.assert_progressing(p);
        if stop_on_r_curly && p.at(T!['}']) {
            break;
        }

        if parse_statement(p, StatementContext::StatementList)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(M_BOGUS_STATEMENT, recovery_set),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    statement_list.complete(p, M_STATEMENT_LIST);
}

/// An expression wrapped in parentheses such as `()`
/// Returns `true` if the closing parentheses is present
fn parenthesized_expression(p: &mut MParser) -> bool {
    let has_l_paren = p.expect(T!['(']);

    parse_expression(
        p,
        ExpressionContext::default().and_object_expression_allowed(has_l_paren),
    )
    .or_add_diagnostic(p, m_parse_error::expected_expression);

    p.expect(T![')'])
}

/// An if statement such as `if (foo) { bar(); }`
// test js if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
fn parse_if_statement(p: &mut MParser) -> ParsedSyntax {
    // test_err js if_stmt_err
    // if (true) else {}
    // if (true) else
    // if else {}
    // if () {} else {}
    // if (true)}}}} {}
    if !p.at(T![if]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![if]);

    // (test)
    parenthesized_expression(p);

    // body
    parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);

    // else clause
    if p.at(T![else]) {
        let else_clause = p.start();
        p.expect(T![else]);
        parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);
        else_clause.complete(p, M_ELSE_CLAUSE);
    }

    Present(m.complete(p, M_IF_STATEMENT))
}

/// A while statement such as `while(true) { do_something() }`
// test js while_stmt
// while (true) {}
// while (5) {}
fn parse_while_statement(p: &mut MParser) -> ParsedSyntax {
    // test_err js while_stmt_err
    // while true {}
    // while {}
    // while (true {}
    // while true) }
    if !p.at(T![while]) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![while]);
    parenthesized_expression(p);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::While)
    })
    .or_add_diagnostic(p, expected_statement);

    Present(m.complete(p, M_WHILE_STATEMENT))
}

pub(crate) fn is_nth_at_variable_declarations(p: &mut MParser, n: usize) -> bool {
    match p.nth(n) {
        T![var] => true,
        _ => false,
    }
}

/// A var, const, using or let declaration statement such as `var a = 5, b;` or `let {a, b} = foo;`
// test js var_decl
// var a = 5;
// let { foo, bar } = 5;
// let bar2, foo2;
// const b = 5;
// const { foo5: [bar11], baz6 } = {};
// let foo6 = "lorem", bar7 = "ipsum", third8 = "value", fourth = 6;
// var q, w, e, r, t;
//
// test js using_declaration_statement
// using a = b;
// using c = d, e = _;
// using [g] = h;
// using [j]
// = k;
// await using l = m;
// await
// using p = q;
// await using[r];
// await using ([s] = t);
// await (using [u] = v);
// using w = {};
// using x = null;
// using y = undefined;
// using z = (foo, bar);
//
// test_err js variable_declaration_statement_err
// let a, { b } = { a: 10 }
// const c = 1, { d } = { a: 10 }
// const e;
// let [f];
// const { g };
//
// test_err js using_declaration_statement_err
// using a;
// using {b};
// using c = d, e;
// export using m = n;
// await using f;
// await using g = h, j;
// await using [o] = p;
// export await using q = r;
// await let s;
// await const t = 1;
pub(crate) fn parse_variable_statement(p: &mut MParser, context: StatementContext) -> ParsedSyntax {
    // test_err js var_decl_err
    // var a =;
    // const b = 5 let c = 5;
    let start = p.cur_range().start();
    let is_var = p.at(T![var]);

    parse_variable_declaration(p, VariableDeclarationParent::VariableStatement).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, TextRange::new(start, p.cur_range().start()));

        let mut statement = m.complete(p, M_VARIABLE_STATEMENT);

        // if !is_var && context.is_single_statement() {
        //     // test js hoisted_declaration_in_single_statement_context
        //     // if (true) var a;
        //     //
        //     // test_err js lexical_declaration_in_single_statement_context
        //     // if (true) let a;
        //     // while (true) const b = 5;
        //     p.error(
        //         p.err_builder(
        //             "Lexical declaration cannot appear in a single-statement context",
        //             statement.range(p),
        //         )
        //         .with_hint("Wrap this declaration in a block statement"),
        //     );
        //     statement.change_to_bogus(p);
        // }

        statement
    })
}

pub(super) fn parse_variable_declaration(
    p: &mut MParser,
    declaration_context: VariableDeclarationParent,
) -> ParsedSyntax {
    let m = p.start();
    if eat_variable_declaration(p, declaration_context).is_some() {
        Present(m.complete(p, M_VARIABLE_DECLARATION))
    } else {
        m.abandon(p);
        Absent
    }
}

/// What's the parent node of the variable declaration
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub(super) enum VariableDeclarationParent {
    /// Declaration inside a `for...of` or `for...in` or `for (;;)` loop
    For,

    /// Declaration as part of a variable statement (`let a`, `const b`, or `var c`).
    VariableStatement,

    /// Declaration as part of another statement, like `export let ...` or `declare let a`
    Clause,
}

/// Parses and consume variable declarations like `var`/`let`/`const`/`using`/`await using`.
/// Returns a tuple where
/// * the first element is the marker to the not yet completed list
/// * the second element is the range of all variable declarations except the first one. Is [None] if
///   there's only one declaration.
fn eat_variable_declaration(
    p: &mut MParser,
    declaration_parent: VariableDeclarationParent,
) -> Option<(CompletedMarker, Option<TextRange>)> {
    let mut context = VariableDeclaratorContext::new(declaration_parent);

    match p.cur() {
        T![var] => {
            p.bump(T![var]);
        }
        _ => {
            return None;
        }
    }

    let mut variable_declarator_list = VariableDeclaratorList {
        declarator_context: context,
        remaining_declarator_range: None,
    };
    let list = variable_declarator_list.parse_list(p);

    Some((list, variable_declarator_list.remaining_declarator_range))
}

struct VariableDeclaratorList {
    declarator_context: VariableDeclaratorContext,
    // Range of the declarators succeeding the first declarator
    // None until this hits the second declarator
    remaining_declarator_range: Option<TextRange>,
}

// test_err js variable_declarator_list_incomplete
// const a = 1,
//
// test_err js variable_declarator_list_empty
// const;
// const
impl ParseSeparatedList for VariableDeclaratorList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_VARIABLE_DECLARATOR_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_variable_declarator(p, &self.declarator_context).map(|declarator| {
            if self.declarator_context.is_first {
                self.declarator_context.is_first = false;
            } else if let Some(range) = self.remaining_declarator_range.as_mut() {
                *range = TextRange::new(range.start(), declarator.range(p).end());
            } else {
                self.remaining_declarator_range = Some(declarator.range(p));
            }
            declarator
        })
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        if self.declarator_context.is_first {
            false
        } else {
            !p.at(T![,])
        }
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS, STMT_RECOVERY_SET.union(token_set!(T![,])))
                .enable_recovery_on_line_break(),
            expected_binding,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn finish_list(&mut self, p: &mut MParser, m: Marker) -> CompletedMarker {
        if self.declarator_context.is_first {
            let m = m.complete(p, M_BOGUS);
            let range = m.range(p);
            p.error(expected_binding(p, range));
            m
        } else {
            m.complete(p, Self::LIST_KIND)
        }
    }
}

struct VariableDeclaratorContext {
    /// What kind of variable declaration is this (`var`, `let`, `const`, 'using')
    kind_name: Option<&'static str>,
    /// Is this the first declaration in the declaration list (a first, b second in `let a, b`)
    is_first: bool,
    /// What's the parent of the variable declaration
    parent: VariableDeclarationParent,
}

impl VariableDeclaratorContext {
    fn new(parent: VariableDeclarationParent) -> Self {
        Self {
            parent,
            kind_name: None,
            is_first: true,
        }
    }

    fn is_var(&self) -> bool {
        self.kind_name.is_none()
    }
}

// test js scoped_declarations
// let a = {
//   test() {
//     let a = "inner";
//   }
// };
//
// test js using_declarations_inside_for_statement
// for (using x of y) {};
// for await (using x of y) {};
// for (await using x of y) {};
// for await (await using x of y) {};
//
// test_err js invalid_using_declarations_inside_for_statement
// for (await using of x) {};
// for await (await using of x) {};
//
// A single declarator, either `ident` or `ident = assign_expr`
fn parse_variable_declarator(p: &mut MParser, context: &VariableDeclaratorContext) -> ParsedSyntax {
    p.state_mut().duplicate_binding_parent = context.kind_name;
    let id = parse_binding_pattern(p, ExpressionContext::default());
    p.state_mut().duplicate_binding_parent = None;

    id.map(|id| {
        let id_kind = id.kind(p);
        let id_range = id.range(p);
        let m = id.precede(p);

        let duplicate_binding_parent = p.state_mut().duplicate_binding_parent.take();

        let mut initializer = parse_initializer_clause(
            p,
            ExpressionContext::default()
                .and_include_in(context.parent != VariableDeclarationParent::For),
        )
        .ok();

        p.state_mut().duplicate_binding_parent = duplicate_binding_parent;

        // Heuristic to determine if we're in a for of or for in loop. This may be off if
        // the user uses a for of/in with multiple declarations but this isn't allowed anyway.
        let is_in_for_loop = context.parent == VariableDeclarationParent::For && context.is_first;
        let is_in_for_in = is_in_for_loop && p.at(T![in]);

        if is_in_for_in {
            if let Some(initializer) = initializer {
                // Initializers are disallowed for `for..in` and `for..of`,
                // except for `for(var ... in ...)` in loose mode

                // test js for_in_initializer_loose_mode
                // // SCRIPT
                // for (var i = 0 in []) {}

                // test_err js for_in_and_of_initializer_loose_mode
                // // SCRIPT
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}

                // test_err js for_in_and_of_initializer_strict_mode
                // for (var i = 0 in []) {}
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}
                // for (using x = y of z) {};
                // for await (using x = y of z) {};
                // for (await using x = y of z) {};
                // for await (await using x = y of z) {};

                if !is_in_for_in || !context.is_var() {
                    let err = p.err_builder(
                        if is_in_for_in {
                            "`for..in` statement declarators cannot have an initializer expression"
                        } else {
                            "`for..of` statement declarators cannot have an initializer expression"
                        },
                        initializer.range(p),
                    );

                    p.error(err);
                }
            }
        }

        m.complete(p, M_VARIABLE_DECLARATOR)
    })
}

/// Parses the header of a for statement into the current node and returns whatever it is a for in/of or "regular" for statement
fn parse_for_head(p: &mut MParser, has_l_paren: bool) -> MSyntaxKind {
    // for (;...
    if p.at(T![;]) {
        parse_normal_for_head(p);
        return M_FOR_STATEMENT;
    }

    // `for (let...` | `for (const...` | `for (var...`

    if is_nth_at_variable_declarations(p, 0) {
        let m = p.start();

        if eat_variable_declaration(p, VariableDeclarationParent::For).is_none() {
            return M_BOGUS;
        }

        m.complete(p, M_VARIABLE_DECLARATION);
        parse_normal_for_head(p);
        M_FOR_STATEMENT
    } else {
        // for (some_expression`
        let init_expr = parse_expression(
            p,
            ExpressionContext::default()
                .and_include_in(false)
                .and_object_expression_allowed(has_l_paren),
        );

        init_expr.or_add_diagnostic(p, m_parse_error::expected_expression);

        parse_normal_for_head(p);
        M_FOR_STATEMENT
    }
}

/// Parses the parenthesized part of a non for in or for of statement
/// Expects to be positioned right after the initializer
fn parse_normal_for_head(p: &mut MParser) {
    p.expect(T![;]);

    if !p.at(T![;]) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, m_parse_error::expected_expression);
    }

    p.expect(T![;]);

    if !p.at(T![')']) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, m_parse_error::expected_expression);
    }
}

/// Parses the header of a forall statement into the current node
fn parse_forall_head(p: &mut MParser, has_l_paren: bool) -> MSyntaxKind {
    // `forall (var x in ...)` | `forall (factory(obj, index)) `

    if is_nth_at_variable_declarations(p, 0) {
        let m = p.start();

        let (declarations, additional_declarations) =
            eat_variable_declaration(p, VariableDeclarationParent::For).unwrap();

        let is_in = p.at(T![in]);

        if is_in {
            // remove the intermediate list node created by parse variable declarations that is not needed
            // for a ForInOrOfInitializer where the variable declaration is a direct child.
            declarations.undo_completion(p).abandon(p);

            if let Some(additional_declarations_range) = additional_declarations {
                p.error(
                    p.err_builder(
                        format!(
                            "Only a single declaration is allowed in a `for...{}` statement.",
                            "in",
                        ),
                        additional_declarations_range,
                    )
                    .with_hint("additional declarations"),
                );
            }

            m.complete(p, M_FOR_VARIABLE_DECLARATION);

            parse_forall_in_head(p)
        } else {
            p.error(p.err_builder(
                format!("Expected in keyword in `for...{}` statement.", "in",),
                p.cur_range(),
            ));

            M_BOGUS
        }
    } else if p.nth_at(1, T!['(']) {
        let m = p.start();
        if let Present(_identifier) = parse_identifier_expression(p) {
            p.bump(T!['(']);

            parse_assignment_expression_or_higher(p, ExpressionContext::default())
                .or_add_diagnostic(p, m_parse_error::expected_expression);

            p.expect(T![,]);

            let context = VariableDeclaratorContext::new(VariableDeclarationParent::For);

            parse_variable_declarator(p, &context)
                .or_add_diagnostic(p, m_parse_error::expected_declaration);

            if p.expect(T![')']) {
                m.complete(p, M_FOR_ITERATOR_FACTORY);
            } else {
                m.complete(p, M_BOGUS_EXPRESSION);
            }

            return M_FOR_ALL_STATEMENT;
        }
        m.complete(p, M_BOGUS_EXPRESSION);

        M_BOGUS
    } else {
        let checkpoint = p.checkpoint();
        let init_expr = parse_expression(
            p,
            ExpressionContext::default()
                .and_include_in(false)
                .and_object_expression_allowed(has_l_paren),
        );
        if p.at(T![in]) {
            // forall (assignment_pattern in ...
            if let Present(assignment_expr) = init_expr {
                expression_to_assignment_pattern(p, assignment_expr, checkpoint);
            }

            return parse_forall_in_head(p);
        }
        M_BOGUS
    }
}

/// Expects to be positioned right before the of or in keyword
fn parse_forall_in_head(p: &mut MParser) -> MSyntaxKind {
    p.bump_any();
    parse_expression(p, ExpressionContext::default())
        .or_add_diagnostic(p, m_parse_error::expected_expression);

    M_FOR_ALL_IN_STATEMENT
}

/// Either a traditional for statement or a for.. in statement
// for (var i = 5; i < 10; i++) {}
// for (;;) {}
fn parse_for_statement(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![for]);

    let has_l_paren = p.expect(T!['(']);
    let kind = parse_for_head(p, has_l_paren);
    p.expect(T![')']);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::For)
    })
    .or_add_diagnostic(p, expected_statement);

    let completed = m.complete(p, kind);

    Present(completed)
}

/// Either a traditional for statement or a for.. in statement
// for (var i = 5; i < 10; i++) {}
// for (;;) {}
fn parse_forall_statement(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![forall]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![forall]);

    let has_l_paren = p.expect(T!['(']);
    let kind = parse_forall_head(p, has_l_paren);
    p.expect(T![')']);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::For)
    })
    .or_add_diagnostic(p, expected_statement);

    let completed = m.complete(p, kind);

    Present(completed)
}

struct SwitchCaseStatementList;

impl ParseNodeList for SwitchCaseStatementList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_STATEMENT_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_statement(p, StatementContext::StatementList)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at_ts(token_set![T![default], T![case], T!['}']])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_STATEMENT, STMT_RECOVERY_SET),
            m_parse_error::expected_case,
        )
    }
}

// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn parse_switch_clause(p: &mut MParser, first_default: &mut Option<TextRange>) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        T![default] => {
            // in case we have two `default` expression, we mark the second one
            // as `M_CASE_CLAUSE` where the "default" keyword is an bogus node
            let syntax_kind = if first_default.is_some() {
                let discriminant = p.start();
                p.bump_any(); // interpret `default` as the test of the case
                discriminant.complete(p, M_BOGUS_EXPRESSION);
                M_CASE_CLAUSE
            } else {
                p.expect(T![default]);
                M_DEFAULT_CLAUSE
            };

            p.expect(T![:]);
            SwitchCaseStatementList.parse_list(p);
            let default = m.complete(p, syntax_kind);
            if let Some(first_default_range) = first_default {
                let err = p
                    .err_builder(
                        "Multiple default clauses inside of a switch statement are not allowed",
                        default.range(p),
                    )
                    .with_detail(default.range(p), "a second clause here is not allowed")
                    .with_detail(
                        *first_default_range,
                        "the first default clause is defined here",
                    );

                p.error(err);
            }

            Present(default)
        }
        T![case] => {
            p.expect(T![case]);
            parse_expression(p, ExpressionContext::default())
                .or_add_diagnostic(p, m_parse_error::expected_expression);
            p.expect(T![:]);

            SwitchCaseStatementList.parse_list(p);
            Present(m.complete(p, M_CASE_CLAUSE))
        }
        _ => {
            m.abandon(p);
            Absent
        }
    }
}
#[derive(Default)]
struct SwitchCasesList {
    first_default: Option<TextRange>,
}

impl ParseNodeList for SwitchCasesList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_SWITCH_CASE_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        let clause = parse_switch_clause(p, &mut self.first_default);

        if let Present(marker) = &clause {
            if marker.kind(p) == M_DEFAULT_CLAUSE && self.first_default.is_none() {
                self.first_default = Some(marker.range(p));
            }
        }

        clause
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if let Present(marker) = parsed_element {
            Ok(marker)
        } else {
            let m = p.start();
            let statements = p.start();

            let recovered_element = parsed_element.or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    M_BOGUS_STATEMENT,
                    token_set![T![default], T![case], T!['}']],
                )
                .enable_recovery_on_line_break(),
                m_parse_error::expected_case_or_default,
            );

            match recovered_element {
                Ok(marker) => {
                    statements.complete(p, M_STATEMENT_LIST);
                    m.complete(p, M_CASE_CLAUSE);
                    Ok(marker)
                }
                Err(err) => {
                    statements.abandon(p);
                    m.abandon(p);
                    Err(err)
                }
            }
        }
    }
}

/// A switch statement such as
///
/// ```js
/// switch (a) {
///     case foo:
///         bar();
/// }
/// ```
// test js switch_stmt
// switch (foo) {
//  case bar:
//  default:
// }
fn parse_switch_statement(p: &mut MParser) -> ParsedSyntax {
    // test_err js switch_stmt_err
    // switch foo {}
    // switch {}
    // switch { var i = 0 }
    // switch { var i = 0; case "bar": {} }
    // switch (foo) {
    //   default: {}
    //   default: {}
    // }
    // switch (foo) { case : }

    if !p.at(T![switch]) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![switch]);
    parenthesized_expression(p);
    p.expect(T!['{']);

    p.with_state(EnterBreakable(BreakableKind::Switch), |p| {
        SwitchCasesList::default().parse_list(p)
    });

    p.expect(T!['}']);
    Present(m.complete(p, M_SWITCH_STATEMENT))
}

fn parse_catch_clause(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![catch]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![catch]);

    parse_catch_declaration(p).ok();
    parse_block_stmt(p).or_add_diagnostic(p, m_parse_error::expected_block_statement);

    Present(m.complete(p, M_CATCH_CLAUSE))
}

fn parse_catch_declaration(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let declaration_marker = p.start();

    p.bump_any(); // bump (
    parse_binding_pattern(p, ExpressionContext::default()).or_add_diagnostic(p, expected_binding);

    p.expect(T![')']);

    Present(declaration_marker.complete(p, M_CATCH_DECLARATION))
}

/// A try statement such as
///
/// ```js
/// try {
///     something();
/// } catch (a) {
///
/// }
/// ```
// test js try_stmt
// try {} catch {}
// try {} catch (e) {}
// try {} catch {} finally {}
// try {} catch (e) {} finally {}
// try {} finally {}
pub(crate) fn parse_try_statement(p: &mut MParser) -> ParsedSyntax {
    // TODO: recover from `try catch` and `try finally`. The issue is block_items
    // will cause infinite recursion because parsing a stmt would not consume the catch token
    // and block_items would not exit, and if we exited on any error that would greatly limit
    // block_items error recovery

    if !p.at(T![try]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![try]);

    parse_block_stmt(p).or_add_diagnostic(p, m_parse_error::expected_block_statement);

    let catch = parse_catch_clause(p);

    if p.at(T![finally]) {
        catch.ok();

        let finalizer = p.start();
        p.expect(T![finally]);
        parse_block_stmt(p).or_add_diagnostic(p, m_parse_error::expected_block_statement);
        finalizer.complete(p, M_FINALLY_CLAUSE);
        Present(m.complete(p, M_TRY_FINALLY_STATEMENT))
    } else {
        catch.or_add_diagnostic(p, m_parse_error::expected_catch_clause);
        Present(m.complete(p, M_TRY_STATEMENT))
    }
}
