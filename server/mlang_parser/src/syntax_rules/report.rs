use mlang_syntax::MSyntaxKind;

use super::expr::{is_at_expression, parse_name};
use super::m_parse_error::{
    expected_binding, expected_block_statement, expected_identifier, expected_statement,
};
use super::stmt::{
    STMT_RECOVERY_SET, parse_block_impl, parse_expression_statement, parse_global_statement,
};
use super::{Absent, MParser, ParsedSyntax, Present};
use mlang_syntax::{MSyntaxKind::*, T};

use biome_parser::ParserProgress;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::*;
use biome_rowan::TextRange;

const REPORT_TOKEN_SET: TokenSet<MSyntaxKind> = token_set!(T![ff2], T![ff], T!['{'], T![EOF]);

const REPORT_RECOVERY_SET: TokenSet<MSyntaxKind> = STMT_RECOVERY_SET.union(REPORT_TOKEN_SET);

pub fn parse_reports(p: &mut MParser, list_marker: Marker) {
    let mut progress = ParserProgress::default();

    while !p.at(EOF) {
        progress.assert_progressing(p);

        let report = parse_report(p);

        let recovered = report.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_STATEMENT, REPORT_RECOVERY_SET),
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
        p.at_ts(REPORT_TOKEN_SET)
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS, REPORT_RECOVERY_SET)
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
            &ParseRecoveryTokenSet::new(M_BOGUS, REPORT_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_block_statement,
        )
    }
}

fn parse_report_section(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![ff]) {
        return Absent;
    }

    let m = p.start();

    parse_section_name(p).or_add_diagnostic(p, expected_identifier);

    // change to bogus all tokens after section name, if it is not a body, report and another section
    if !p.at_ts(REPORT_TOKEN_SET) {
        let mut progress = ParserProgress::default();
        let bogus = p.start();
        let range = p.cur_range();

        while !p.at_ts(REPORT_TOKEN_SET) {
            progress.assert_progressing(p);
            p.bump_any();
        }
        bogus.complete(p, M_BOGUS);
        p.error(p.err_builder(
            "Expected block statement, report section or report",
            TextRange::new(range.start(), p.cur_range().end()),
        ));
    }

    parse_block_impl(p, M_BLOCK_STATEMENT).or_add_diagnostic(p, expected_block_statement);

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
