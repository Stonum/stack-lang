use super::expr::{is_nth_at_identifier, parse_identifier, ExpressionContext};

use super::syntax::{MSyntaxKind::*, *};
use super::{MParser, ParsedSyntax};

use biome_parser::prelude::*;
use biome_rowan::SyntaxKind as SyntaxKindTrait;

pub(super) fn parse_binding_pattern(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    match p.cur() {
        _ => parse_identifier_binding(p),
    }
}

#[inline]
pub(super) fn is_at_identifier_binding(p: &mut MParser) -> bool {
    is_nth_at_identifier_binding(p, 0)
}

#[inline]
pub(super) fn is_nth_at_identifier_binding(p: &mut MParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
}

#[inline]
pub(super) fn parse_binding(p: &mut MParser) -> ParsedSyntax {
    parse_identifier_binding(p)
}

// test_err js binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
// let let = 5;
// const let = 5;
// let a, a;
//
// test_err js binding_identifier_invalid_script
// // SCRIPT
// let let = 5;
// const let = 5;
/// Parses an identifier binding or returns an invalid syntax if the identifier isn't valid in this context.
/// An identifier may not be valid if:
/// * it is named "eval" or "arguments" inside of strict mode
/// * it is named "let" inside of a "let" or "const" declaration
/// * the same identifier is bound multiple times inside of a `let` or const` declaration
/// * it is named "yield" inside of a generator function or in strict mode
/// * it is named "await" inside of an async function
pub(super) fn parse_identifier_binding(p: &mut MParser) -> ParsedSyntax {
    let parsed = parse_identifier(p, M_IDENTIFIER_BINDING);

    parsed.map(|mut identifier| {
        if identifier.kind(p).is_bogus() {
            return identifier;
        }

        let identifier_name = identifier.text(p);
        /*
               if let Some(parent) = p.state().duplicate_binding_parent {

                   if let Some(existing) = p.state().name_map.get(identifier_name) {
                       let err = p
                           .err_builder(
                               format!(
                                   "Declarations inside of a `{parent}` declaration may not have duplicates"
                               ),
                               identifier.range(p),
                           )
                           .with_detail(
                               identifier.range(p),
                               format!(
                                   "a second declaration of `{identifier_name}` is not allowed"
                               ),
                           )
                           .with_detail(
                               *existing,
                               format!("`{identifier_name}` is first declared here"),
                           );
                       p.error(err);
                       identifier.change_to_bogus(p);
                       return identifier;
                   }

                   let identifier_name = String::from(identifier_name);
                   let identifier_range = identifier.range(p);
                   p.state_mut()
                       .name_map
                       .insert(identifier_name, identifier_range.as_range());
               }
        */
        identifier
    })
}
