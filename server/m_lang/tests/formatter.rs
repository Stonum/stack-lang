use m_lang::formatter::{format_node, IndentStyle, IndentWidth, LineWidth, MFormatOptions};
use m_lang::parser::parse;
use m_lang::syntax::MFileSource;

macro_rules! assert_fmt {
    ($src:expr) => {
        let syntax = MFileSource::script();
        let tree = parse($src, syntax);

        let options = MFormatOptions::new(syntax)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_indent_width(IndentWidth::from(3));

        let doc = format_node(options, &tree.syntax());
        let result = doc.unwrap().print().unwrap();
        let result = result.as_code();
        assert_eq!($src, result, "formatted code: \n{}", result);
    };
}

#[test]
fn format_switch_with_multiple_case_options() {
    assert_fmt!(
        r#"#
switch( val )
{
   case "single":
      do_single();

   case "one", "two":
      do_single();

   case
      "long case",
      "another long case",
      "very long case",
      "very very very long case",
      "very very very long cas 123",
      "very very very very long case":
   {
      do_something();
   }
}
"#
    );
}

#[test]
fn format_query_like_expressions() {
    assert_fmt!(
        r#"#
var qq = Query(`
    select row_id from ~Лицевые договора~ where Договор = :1
`, 1, "p1,S");

var qq = Command(
   `update stack."Лицевые договора" set Договор = :1 where row_id = :2 and now() between ДатНач and ДатКнц`,
   1, "p1,S,p2,S"
);

var qq = сессия.Query(
   `select row_id from ~Лицевые договора~ where Договор = :1 and Лицевой = :2 and now() between ДатНач and ДатКнц`,
   1, "p1,S,p2,S"
);

var qq = Query(`select row_id from ~Лицевые договора~ `, 1, "p1,S");
"#
    );
}
