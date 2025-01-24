use super::single_token_parse_recovery::SingleTokenParseRecovery;
use super::{Absent, MParser, ParseRecoveryTokenSet, ParsedSyntax, Present, RecoveryResult};

use super::expr::{
    is_nth_at_reference_identifier, parse_assignment_expression_or_higher, parse_expression,
    parse_reference_identifier, ExpressionContext,
};
use super::function::{parse_function_body, parse_parameter_list, ParameterContext};
use super::m_parse_error;
use super::state::SignatureFlags;
use super::syntax::{
    MSyntaxKind::{self, *},
    T,
};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::*;

// test js object_expr
// let a = {};
// let b = {foo,}
//
// test_err js object_expr_err
// let a = {, foo}
// let b = { foo bar }
// let b = { foo

struct ObjectMembersList;

impl ParseSeparatedList for ObjectMembersList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_OBJECT_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_object_member(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_MEMBER, token_set![T![,], T!['}'], T![;], T![:]])
                .enable_recovery_on_line_break(),
            m_parse_error::expected_object_member,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An object literal such as `{ a: b, "b": 5 + 5 }`.
pub(crate) fn parse_object_expression(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![@]) && !p.nth_at(1, T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);
    p.bump(T!['{']);

    ObjectMembersList.parse_list(p);

    p.expect(T!['}']);
    Present(m.complete(p, M_OBJECT_EXPRESSION))
}

struct HashMapMembersList;

impl ParseSeparatedList for HashMapMembersList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_HASH_MAP_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_object_member(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_MEMBER, token_set![T![,], T![')'], T![;], T![:]])
                .enable_recovery_on_line_break(),
            m_parse_error::expected_object_member,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

pub(crate) fn parse_hashmap_expression(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![@]) && !p.nth_at(1, T!['(']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);
    p.bump(T!['(']);

    HashMapMembersList.parse_list(p);

    p.expect(T![')']);
    Present(m.complete(p, M_HASH_MAP_EXPRESSION))
}

/// An individual object property such as `"a": b` or `5: 6 + 6`.
fn parse_object_member(p: &mut MParser) -> ParsedSyntax {
    match p.cur() {
        // test js object_expr_spread_prop
        // let a = {...foo}
        T![...] => {
            let m = p.start();
            p.bump_any();
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
                .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);
            Present(m.complete(p, M_SPREAD))
        }
        _ => {
            let m = p.start();

            if is_nth_at_reference_identifier(p, 0)
                && !token_set![T!['('], T![<], T![:]].contains(p.nth(1))
            {
                // test js object_expr_ident_prop
                // ({foo})
                parse_reference_identifier(p).unwrap();

                // There are multiple places where it's first needed to parse an expression to determine if
                // it is an assignment target or not. This requires that parse expression is valid for any
                // assignment expression. Thus, it's needed that the parser silently parses over a "{ arrow = test }"
                // property
                if p.at(T![=]) {
                    // test js assignment_shorthand_prop_with_initializer
                    // for ({ arrow = () => {} } of [{}]) {}
                    //
                    // test_err js object_shorthand_with_initializer
                    // ({ arrow = () => {} })
                    p.error(p.err_builder("Did you mean to use a `:`?", p.cur_range()));
                    p.bump(T![=]);
                    parse_assignment_expression_or_higher(p, ExpressionContext::default()).ok();
                    return Present(m.complete(p, M_BOGUS_MEMBER));
                }

                return Present(m.complete(p, M_SHORTHAND_PROPERTY_OBJECT_MEMBER));
            }

            let checkpoint = p.checkpoint();
            let member_name = parse_object_member_name(p)
                .or_add_diagnostic(p, m_parse_error::expected_object_member);

            // test js object_expr_method
            // let b = {
            //   foo() {},
            //   "bar"(a, b, c) {},
            //   ["foo" + "bar"](a) {},
            //   5(...rest) {}
            // }

            if member_name.is_some() {
                // test js object_prop_name
                // let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}

                // test js object_expr_ident_literal_prop
                // let b = { a: true }

                // If the member name was a literal OR we're at a colon
                p.expect(T![:]);

                // test js object_prop_in_rhs
                // for ({ a: "x" in {} };;) {}
                parse_assignment_expression_or_higher(p, ExpressionContext::default())
                    .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);
                Present(m.complete(p, M_PROPERTY_OBJECT_MEMBER))
            } else {
                // test_err js object_expr_error_prop_name
                // let a = { /: 6, /: /foo/ }
                // let b = {{}}

                // test_err js object_expr_non_ident_literal_prop
                // let d = {5}

                SingleTokenParseRecovery::new(token_set![T![:], T![,]], M_BOGUS).recover(p);

                if p.eat(T![:]) {
                    parse_assignment_expression_or_higher(p, ExpressionContext::default())
                        .or_add_diagnostic(p, m_parse_error::expected_object_member);
                    Present(m.complete(p, M_PROPERTY_OBJECT_MEMBER))
                } else {
                    // It turns out that this isn't a valid member after all. Make sure to throw
                    // away everything that has been parsed so far so that the caller can
                    // do its error recovery
                    p.rewind(checkpoint);
                    m.abandon(p);
                    Absent
                }
            }
        }
    }
}

// test js object_member_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
/// Parses a `MAnyObjectMemberName` and returns its completion marker
pub fn parse_object_member_name(p: &mut MParser) -> ParsedSyntax {
    match p.cur() {
        T!['['] => parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    }
}

pub fn is_nth_at_type_member_name(p: &mut MParser, offset: usize) -> bool {
    let nth = p.nth(offset);

    let start_names = token_set![M_STRING_LITERAL, M_NUMBER_LITERAL, T![ident], T!['[']];

    nth.is_keyword() || start_names.contains(nth)
}

pub fn is_at_object_member_name(p: &mut MParser) -> bool {
    is_nth_at_type_member_name(p, 0)
}

pub fn parse_computed_member_name(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['[']);

    // test js computed_member_name_in
    // for ({["x" in {}]: 3} ;;) {}
    parse_expression(p, ExpressionContext::default())
        .or_add_diagnostic(p, m_parse_error::expected_expression);

    p.expect(T![']']);
    Present(m.complete(p, M_COMPUTED_MEMBER_NAME))
}

pub(crate) fn is_at_literal_member_name(p: &mut MParser, offset: usize) -> bool {
    matches!(
        p.nth(offset),
        M_STRING_LITERAL | M_NUMBER_LITERAL | T![ident]
    ) || p.nth(offset).is_keyword()
}

pub(crate) fn parse_literal_member_name(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        M_STRING_LITERAL | M_NUMBER_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    }
    Present(m.complete(p, M_LITERAL_MEMBER_NAME))
}
