use super::ParsedSyntax::{Absent, Present};
use super::{MParser, ParsedSyntax, RecoveryResult};

use super::binding::parse_binding;
use super::expr::{parse_assignment_expression_or_higher, parse_lhs_expr, ExpressionContext};

use super::function::{
    parse_any_parameter, parse_formal_parameter, parse_function_body, parse_parameter_list,
    parse_parameters_list, ParameterContext,
};
use super::m_parse_error;
use super::m_parse_error::unexpected_body_inside_ambient_context;

use super::object::{
    is_at_literal_member_name, parse_computed_member_name, parse_literal_member_name,
};
use super::state::{EnterParameters, SignatureFlags};
use super::stmt::{optional_semi, StatementContext};

use super::syntax::MSyntaxKind::*;
use super::syntax::{MSyntaxKind, T};

use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::*;

use biome_rowan::{SyntaxKind, TextRange};

// test js class_declaration
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}

// test_err js class_decl_err
// class {}
// class extends bar {}
// class foo { set {} }
// class extends {}

/// Parses a class declaration if it is valid and otherwise returns [Invalid].
///
/// A class can be invalid if
/// * It uses an illegal identifier name
pub(super) fn parse_class_declaration(p: &mut MParser, context: StatementContext) -> ParsedSyntax {
    if !matches!(p.cur(), T![class]) {
        return Absent;
    }

    let mut class = parse_class(p, ClassKind::Declaration);

    if !class.kind(p).is_bogus() && context.is_single_statement() {
        // test_err js class_in_single_statement_context
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
    Expression,
}

impl ClassKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, ClassKind::Expression)
    }
}

impl From<ClassKind> for MSyntaxKind {
    fn from(kind: ClassKind) -> Self {
        match kind {
            ClassKind::Declaration => M_CLASS_DECLARATION,
            ClassKind::Expression => M_CLASS_EXPRESSION,
        }
    }
}

// test js class_named_abstract_is_valid_in_js
// class abstract {}

// test ts ts_class_named_abstract_is_valid_in_ts
// class abstract {}
#[inline]
fn parse_class(p: &mut MParser, kind: ClassKind) -> CompletedMarker {
    let m = p.start();
    let is_abstract = false;

    let class_token_range = p.cur_range();
    p.expect(T![class]);

    // test_err ts class_decl_no_id
    // class {}
    // class implements B {}
    let id = parse_binding(p);

    // parse class id
    match id {
        Present(id) => {
            let text = p.text(id.range(p));
        }
        Absent => {
            if !kind.is_id_optional() {
                let err = p.err_builder(
                    "class declarations must have a name",
                    class_token_range.start()..p.cur_range().start(),
                );

                p.error(err);
            }
        }
    }

    eat_class_heritage_clause(p);

    p.expect(T!['{']);
    ClassMembersList {
        inside_abstract_class: is_abstract,
    }
    .parse_list(p);
    p.expect(T!['}']);

    m.complete(p, kind.into())
}

// test_err js class_extends_err
// class A extends bar extends foo {}
// class B extends bar, foo {}
// class C implements B {}
//
// test_err ts ts_class_heritage_clause_errors
// class A {}
// interface Int {}
// class B implements Int extends A {}
// class C implements Int implements Int {}
// class D implements {}
// class E extends {}
// class F extends E, {}
/// Eats a class's 'implements' and 'extends' clauses, attaching them to the current active node.
/// Implements error recovery in case a class has multiple extends/implements clauses or if they appear
/// out of order
fn eat_class_heritage_clause(p: &mut MParser) {
    let mut first_extends: Option<CompletedMarker> = None;
    let mut first_implements: Option<CompletedMarker> = None;

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

// test ts ts_extends_generic_type
// type IHasVisualizationModel = string;
// class D extends C<IHasVisualizationModel> {
//     x = "string";
// }
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
        // * extends {} implements
        // * extends {},
        if !matches!(p.nth(2), T![extends] | T!['{'] | T![,]) {
            return Absent;
        }
    }

    parse_lhs_expr(p, ExpressionContext::default())
}

struct ClassMembersList {
    inside_abstract_class: bool,
}

impl ParseNodeList for ClassMembersList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: MSyntaxKind = M_CLASS_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_class_member(p, self.inside_abstract_class)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        // test_err js invalid_method_recover
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

// test js class_declare
// class B { declare() {} }
// class B { declare = foo }

fn parse_class_member(p: &mut MParser, inside_abstract_class: bool) -> ParsedSyntax {
    let member_marker = p.start();
    // test js class_empty_element
    // class foo { ;;;;;;;;;; get foo() {};;;;}
    if p.eat(T![;]) {
        return Present(member_marker.complete(p, M_EMPTY_CLASS_MEMBER));
    }

    let member = parse_class_member_impl(p, member_marker);

    member
}

fn parse_class_member_impl(p: &mut MParser, member_marker: Marker) -> ParsedSyntax {
    let start_token_pos = p.source().position();
    let generator_range = p.cur_range();

    // test js getter_class_member
    // class Getters {
    //   get foo() {}
    //   get static() {}
    //   static get bar() {}
    //   get "baz"() {}
    //   get ["a" + "b"]() {}
    //   get 5() {}
    //   get #private() {}
    // }
    // class NotGetters {
    //   get() {}
    //   async get() {}
    //   static get() {}
    // }
    //
    // test_err js method_getter_err
    // class foo {
    //  get {}
    // }
    //

    // test js setter_class_member
    // class Setters {
    //   set foo(a) {}
    //   set bax(a,) {}
    //   set static(a) {}
    //   static set bar(a) {}
    //   static set baz(a,) {}
    //   set "baz"(a) {}
    //   set ["a" + "b"](a) {}
    //   set 5(a) {}
    //   set 6(a,) {}
    //   set #private(a) {}
    // }
    // class NotSetters {
    //   set(a) {}
    //   async set(a) {}
    //   static set(a) {}
    // }
    //
    // test_err js setter_class_member
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

            let member_kind = expect_accessor_body(p, &member_marker);
            member_marker.complete(p, member_kind.as_getter_syntax_kind())
        } else {
            let has_l_paren = p.expect(T!['(']);
            p.with_state(EnterParameters(SignatureFlags::empty()), |p| {
                // test ts ts_decorator_on_class_setter { "parse_class_parameter_decorators": true }
                // class A {
                //     set val(@dec x) {}
                //     set val(@dec.fn() x) {}
                //     set val(@dec() x) {}
                // }

                // test_err ts ts_decorator_on_class_setter
                // class A {
                //     set val(@dec x) {}
                //     set val(@dec.fn() x) {}
                //     set val(@dec() x) {}
                // }
                parse_formal_parameter(
                    p,
                    ParameterContext::ClassSetter,
                    ExpressionContext::default().and_object_expression_allowed(has_l_paren),
                )
            })
            .or_add_diagnostic(p, m_parse_error::expected_parameter);

            if p.at(T![,]) {
                p.bump_any();
            }

            p.expect(T![')']);

            let member_kind = expect_accessor_body(p, &member_marker);
            member_marker.complete(p, member_kind.as_setter_syntax_kind())
        };

        return Present(completed);
    }

    let is_constructor = is_at_constructor(p);
    let member_name =
        parse_class_member_name(p).or_add_diagnostic(p, m_parse_error::expected_class_member_name);

    if is_at_method_class_member(p, 0) {
        // test js class_static_constructor_method
        // class B { static constructor() {} }

        // test js constructor_class_member
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
            // test js method_class_member
            // class Test {
            //   method() {}
            //   async asyncMethod() {}
            //   async* asyncGeneratorMethod() {}
            //   * generatorMethod() {}
            //   "foo"() {}
            //   ["foo" + "bar"]() {}
            //   5() {}
            //   #private() {}
            // }
            // class ContextualKeywords {
            //    // Methods called static
            //   static() {}
            //   async static() {}
            //   * static() {}
            //   async* static() {}
            //   declare() {}
            //   get() {} // Method called get
            //   set() {} // Method called set
            // }
            // class Static {
            //   static method() {}
            //   static async asyncMethod() {}
            //   static async* asyncGeneratorMethod() {}
            //   static * generatorMethod() {}
            //   static static() {}
            //   static async static() {}
            //   static async* static() {}
            //   static * static() {}
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
            // test js property_class_member
            // class foo {
            //   property
            //   declare;
            //   initializedProperty = "a"
            //   "a";
            //   5
            //   ["a" + "b"]
            //   static staticProperty
            //   static staticInitializedProperty = 1
            //   #private
            //   #privateInitialized = "a"
            //   static #staticPrivate
            //   static #staticPrivateInitializedProperty = 1
            // }
            //
            // test_err js class_declare_member
            // class B { declare foo }

            // test ts ts_property_class_member_can_be_named_set_or_get
            // class B { set: String; get: Number }
            // let mut property = parse_property_class_member_body(p, member_marker, modifiers);

            // if !property.kind(p).is_bogus() && is_constructor {
            //     let err = p.err_builder(
            //         "class properties may not be called `constructor`",
            //         property.range(p),
            //     );

            //     p.error(err);
            //     property.change_to_bogus(p);
            // }

            // Present(property)

            debug_assert_eq!(
                p.source().position(),
                start_token_pos,
                "Parser shouldn't be progressing when returning Absent"
            );
            Absent
        }
        None => {
            // test_err js block_stmt_in_class
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

fn expect_member_semi(p: &mut MParser, member_marker: &Marker, name: &str) {
    if !optional_semi(p) {
        // Gets the start of the member
        let end = p.last_end().unwrap_or_else(|| p.cur_range().start());

        let err = p.err_builder(
            format!("expected a semicolon to end the {name}, but found none"),
            member_marker.start()..end,
        );

        p.error(err);
    }
}

/// Eats the '?' token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut MParser) -> Result<Option<TextRange>, TextRange> {
    if p.at(T![?]) {
        let range = p.cur_range();
        p.bump(T![?]);

        // test_err js optional_member
        // class B { foo?; }

        let err = p.err_builder("`?` modifiers can only be used in TypeScript files", range);

        p.error(err);
        Err(range)
    } else {
        Ok(None)
    }
}

// test_err js class_property_initializer
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

fn parse_method_class_member(p: &mut MParser, m: Marker, flags: SignatureFlags) -> CompletedMarker {
    parse_class_member_name(p).or_add_diagnostic(p, m_parse_error::expected_class_member_name);
    parse_method_class_member_rest(p, m, flags)
}

// test_err js class_member_method_parameters
// class B { foo(a {} }

// test ts ts_method_class_member
// class Test {
//   test<A, B extends A, R>(a: A, b: B): R {}
// }

/// Parses the body (everything after the identifier name) of a method class member
/// that includes: parameters and its types, return type and method body
fn parse_method_class_member_rest(
    p: &mut MParser,
    m: Marker,
    flags: SignatureFlags,
) -> CompletedMarker {
    // test ts ts_optional_method_class_member
    // class A { test?() {} }
    let optional = optional_member_token(p);

    let parameter_context = ParameterContext::Declaration;

    parse_parameter_list(p, parameter_context, flags)
        .or_add_diagnostic(p, m_parse_error::expected_class_parameters);

    let member_kind = expect_method_body(p, &m, ClassMethodMemberKind::Method(flags));
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
    /// The body of methods is optional if it's a method overload definition
    /// ```ts
    /// class Test {
    ///   method();
    ///   method() { ... }
    /// }
    /// ```
    const fn is_body_optional(&self) -> bool {
        matches!(
            self,
            ClassMethodMemberKind::Method(_) | ClassMethodMemberKind::Constructor
        )
    }

    const fn is_constructor(&self) -> bool {
        matches!(self, ClassMethodMemberKind::Constructor)
    }

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
/// TypeScript supports method signatures. These are methods without a body (and are terminated by a semicolon or ASI).
/// A method is a signature if one of the following applies
/// * The member has an `abstract` modifier (not supported by constructors)
/// * It's a declaration in an ambient context (`declare class { ... }` or `declare namespace { class { ... } }`).
/// * It's a method overload (doesn't apply to getters/setters)
///
/// The method determines which case applies to the current member and emits a diagnostic if:
/// * the body is absent for a method declaration
/// * the body is present for a method signature
/// * a method signature isn't terminate by a semicolon or ASI
///
/// The method returns the inferred kind (signature or declaration) of the parsed method body
fn expect_method_body(
    p: &mut MParser,
    member_marker: &Marker,
    method_kind: ClassMethodMemberKind,
) -> MemberKind {
    let body = parse_function_body(p, method_kind.signature_flags());

    // test ts typescript_members_can_have_no_body_in_ambient_context
    // declare class Test {
    //     constructor();
    //     name();
    //     get test(): string;
    //     set test(v);
    // }
    // declare namespace n {
    //      class Test {
    //          constructor()
    //          name()
    //          get test(): string
    //          set test(v)
    //      }
    // }

    // test_err ts ts_method_members_with_missing_body
    // class Test {
    //      constructor() method() get test()
    //      set test(value)
    // }
    body.or_add_diagnostic(p, m_parse_error::expected_class_method_body);
    MemberKind::Declaration
}

// test_err js getter_class_no_body
// class Getters {
//   get foo()
// }

// test_err js setter_class_no_body
// class Setters {
//   set foo(a)
// }
fn expect_accessor_body(p: &mut MParser, member_marker: &Marker) -> MemberKind {
    expect_method_body(p, member_marker, ClassMethodMemberKind::Accessor)
}

fn parse_constructor_class_member_body(p: &mut MParser, member_marker: Marker) -> CompletedMarker {
    if let Ok(Some(range)) = optional_member_token(p) {
        let err = p.err_builder("constructors cannot be optional", range);

        p.error(err);
    }

    parse_constructor_parameter_list(p)
        .or_add_diagnostic(p, m_parse_error::expected_constructor_parameters);

    let constructor_kind =
        expect_method_body(p, &member_marker, ClassMethodMemberKind::Constructor);

    member_marker.complete(p, constructor_kind.as_constructor_syntax_kind())
}

fn parse_constructor_parameter_list(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();

    // test js super_expression_in_constructor_parameter_list
    // class A extends B { constructor(c = super()) {} }
    //
    // test_err js super_expression_in_constructor_parameter_list
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

// test_err js m_constructor_parameter_reserved_names
// // SCRIPT
// class A { constructor(readonly, private, protected, public) {} }
fn parse_constructor_parameter(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    parse_any_parameter(p, ParameterContext::ClassImplementation, context)
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

// test js static_generator_constructor_method
// class A {
// 	static async * constructor() {}
// 	static * constructor() {}
// }
fn is_at_constructor(p: &MParser) -> bool {
    p.at(T![constructor]) || matches!(p.cur_text(), "\"constructor\"" | "'constructor'")
}
