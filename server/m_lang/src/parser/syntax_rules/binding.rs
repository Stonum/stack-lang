use super::expr::parse_identifier;

use super::syntax::MSyntaxKind::*;
use super::{MParser, ParsedSyntax};

pub(crate) fn parse_binding_pattern(p: &mut MParser) -> ParsedSyntax {
    parse_identifier_binding(p)
}

#[inline]
pub(crate) fn parse_binding(p: &mut MParser) -> ParsedSyntax {
    parse_identifier_binding(p)
}

pub(crate) fn parse_identifier_binding(p: &mut MParser) -> ParsedSyntax {
    parse_identifier(p, M_IDENTIFIER_BINDING)
}
