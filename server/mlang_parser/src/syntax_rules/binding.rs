use biome_parser::prelude::*;

use mlang_syntax::MSyntaxKind;

use super::expr::{parse_any_name, parse_identifier};
use biome_parser::prelude::ParsedSyntax::Present;

use super::m_parse_error::expected_identifier;
use mlang_syntax::MSyntaxKind::*;
use super::{MParser, ParsedSyntax};

pub(crate) fn parse_identifier_binding(p: &mut MParser) -> ParsedSyntax {
    parse_identifier(p, M_IDENTIFIER_BINDING)
}

pub(crate) fn parse_dot_binding(
    p: &mut MParser,
    lhs: CompletedMarker,
    operator: MSyntaxKind,
) -> ParsedSyntax {
    let m = lhs.precede(p);
    p.expect(operator);

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, M_EXTENDED_BINDING))
}
