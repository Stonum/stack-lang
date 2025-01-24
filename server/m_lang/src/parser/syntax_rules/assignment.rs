use super::rewrite_parser::{RewriteMarker, RewriteParser, RewriteToken};
use super::MParserCheckpoint;

use super::class::parse_initializer_clause;
use super::expr::{is_at_identifier, parse_conditional_expr, parse_unary_expr, ExpressionContext};
use super::m_parse_error::{
    expected_assignment_target, expected_identifier, expected_object_member_name,
    invalid_assignment_error,
};
use super::object::{is_at_object_member_name, parse_object_member_name};
use super::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use super::rewrite::{rewrite_events, RewriteParseEvents};
use super::syntax::{MSyntaxKind::*, *};
use super::MParser;

use biome_parser::diagnostic::expected_any;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::*;

// test js assignment_target
// foo += bar = b ??= 3;
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count === 3
// a['b'] = c[d] = "test"

// test_err js invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;
// a?.b = b;
// a?.["b"] = b;
// (a +) = b;

// test ts ts_non_null_assignment
// let a;
// a! &= 2;
// let b = { a: null };
// b.a! &= 5

// test ts ts_as_assignment
// let a: any;
// type B<A> = { a: A };
// (a as string) = "string";
// ((a as any) as string) = null;
// ({ b: a as string } = { b: "test" });
// ([ a as string ] = [ "test" ]);
// for (a as string in []) {}
// (a as B<string>) = { a: "test" };
// (<number> a) += 1

// test_err ts ts_as_assignment_no_parenthesize
// let a: any;
// a as string = "string";
// (a() as string) = "string";
// <number> a = 3;

// test ts ts_satisfies_assignment
// let a: any;
// type B<A> = { a: A };
// (a satisfies string) = "string";
// ((a satisfies any) satisfies string) = null;
// ({ b: a satisfies string } = { b: "test" });
// ([ a satisfies string ] = [ "test" ]);
// for (a satisfies string in []) {}
// (a satisfies B<string>) = { a: "test" };

// test_err ts ts_satisfies_assignment_no_parenthesize
// let a: any;
// a satisfies string = "string";
// (a() satisfies string) = "string";

/// Converts the passed in lhs expression to an assignment pattern
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub fn expression_to_assignment_pattern(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> CompletedMarker {
    match target.kind(p) {
        // M_OBJECT_EXPRESSION => {
        //     p.rewind(checkpoint);
        //     ObjectAssignmentPattern.parse_object_pattern(p).unwrap()
        // }
        // M_ARRAY_EXPRESSION => {
        //     p.rewind(checkpoint);
        //     ArrayAssignmentPattern.parse_array_pattern(p).unwrap()
        // }
        _ => expression_to_assignment(p, target, checkpoint),
    }
}

// test js array_or_object_member_assignment
// [{
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42] = [23];
// ({ x: {
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42 } = { x: 23 });
pub fn parse_assignment_pattern(p: &mut MParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = parse_conditional_expr(p, ExpressionContext::default());

    assignment_expression
        .map(|expression| expression_to_assignment_pattern(p, expression, checkpoint))
}

/// Re-parses an expression as an assignment.
pub fn expression_to_assignment(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> CompletedMarker {
    try_expression_to_assignment(p, target, checkpoint).unwrap_or_else(
        // test_err js M_regex_assignment
        // /=0*_:m/=/*_:|
        |mut invalid_assignment_target| {
            // Doesn't seem to be a valid assignment target. Recover and create an error.
            invalid_assignment_target.change_kind(p, M_BOGUS_ASSIGNMENT);

            p.error(invalid_assignment_error(
                p,
                invalid_assignment_target.range(p),
            ));

            invalid_assignment_target
        },
    )
}

pub enum AssignmentExprPrecedence {
    Unary,
    Conditional,
}

impl AssignmentExprPrecedence {
    fn parse_expression(&self, p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
        match self {
            AssignmentExprPrecedence::Unary => parse_unary_expr(p, context),
            AssignmentExprPrecedence::Conditional => parse_conditional_expr(p, context),
        }
    }
}

pub fn parse_assignment(
    p: &mut MParser,
    expr_kind: AssignmentExprPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = expr_kind.parse_expression(p, context);

    assignment_expression.map(|expr| expression_to_assignment(p, expr, checkpoint))
}

fn try_expression_to_assignment(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> Result<CompletedMarker, CompletedMarker> {
    if !matches!(
        target.kind(p),
        M_PARENTHESIZED_EXPRESSION
            | M_STATIC_MEMBER_EXPRESSION
            | M_COMPUTED_MEMBER_EXPRESSION
            | M_IDENTIFIER_EXPRESSION
    ) {
        return Err(target);
    }

    // At this point it's guaranteed that the root node can be mapped to an assignment,
    // but it's not yet guaranteed if it is valid or not (for example, a static member expression
    // is valid, except if it uses optional chaining).
    let mut reparse_assignment = ReparseAssignment::new();
    rewrite_events(&mut reparse_assignment, checkpoint, p);

    Ok(reparse_assignment.result.unwrap())
}

struct ReparseAssignment {
    // Stores the unfinished parents
    // Index 0: Re-mapped kind of the node
    // Index 1: Started marker. A `None` marker means that this node should be dropped
    //          from the re-written tree
    parents: Vec<(MSyntaxKind, Option<RewriteMarker>)>,
    // Stores the completed assignment node (valid or invalid).
    result: Option<CompletedMarker>,
    // Tracks if the visitor is still inside an assignment
    inside_assignment: bool,
}

impl ReparseAssignment {
    pub fn new() -> Self {
        Self {
            parents: Vec::default(),
            result: None,
            inside_assignment: true,
        }
    }
}

/// Rewrites expressions to assignments
/// * Converts parenthesized expression to parenthesized assignment
/// * Converts computed/static member expressions to computed/static member assignment.
///   Validates that the operator isn't `?.` .
/// * Converts identifier expressions to identifier assignment, drops the inner reference identifier
impl RewriteParseEvents for ReparseAssignment {
    fn start_node(&mut self, kind: MSyntaxKind, p: &mut RewriteParser) {
        if !self.inside_assignment {
            self.parents.push((kind, Some(p.start())));
            return;
        }

        // Make sure to also add the kind to the match in `try_expression_to_assignment`
        let mapped_kind = match kind {
            M_PARENTHESIZED_EXPRESSION => M_PARENTHESIZED_ASSIGNMENT,
            M_STATIC_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                M_STATIC_MEMBER_ASSIGNMENT
            }
            M_COMPUTED_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                M_COMPUTED_MEMBER_ASSIGNMENT
            }
            M_IDENTIFIER_EXPRESSION => M_IDENTIFIER_ASSIGNMENT,
            M_REFERENCE_IDENTIFIER => {
                self.parents.push((kind, None)); // Omit reference identifiers
                return;
            }
            _ => {
                self.inside_assignment = false;
                M_BOGUS_ASSIGNMENT
            }
        };

        self.parents.push((mapped_kind, Some(p.start())));
    }

    fn finish_node(&mut self, p: &mut RewriteParser) {
        let (kind, m) = self.parents.pop().unwrap();

        if let Some(m) = m {
            let mut completed = m.complete(p, kind);

            match kind {
                M_IDENTIFIER_ASSIGNMENT => {
                    // test_err js eval_arguments_assignment
                    // eval = "test";
                    // arguments = "test";
                    let name = completed.text(p);
                    if matches!(name, "eval" | "arguments") {
                        let error = p.err_builder(
                            format!("Illegal use of `{name}` as an identifier in strict mode"),
                            completed.range(p),
                        );
                        p.error(error);

                        completed.change_to_bogus(p);
                    }
                }
                M_BOGUS_ASSIGNMENT => {
                    let range = completed.range(p);
                    p.error(
                        p.err_builder(
                            format!("Invalid assignment to `{}`", completed.text(p)),
                            range,
                        )
                        .with_hint("This expression cannot be assigned to"),
                    );
                }
                _ => {}
            }

            self.result = Some(completed.into());
        }

        // if AnyTsType::can_cast(kind)
        //     && matches!(
        //         self.parents.last(),
        //         Some((
        //             TS_TYPE_ASSERTION_ASSIGNMENT | TS_AS_ASSIGNMENT | TS_SATISFIES_ASSIGNMENT,
        //             _
        //         ))
        //     )
        // {
        //     self.inside_assignment = true;
        // }
    }

    fn token(&mut self, token: RewriteToken, p: &mut RewriteParser) {
        let parent = self.parents.last_mut();

        // if let Some((parent_kind, _)) = parent {
        //     if matches!(
        //         *parent_kind,
        //         M_COMPUTED_MEMBER_ASSIGNMENT | M_STATIC_MEMBER_ASSIGNMENT
        //     ) && token.kind == T![?.]
        //     {
        //         *parent_kind = M_BOGUS_ASSIGNMENT
        //     }
        // }

        p.bump(token)
    }
}
