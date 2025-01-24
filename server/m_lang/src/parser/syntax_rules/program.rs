//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::stmt::parse_statements;
use super::syntax::MSyntaxKind::*;
use super::syntax::ModuleKind;

use biome_parser::prelude::*;

// use super::stmt::parse_directives;
use super::MParser;

pub(crate) fn parse(p: &mut MParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);
    // p.eat(M_SHEBANG);

    let statement_list = p.start();

    let result = match p.source_type().module_kind() {
        ModuleKind::Script => {
            parse_statements(p, false, statement_list);
            m.complete(p, M_SCRIPT)
        }
        ModuleKind::Module => {
            parse_module_body(p, statement_list);
            m.complete(p, M_MODULE)
        }
        ModuleKind::Report => todo!(),
    };

    result
}
