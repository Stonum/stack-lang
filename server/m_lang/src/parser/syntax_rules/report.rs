use crate::syntax::MSyntaxKind;

use super::expr::{is_at_expression, parse_name};
use super::m_parse_error::{
    expected_binding, expected_block_statement, expected_identifier, expected_statement,
};
use super::stmt::{
    parse_block_impl, parse_expression_statement, parse_global_statement, STMT_RECOVERY_SET,
};
use super::syntax::{MSyntaxKind::*, T};
use super::{Absent, MParser, ParsedSyntax, Present};

use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::*;
use biome_parser::ParserProgress;

pub fn parse_reports(p: &mut MParser, list_marker: Marker) {
    let mut progress = ParserProgress::default();

    let recovery_set = STMT_RECOVERY_SET.union(token_set!(T![ff2]));
    while !p.at(EOF) {
        progress.assert_progressing(p);

        let report = parse_report(p);

        let recovered = report.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_STATEMENT, recovery_set),
            expected_statement,
        );

        if recovered.is_err() {
            break;
        }
    }

    list_marker.complete(p, M_REPORT_LIST);
}

fn parse_report(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![ff2]) {
        return Absent;
    }

    let report = p.start();

    let _name = parse_report_name(p);
    ReportAssignmentList.parse_list(p);

    let _body = parse_block_impl(p, M_BLOCK_STATEMENT);

    ReportSectionList.parse_list(p);

    Present(report.complete(p, M_REPORT))
}

fn parse_report_name(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![ff2]) {
        return Absent;
    }

    let m = p.start();

    p.expect(T![ff2]);

    parse_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, M_REPORT_NAME))
}

struct ReportAssignmentList;
impl ParseNodeList for ReportAssignmentList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: MSyntaxKind = M_REPORT_INIT_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        match p.cur() {
            T![.] => parse_global_statement(p),
            _ if is_at_expression(p) => parse_expression_statement(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['{']) | p.at(T![ff2]) | p.at(T![ff]) | p.at(EOF)
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS, STMT_RECOVERY_SET.union(token_set!(T![ff2])))
                .enable_recovery_on_line_break(),
            expected_binding,
        )
    }
}

struct ReportSectionList;
impl ParseNodeList for ReportSectionList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: MSyntaxKind = M_REPORT_SECTION_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        parse_report_section(p)
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![ff2]) | p.at(EOF)
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS, STMT_RECOVERY_SET.union(token_set!(T![ff])))
                .enable_recovery_on_line_break(),
            expected_block_statement,
        )
    }
}

fn parse_report_section(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();

    let _name = parse_section_name(p);
    let _body = parse_block_impl(p, M_BLOCK_STATEMENT);

    Present(m.complete(p, M_REPORT_SECTION))
}

fn parse_section_name(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![ff]) {
        return Absent;
    }

    let m = p.start();

    p.expect(T![ff]);

    parse_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, M_REPORT_SECTION_NAME))
}
