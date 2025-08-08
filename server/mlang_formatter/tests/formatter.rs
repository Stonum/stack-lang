use mlang_formatter::{
    IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node, format_range,
};
use mlang_parser::parse;
use mlang_syntax::{MFileSource, TextRange, TextSize};

macro_rules! assert_fmt {
    ($src:expr $(, $file_type:expr)?) => {
        let syntax = MFileSource::script();
        $(
            let syntax = $file_type;
        )?
        let tree = parse($src, syntax);

        let options = MFormatOptions::new(syntax)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_indent_width(IndentWidth::from(3));

        let doc = format_node(options, &tree.syntax());
        let result = doc.unwrap().print().unwrap();
        let result = result.as_code();
        assert_eq!(
            $src, result,
            "formatted code: \n======\n{}\n======\n",
            result
        );
    };
}

macro_rules! assert_fmt_range {
    ($src:expr,$dest:expr,$range:expr) => {
        let syntax = MFileSource::script();
        let tree = parse($src, syntax);

        let options = MFormatOptions::new(syntax)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_indent_width(IndentWidth::from(3));

        let doc = format_range(
            options,
            &tree.syntax(),
            TextRange::new(TextSize::from($range.start), TextSize::from($range.end)),
        );
        let result = doc.unwrap().into_code();
        assert_eq!(
            $dest, result,
            "formatted code: \n======\n{}\n======\n",
            result
        );
    };
}

#[test]
fn format_range_if_statement_with_leading_comments() {
    assert_fmt_range!(
        r#"
var isCalc = shema;

# leading comment
if( true ) 
   isCalc = calc(_ls, _usl, _p);

var pars = _p;
    "#,
        r#"# leading comment
if( true )
   isCalc = calc(_ls, _usl, _p);"#,
        42..83
    );
}

#[test]
fn format_statement_with_trailing_comments() {
    assert_fmt!(
        r#"#
перем фЕстьНашеВложение =
   оВложение != null или
   Извлечь(_мВложение.ИдФайла, "") == "" или # не передан объект вложения
   Извлечь(_мВложение.ИдФайла, "").начинаетсяС("ON"); # старый идентификатор, он не совпадает с тем что в сбисе
"#
    );
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

#[test]
fn format_report() {
    assert_fmt!(
        r#"#
CommonReport
.CloseWindow = 1;
.Template = "tmp.xlsx";
.ReportFile = "rep.xlsx";
{
   var month = WorkMonth();
}
Function declaration
{
   func add( i )
   {
      return i++;
   }
}
print
{
   print("hey");
}
"#,
        MFileSource::report()
    );
}

#[test]
fn format_variable_declaration_list() {
    assert_fmt!(
        r#"#
var reactions = res = @{};
var x1 = x2 = x3 = x4 = 0;
x1 = x2 = x3 = x4 = 0;
"#
    );
}

#[test]
fn format_class() {
    assert_fmt!(
        r#"#
class foo
{
   constructor()
   {
      this._bar = null;
   }

   get bar()
   {
      return this._bar;
   }

   set bar(_val)
   {
      this._bar = _val;
   }

   compute()
   {
      return std.math.pow(this._bar, 2);
   }
}
"#
    );
}

#[test]
fn format_bit_operations() {
    assert_fmt!(
        r#"#
x & 4 == 0;
(a * 3) & 5;
"#
    );
}

#[test]
fn format_some_ru_keywords() {
    assert_fmt!(
        r#"#
перем х = нуль;
Если( истина )
   х = ложь;
"#
    );
}
