//! Implements the parsing logic for ES Module syntax

use super::m_parse_error::expected_statement;
use super::stmt::{parse_statement, StatementContext, STMT_RECOVERY_SET};

use super::syntax::{MSyntaxKind::*, T};
use super::{MParser, ParsedSyntax};

use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::*;
use biome_parser::ParserProgress;

// test js module
// import a from "b";
// export { a };
// c();
// import { c } from "c";
pub fn parse_module_body(p: &mut MParser, statement_list: Marker) {
    parse_module_item_list(p, ModuleItemListParent::Module, statement_list);
}

pub enum ModuleItemListParent {
    Module,
    Block,
}

impl ModuleItemListParent {
    fn is_module(&self) -> bool {
        matches!(self, ModuleItemListParent::Module)
    }

    #[inline]
    fn is_at_list_end(&self, p: &MParser) -> bool {
        if p.at(EOF) {
            return true;
        }

        match self {
            ModuleItemListParent::Block => p.at(T!['}']),
            _ => false,
        }
    }
}

pub fn parse_module_item_list(p: &mut MParser, parent: ModuleItemListParent, list_marker: Marker) {
    let mut progress = ParserProgress::default();

    let recovery_set = if parent.is_module() {
        STMT_RECOVERY_SET
    } else {
        // test_err ts module_closing_curly
        // declare module A {
        //  "name": "troublesome-lib",
        //  "typings": "lib/index.d.ts",
        //  "version": "0.0.1"
        // }

        // don't eat the closing `}` if inside a block
        STMT_RECOVERY_SET.union(token_set!(T!['}']))
    };

    while !parent.is_at_list_end(p) {
        progress.assert_progressing(p);

        let module_item = parse_module_item(p);

        let recovered = module_item.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_STATEMENT, recovery_set),
            expected_statement,
        );

        if recovered.is_err() {
            break;
        }
    }

    list_marker.complete(p, M_MODULE_ITEM_LIST);
}

fn parse_module_item(p: &mut MParser) -> ParsedSyntax {
    parse_statement(p, StatementContext::StatementList)
}
