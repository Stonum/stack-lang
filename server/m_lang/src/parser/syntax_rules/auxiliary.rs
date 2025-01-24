use super::class::parse_class_declaration;
use super::function::parse_function_declaration;
use super::stmt::{parse_variable_declaration, semi, StatementContext, VariableDeclarationParent};
use super::syntax::MSyntaxKind::M_VARIABLE_DECLARATION_CLAUSE;
use super::syntax::{TextRange, TextSize, T};
use super::{Absent, MParser, ParsedSyntax};

use biome_parser::prelude::*;

// test js export_variable_clause
// export let a;
// export const b = 3;
// export var c, d, e = 3;
//
// test_err js export_variable_clause_error
// export let a = ;
// export const b;
// export let d, c;
pub fn parse_variable_declaration_clause(p: &mut MParser) -> ParsedSyntax {
    let start = p.cur_range().start();

    parse_variable_declaration(p, VariableDeclarationParent::Clause).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, TextRange::new(start, p.cur_range().end()));
        m.complete(p, M_VARIABLE_DECLARATION_CLAUSE)
    })
}

pub fn parse_declaration_clause(p: &mut MParser, stmt_start_pos: TextSize) -> ParsedSyntax {
    match p.cur() {
        T![function] => parse_function_declaration(p, StatementContext::StatementList),
        T![class] => parse_class_declaration(p, StatementContext::StatementList),
        // test ts ts_ambient_var_statement
        // declare var a, b, c;
        T![var] => parse_variable_declaration_clause(p),

        _ => Absent,
    }
}
