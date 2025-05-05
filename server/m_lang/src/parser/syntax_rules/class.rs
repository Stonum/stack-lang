use super::annotation::parse_annotation_list;
use super::binding::parse_identifier_binding;
use super::ParsedSyntax::{Absent, Present};
use super::{MParser, ParsedSyntax, RecoveryResult};

use super::expr::{
    eat_doc_string_expression, parse_assignment_expression_or_higher, parse_lhs_expr,
    ExpressionContext,
};

use super::function::{
    parse_any_parameter, parse_formal_parameter, parse_function_body, parse_parameter_list,
    parse_parameters_list,
};
use super::m_parse_error;

use super::object::{
    is_at_literal_member_name, parse_computed_member_name, parse_literal_member_name,
};
use super::state::{EnterParameters, SignatureFlags};
use super::stmt::StatementContext;

use super::syntax::MSyntaxKind::*;
use super::syntax::{MSyntaxKind, T};

use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::*;

use biome_rowan::{SyntaxKind, TextRange};

// test class_declaration
// class foo {}
// class foo extends bar {}

// test_err class_decl_err
// class {}
// class extends bar {}
// class foo { set {} }
// class extends {}

/// Parses a class declaration if it is valid and otherwise returns [Invalid].
///
/// A class can be invalid if
/// * It uses an illegal identifier name
pub(crate) fn parse_class_declaration(
    p: &mut MParser,
    context: StatementContext,
    annotation: Option<CompletedMarker>,
) -> ParsedSyntax {
    if !matches!(p.cur(), T![class]) {
        return Absent;
    }

    let mut class = parse_class(p, ClassKind::Declaration, annotation);

    if !class.kind(p).is_bogus() && context.is_single_statement() {
        // test_err class_in_single_statement_context
        // if (true) class A {}
        p.error(
            p.err_builder(
                "Classes can only be declared at top level or inside a block",
                class.range(p),
            )
            .with_hint("wrap the class in a block statement"),
        );
        class.change_to_bogus(p)
    }

    Present(class)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
    Declaration,
}

impl From<ClassKind> for MSyntaxKind {
    fn from(_kind: ClassKind) -> Self {
        M_CLASS_DECLARATION
    }
}

#[inline]
fn parse_class(
    p: &mut MParser,
    kind: ClassKind,
    annotation: Option<CompletedMarker>,
) -> CompletedMarker {
    let a = annotation.unwrap_or_else(|| {
        let a = p.start();
        a.complete(p, M_ANNOTATION_GROUP_LIST)
    });
    let m = a.precede(p);

    let class_token_range = p.cur_range();
    p.expect(T![class]);

    // test_err class_decl_no_id
    // class {}
    // class implements B {}
    let id = parse_identifier_binding(p);

    // parse class id
    if id == Absent {
        let err = p.err_builder(
            "class declarations must have a name",
            class_token_range.start()..p.cur_range().start(),
        );

        p.error(err);
    }

    eat_class_heritage_clause(p);

    eat_doc_string_expression(p);

    p.expect(T!['{']);
    ClassMembersList.parse_list(p);
    p.expect(T!['}']);

    m.complete(p, kind.into())
}

// test_err  class_extends_err
// class A extends bar extends foo {}
// class B extends bar, foo {}
// class C implements B {}
//
/// Eats a class's 'extends' clauses, attaching them to the current active node.
/// Implements error recovery in case a class has multiple extends/implements clauses or if they appear
/// out of order
fn eat_class_heritage_clause(p: &mut MParser) {
    let mut first_extends: Option<CompletedMarker> = None;

    loop {
        match p.cur() {
            T![extends] => {
                let current = parse_extends_clause(p).expect(
                    "Expected extends clause because parser is positioned at extends keyword",
                );

                match first_extends.as_ref() {
                    None => first_extends = Some(current),
                    Some(first_extends) => p.error(
                        p.err_builder("'extends' clause already seen.", current.range(p))
                            .with_detail(first_extends.range(p), "first 'extends' clause"),
                    ),
                }
            }
            _ => break,
        }
    }
}

fn parse_extends_clause(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    let extends_end = p.cur_range().end();
    p.expect(T![extends]);

    if parse_extends_expression(p).is_absent() {
        p.error(p.err_builder("'extends' list cannot be empty.", extends_end..extends_end))
    }

    while p.at(T![,]) {
        let comma_range = p.cur_range();
        p.bump(T![,]);

        let extra = p.start();
        if parse_extends_expression(p).is_absent() {
            p.error(p.err_builder("Trailing comma not allowed.", comma_range));
            extra.abandon(p);
            break;
        }

        let extra_class = extra.complete(p, M_BOGUS);

        p.error(p.err_builder(
            "Classes can only extend a single class.",
            extra_class.range(p),
        ));
    }

    Present(m.complete(p, M_EXTENDS_CLAUSE))
}

fn parse_extends_expression(p: &mut MParser) -> ParsedSyntax {
    if p.at(T!['{']) && p.nth_at(1, T!['}']) {
        // Don't eat the body of the class as an object expression except if we have
        // * extends {} {
        // * extends {},
        if !matches!(p.nth(2), T![extends] | T!['{'] | T![,]) {
            return Absent;
        }
    }

    parse_lhs_expr(p, ExpressionContext::default())
}

struct ClassMembersList;

impl ParseNodeList for ClassMembersList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: MSyntaxKind = M_CLASS_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_class_member(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        // test_err invalid_method_recover
        // class {
        //   [1 + 1] = () => {
        //     let a=;
        //   };
        // };
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_MEMBER, token_set![T![;], T![ident], T!['}'],]),
            m_parse_error::expected_class_member,
        )
    }
}

// test class_declare
// class B { declare() {} }
// class B { declare = foo }

fn parse_class_member(p: &mut MParser) -> ParsedSyntax {
    let member_marker = if p.at(T![:]) && p.nth_at(1, T!['[']) {
        parse_annotation_list(p).precede(p)
    } else {
        p.start().complete(p, M_ANNOTATION_GROUP_LIST).precede(p)
    };

    // test class_empty_element
    // class foo { ;;;;;;;;;; get foo() {};;;;}
    if p.eat(T![;]) {
        return Present(member_marker.complete(p, M_EMPTY_CLASS_MEMBER));
    }

    parse_class_member_impl(p, member_marker)
}

fn parse_class_member_impl(p: &mut MParser, member_marker: Marker) -> ParsedSyntax {
    let start_token_pos = p.source().position();

    // test getter_class_member
    // class Getters {
    //   get foo() {}
    //   get static() {}
    //   get "baz"() {}
    //   get ["a" + "b"]() {}
    //   get 5() {}
    // }
    //
    // test_err method_getter_err
    // class foo {
    //  get {}
    // }
    //

    // test setter_class_member
    // class Setters {
    //   set foo(a) {}
    //   set bax(a,) {}
    //   set static(a) {}
    //   set "baz"(a) {}
    //   set ["a" + "b"](a) {}
    //   set 5(a) {}
    //   set 6(a,) {}
    // }
    //
    // test_err setter_class_member
    // class Setters {
    //   set foo() {}
    // }
    if matches!(p.cur(), T![get] | T![set]) && is_at_class_member_name(p, 1) {
        let is_getter = p.at(T![get]);
        if is_getter {
            p.expect(T![get]);
        } else {
            p.expect(T![set]);
        }

        // So we've seen a get that now must be followed by a getter/setter name
        parse_class_member_name(p).or_add_diagnostic(p, m_parse_error::expected_class_member_name);

        let completed = if is_getter {
            p.expect(T!['(']);
            p.expect(T![')']);

            eat_doc_string_expression(p);

            let member_kind = expect_accessor_body(p);
            member_marker.complete(p, member_kind.as_getter_syntax_kind())
        } else {
            let has_l_paren = p.expect(T!['(']);
            p.with_state(EnterParameters(SignatureFlags::empty()), |p| {
                parse_formal_parameter(
                    p,
                    ExpressionContext::default().and_object_expression_allowed(has_l_paren),
                )
            })
            .or_add_diagnostic(p, m_parse_error::expected_parameter);

            if p.at(T![,]) {
                p.bump_any();
            }

            p.expect(T![')']);

            eat_doc_string_expression(p);

            let member_kind = expect_accessor_body(p);
            member_marker.complete(p, member_kind.as_setter_syntax_kind())
        };

        return Present(completed);
    }

    let is_constructor = is_at_constructor(p);
    let member_name =
        parse_class_member_name(p).or_add_diagnostic(p, m_parse_error::expected_class_member_name);

    if is_at_method_class_member(p, 0) {
        // test  constructor_class_member
        // class Foo {
        //   constructor(a) {
        //     this.a = a;
        //   }
        // }
        // class Bar {
        //   "constructor"(b) {
        //     this.b = b;
        //   }
        // }
        return if is_constructor {
            Present(parse_constructor_class_member_body(p, member_marker))
        } else {
            // test method_class_member
            // class Test {
            //   method() {}
            //   "foo"() {}
            //   ["foo" + "bar"]() {}
            //   5() {}
            // }
            Present(parse_method_class_member_rest(
                p,
                member_marker,
                SignatureFlags::empty(),
            ))
        };
    }

    match member_name {
        Some(_) => {
            debug_assert_eq!(
                p.source().position(),
                start_token_pos,
                "Parser shouldn't be progressing when returning Absent"
            );
            Absent
        }
        None => {
            // test_err block_stmt_in_class
            // class S{{}}
            debug_assert_eq!(
                p.source().position(),
                start_token_pos,
                "Parser shouldn't be progressing when returning Absent"
            );

            member_marker.abandon(p);
            Absent
        }
    }
}

/// Eats the '?' token for optional member
fn optional_member_token(p: &mut MParser) -> Result<Option<TextRange>, TextRange> {
    if p.at(T![?]) {
        let range = p.cur_range();
        p.bump(T![?]);

        // test_err optional_member
        // class B { foo?; }

        let err = p.err_builder("`?` modifiers can only be used in TypeScript files", range);

        p.error(err);
        Err(range)
    } else {
        Ok(None)
    }
}

// test_err class_property_initializer
// class B { lorem = ; }
pub(crate) fn parse_initializer_clause(
    p: &mut MParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    if p.at(T![=]) {
        let m = p.start();
        p.bump(T![=]);

        parse_assignment_expression_or_higher(p, context)
            .or_add_diagnostic(p, m_parse_error::expected_expression_assignment);

        Present(m.complete(p, M_INITIALIZER_CLAUSE))
    } else {
        Absent
    }
}

// test_err class_member_method_parameters
// class B { foo(a {} }

/// Parses the body (everything after the identifier name) of a method class member
/// that includes: parameters and its types, return type and method body
fn parse_method_class_member_rest(
    p: &mut MParser,
    m: Marker,
    flags: SignatureFlags,
) -> CompletedMarker {
    let optional = optional_member_token(p);

    parse_parameter_list(p, flags).or_add_diagnostic(p, m_parse_error::expected_class_parameters);

    eat_doc_string_expression(p);
    let member_kind = expect_method_body(p, ClassMethodMemberKind::Method(flags));
    let mut member = m.complete(p, member_kind.as_method_syntax_kind());

    if optional.is_err() {
        // error already emitted by `optional_member_token()`
        member.change_to_bogus(p);
    }

    member
}

#[derive(Debug)]
enum MemberKind {
    Declaration,
}

impl MemberKind {
    const fn as_method_syntax_kind(&self) -> MSyntaxKind {
        match self {
            MemberKind::Declaration => M_METHOD_CLASS_MEMBER,
        }
    }

    const fn as_constructor_syntax_kind(&self) -> MSyntaxKind {
        match self {
            MemberKind::Declaration => M_CONSTRUCTOR_CLASS_MEMBER,
        }
    }

    const fn as_setter_syntax_kind(&self) -> MSyntaxKind {
        match self {
            MemberKind::Declaration => M_SETTER_CLASS_MEMBER,
        }
    }

    const fn as_getter_syntax_kind(&self) -> MSyntaxKind {
        match self {
            MemberKind::Declaration => M_GETTER_CLASS_MEMBER,
        }
    }
}

#[derive(Debug)]
enum ClassMethodMemberKind {
    /// `get` or `set`
    Accessor,

    /// A class's constructor function
    Constructor,

    /// Any other regular method
    Method(SignatureFlags),
}

impl ClassMethodMemberKind {
    const fn signature_flags(&self) -> SignatureFlags {
        match self {
            ClassMethodMemberKind::Method(flags) => *flags,
            ClassMethodMemberKind::Constructor => SignatureFlags::CONSTRUCTOR,
            ClassMethodMemberKind::Accessor => SignatureFlags::empty(),
        }
    }
}

/// Parses the body of a method (constructor, getter, setter, or regular method).
///
/// The method determines which case applies to the current member and emits a diagnostic if:
/// * the body is absent for a method declaration
/// * the body is present for a method signature
/// * a method signature isn't terminate by a semicolon or ASI
///
/// The method returns the inferred kind (signature or declaration) of the parsed method body
fn expect_method_body(p: &mut MParser, method_kind: ClassMethodMemberKind) -> MemberKind {
    let body = parse_function_body(p, method_kind.signature_flags());

    body.or_add_diagnostic(p, m_parse_error::expected_class_method_body);
    MemberKind::Declaration
}

// test_err getter_class_no_body
// class Getters {
//   get foo()
// }

// test_err setter_class_no_body
// class Setters {
//   set foo(a)
// }
fn expect_accessor_body(p: &mut MParser) -> MemberKind {
    expect_method_body(p, ClassMethodMemberKind::Accessor)
}

fn parse_constructor_class_member_body(p: &mut MParser, member_marker: Marker) -> CompletedMarker {
    if let Ok(Some(range)) = optional_member_token(p) {
        let err = p.err_builder("constructors cannot be optional", range);

        p.error(err);
    }

    parse_constructor_parameter_list(p)
        .or_add_diagnostic(p, m_parse_error::expected_constructor_parameters);

    eat_doc_string_expression(p);
    let constructor_kind = expect_method_body(p, ClassMethodMemberKind::Constructor);

    member_marker.complete(p, constructor_kind.as_constructor_syntax_kind())
}

fn parse_constructor_parameter_list(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();

    // test  super_expression_in_constructor_parameter_list
    // class A extends B { constructor(c = super()) {} }
    //
    // test_err super_expression_in_constructor_parameter_list
    // class A extends B { constructor(super()) {} }
    let flags = SignatureFlags::CONSTRUCTOR;

    parse_parameters_list(
        p,
        flags,
        parse_constructor_parameter,
        M_CONSTRUCTOR_PARAMETER_LIST,
    );
    Present(m.complete(p, M_CONSTRUCTOR_PARAMETERS))
}

// test_err m_constructor_parameter_reserved_names
// // SCRIPT
// class A { constructor(readonly, private, protected, public) {} }
fn parse_constructor_parameter(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    parse_any_parameter(p, context)
}

fn is_at_class_member_name(p: &mut MParser, offset: usize) -> bool {
    matches!(p.nth(offset), T!['[']) || is_at_literal_member_name(p, offset)
}

/// Parses a `AnyMClassMemberName` and returns its completion marker
fn parse_class_member_name(p: &mut MParser) -> ParsedSyntax {
    match p.cur() {
        T!['['] => parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    }
}

fn is_at_method_class_member(p: &mut MParser, mut offset: usize) -> bool {
    if p.nth_at(offset, T![?]) {
        offset += 1;
    }

    p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<])
}

fn is_at_constructor(p: &MParser) -> bool {
    p.at(T![constructor]) || matches!(p.cur_text(), "\"constructor\"" | "'constructor'")
}
