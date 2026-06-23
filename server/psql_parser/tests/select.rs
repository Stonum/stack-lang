#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::PsqlFileSource;

#[test]
fn test_select_literal_list() {
    let res = parse(
        "select *, 1, 2, true, false, 'str', 1.15",
        PsqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_select_asliased_literals() {
    let res = parse(
        "select 1 as x, 2 as y, 3 z, 4 \"as\"",
        PsqlFileSource::script(),
    );

    assert_parser!(res, debug);
}
