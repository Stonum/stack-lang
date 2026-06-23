use super::stmt::parse_statements;
use psql_syntax::PsqlSyntaxKind::*;

use biome_parser::prelude::*;

use crate::PsqlParser;

pub(crate) fn parse(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);

    let statement_list = p.start();

    parse_statements(p, statement_list);
    m.complete(p, PSQL_ROOT)
}
