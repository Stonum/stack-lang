use super::binding::parse_binding;
use super::class::parse_class_declaration;
use super::expr::parse_literal_expression;
use super::function::parse_function_declaration;
use super::m_parse_error::{expected_binding, expected_identifier, expected_literal_expression};
use super::stmt::{StatementContext, STMT_RECOVERY_SET};
use super::syntax::{MSyntaxKind::*, T, *};
use super::{Absent, MParser, Present};

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::*;
use biome_parser::ParserProgress;

pub(crate) fn parse_annotation_statement(
    p: &mut MParser,
    context: StatementContext,
) -> ParsedSyntax {
    if !p.at(T![:]) || !p.nth_at(1, T!['[']) {
        return Absent;
    }

    let annotation = parse_annotation_list(p);

    match p.cur() {
        // function
        T![function] => parse_function_declaration(p, context, Some(annotation)),

        // class
        T![class] => parse_class_declaration(p, context, Some(annotation)),
        _ => Absent,
    }
}

pub(crate) fn parse_annotation_list(p: &mut MParser) -> CompletedMarker {
    let mut progress = ParserProgress::default();

    let list = p.start();
    while !p.at(EOF) {
        progress.assert_progressing(p);

        if !p.at(T![:]) || !p.nth_at(1, T!['[']) {
            break;
        }

        let m = p.start();
        p.bump(T![:]);
        p.bump(T!['[']);

        AnnotationList.parse_list(p);

        p.expect(T![']']);
        m.complete(p, M_ANNOTATION_GROUP);
    }

    list.complete(p, M_ANNOTATION_GROUP_LIST)
}

struct AnnotationList;
impl ParseSeparatedList for AnnotationList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_ANNOTATION_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_annotation(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![']'])
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

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn parse_annotation(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();

    parse_binding(p).or_add_diagnostic(p, expected_identifier);

    let kind = if p.at(T!['(']) {
        p.bump(T!['(']);
        AnnotationAttributesList.parse_list(p);
        p.expect(T![')']);
        M_ANNOTATION_ELEMENT
    } else {
        M_ANNOTATION_BINDING
    };

    Present(m.complete(p, kind))
}

struct AnnotationAttributesList;
impl ParseSeparatedList for AnnotationAttributesList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_ANNOTATION_ATTRIBUTE_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_annotation_attribute(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![')'])
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

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline]
fn parse_annotation_attribute(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_binding(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T![=]);
    parse_literal_expression(p).or_add_diagnostic(p, expected_literal_expression);

    Present(m.complete(p, M_ANNOTATION_ATTRIBUTE))
}
