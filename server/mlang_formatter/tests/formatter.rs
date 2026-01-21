use mlang_formatter::{
    IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node, format_range,
};
use mlang_parser::parse;
use mlang_syntax::{MFileSource, TextRange, TextSize};

macro_rules! assert_fmt {
    ($src:expr) => {
        assert_fmt!($src, MFileSource::script());
    };

    ($src:expr, $file_type:expr) => {
        let syntax = $file_type;
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
    ($src:expr, $dest:expr, $range:expr) => {
        assert_fmt_range!($src, $dest, $range, MFileSource::script());
    };

    ($src:expr, $dest:expr, $range:expr, $file_type:expr) => {
        let syntax = $file_type;
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
fn format_report_section() {
    assert_fmt_range!(
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
        r#"print
{
   print("hey");
}"#,
        189..194, // section "print" name
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
fn format_class_with_annotations() {
    assert_fmt!(
        r#"#
:[Report(Name = "report name")]
class rep extends report
{
   :[test]
   constructor()
   {
      super();
   }

   :[method]
   met()
   {
      do_something();
   }

   m2()
   {
      do_something();
   }
}
"#
    );
}

#[test]
fn format_func_with_annotations() {
    assert_fmt!(
        r#"#
:[fun]
func do()
{
   do_something();
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

#[test]
fn format_inline_function_declaration() {
    assert_fmt!(
        r#"#
inline func i()
{
   return 1;
}
"#
    );
}

#[test]
fn format_if_statement_comments() {
    assert_fmt!(
        r#"#
if( test ) # ifcomment
{
}
else # elsecomment
{
   println(2);
}
"#
    );

    assert_fmt!(
        r#"#
if( test ) # ifcomment
   println(1);
else # else comment
   print(2);
"#
    );

    assert_fmt!(
        r#"#
# comment before if statement
if( test ) # ifcomment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before if statement
if( test ) # ifcomment
# more comments
   println(1);
"#
    );
}

#[test]
fn format_while_statement_comments() {
    assert_fmt!(
        r#"#
while( test ) # comment
{
   print(test);
}
"#
    );

    assert_fmt!(
        r#"#
while( test ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before while statement
while( test ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before while statement
while( test ) # ifcomment
# more comments
   println(1);
"#
    );
}

#[test]
fn format_for_statement_comments() {
    assert_fmt!(
        r#"#
for(;;) # comment
{
   print(test);
}
"#
    );

    assert_fmt!(
        r#"#
for(;;) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before for statement
for(;;) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before for statement
for(;;) # comment
# more comments
   println(1);
"#
    );
}

#[test]
fn format_forall_statement_comments() {
    assert_fmt!(
        r#"#
forall( iterator(arr, ind) ) # comment
{
   print(test);
}
"#
    );

    assert_fmt!(
        r#"#
forall( iterator(arr, ind) ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before for statement
forall( iterator(arr, ind) ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before forall statement
forall( iterator(arr, ind) ) # comment
# more comments
   println(1);
"#
    );
}

#[test]
fn format_forallin_statement_comments() {
    assert_fmt!(
        r#"#
forall( x in arr ) # comment
{
   print(test);
}
"#
    );

    assert_fmt!(
        r#"#
forall( x in arr ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before forall statement
forall( x in arr ) # comment
   println(1);
"#
    );

    assert_fmt!(
        r#"#
# comment before forall statement
forall( x in arr ) # comment
# more comments
   println(1);
"#
    );
}

#[test]
fn format_try_catch_statement() {
    assert_fmt!(
        r#"#
try
{
   do_something();
}
catch( e )
{
   log_exception(e);
}
"#
    );

    assert_fmt!(
        r#"#
try
{
   do_something();
}
catch( e ) {}
"#
    );
}

#[test]
fn format_instanceof_expressions() {
    assert_fmt!(
        r#"#
class S {}
class D extends S {}

var si = new S();
var di = new D();

si instanceof S; # true
di instanceof S; # true
"#
    );
}

#[test]
fn format_classof_expressions() {
    assert_fmt!(
        r#"#
class C {}
var x = new C();

classof x;

var isObject = true;
var str = isObject ? (classof x).name : typeof(x);
"#
    );
}
