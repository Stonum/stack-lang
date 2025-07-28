//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::report::parse_reports;
use super::stmt::{parse_directives, parse_statements};
use super::syntax::MSyntaxKind::*;
use super::syntax::ModuleKind;

use biome_parser::prelude::*;

use super::MParser;

pub(crate) fn parse(p: &mut MParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);

    if p.source_type().module_kind().is_module() {
        parse_directives(p);
    }

    let statement_list = p.start();

    match p.source_type().module_kind() {
        ModuleKind::Script => {
            parse_statements(p, false, statement_list);
            m.complete(p, M_SCRIPT)
        }
        ModuleKind::Module | ModuleKind::Handler => {
            parse_module_body(p, statement_list);
            m.complete(p, M_MODULE)
        }
        ModuleKind::Report => {
            parse_reports(p, statement_list);
            m.complete(p, M_REPORT_FILE)
        }
    }
}
