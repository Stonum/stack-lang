use m_lang::{
    parser::parse,
    syntax::MFileSource,
};

macro_rules! assert_parser {
    ($res:expr) => {
        assert!($res.try_tree().is_some());
        assert!(!$res.has_errors());
    };
}

#[test]
fn test_parse_function_declaration() {
    let res = parse(
        r#"
            func b() {}
            func z(a, b, c = 10) {}
            func z(a, b, ... ) {}
            func z(a, b, ...param ) {}
        "#,
        MFileSource::module(),
    );

    assert_parser!(res);
}
#[test]
fn test_parse_class_declaration() {
    let res = parse(
        r#"
            class a {
                constructor() {
                    var _b = 1; # inline comment
                }

                get b() { return this._b; }
                set b(_val) { this._b = _val }

                print() {
                    println(this._b);
                }
            }
            class b extends a {
                constructor() {
                    super();
                }
            }
        "#,
        MFileSource::module(),
    );

    assert_parser!(res);
}
#[test]
fn test_parse_expressions() {
    let res = parse(
        r#"
            var num = 1;
            x += 5 - 10;
            x++ - y++;
            -x + (y - 5) * 6;
            var arr = @[1, null, "hello", 5.55, true, x];
            var set = set(1, null, "hello", 5.55, true, x);
            var obj = @{a: 1, b: null, c: "hello", d: 5.55, "e": true};
            var map = @(a: 1, b: null, c: "hello", d: 5.55, "e": true);
            var binary = 1 + 2.3 + "x";
            x == 5 && y == 10;
            params[10, 10];
            x = y < 3 ? 5 : 10;
            x.sum(x, 5)
            .x = 10;
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parse_loop() {
    let res = parse(
        r#"
            for (var i = 0; i < 10; i++) {
                println(i);
            }
            forall (var x in @[1,2,3]) {
                println(x);
            }
            while(x < 10) {
                println(x);
                x++;
            }
            forall (factory(@[1,2,3], x)) {
                println(x);
            }
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parse_condition() {
    let res = parse(
        r#"
            if (x == 1) {
                println(x);
            } else if (x == 2) {
                println(x);
            } else {
                println(x);
            }
            var x = z == 1 ? 1 : 2;
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);

    let res = parse(
        r#"
            if (x == 1 || x == 2)
                println(x);
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);

    let res = parse(
        r#"
            if (x == 1 or x == 2)
                println(x);
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);
}

#[test]
fn test_parse_switch_condition() {
    let res = parse(
        r#"
            switch (x) {
                case 1: println(x);
                case 2: println(x);
                else println(x);
            }
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);

    let res = parse(
        r#"
            ВыборПо(x) {
                выбор 1: Сообщить(x);
                выбор 2: Сообщить(x);
                иначе Сообщить(x);
            }
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);
}

#[test]
fn test_parse_strings_with_keyword() {
    let res = parse(
        r#"
           var short_string = "short string with class and class" ;
           var long_string = `very very long string with class and class`;
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_parse_doc_string() {
    let res = parse(
        r#"
            func mega_func() 
            `mega function documentation`
            {
                return 123;
            }

            class mega 
            `mega class documentation`
            {
                constructor() 
                `constructor 
                 multiline 
                 docs`
                { 
                    this.a = 1; 
                }
            }
        "#,
        MFileSource::module(),
    );

    assert_parser!(res);
}

#[test]
fn test_computed_fields() {
    let res = parse(
        r#"
            a[x] = 10;
            a["x"] = 20;
            f[a,b,c] = 30;
            f["a","b","c"] = 40;
            f["a", b, "c"] = f[a,b,c];
            
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_computed_call() {
    let res = parse(
        r#"
            var name = "function_name";
            var z = typeof([name]);
            var y = ["function_name"]();
            ["object name"] = new ["class_name"]();
            ["object name"]["method name"]('.');
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_keyword_as_identifier() {
    let res = parse(
        r#"
            в = 1;
            перем в = 2;
            в.поле = в;
        "#,
        MFileSource::script(),
    );
    assert_parser!(res);
}

#[test]
fn test_global_identifier() {
    let res = parse(
        r#"
            Выполнить( . );
            Выполнить( .в );
            .в = 1;
            перем в = .x;
            .в.поле = x;
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_static_member_expression() {
    let res = parse(
        r#"
            x = таблица.поле;
            y = таблица.1;
            z = таблица.1.поле;
            таблица.поле = z;
            таблица.3 = z;
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_date_and_time_literals() {
    let res = parse(
        r#"
            var date = 01.01.2001;
            if( date < 09.05.2045 ) {
                println(date);
            }
            var time = 10:55:55;
            if( time > 00:00:00 ) {
                println(time);
            }
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);

    let res = parse(
        r#"
            var obj = @{
                1:10,
                10:55,
                10:null
            }
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_constant_expression() {
    let res = parse(
        r#"
            перем стр = к"Строка";
            перем к = 1;
            перем стр = к;
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_directives() {
    let res = parse(
        r#"
           version 2

           func b() {
              var version = 123;
              return version && 2;
           }
        "#,
        MFileSource::module(),
    );

    assert_parser!(res);
}

#[test]
fn test_annotations() {
    let res = parse(
        r#"
            :[test]
            func f() {}

            :[test(order = 7, description = "some test", disabled = true)]
            class C {}

            :[test()]
            func f() {}

            :[test(disabled = true,)]
            func f() {}

            :[test, disabled]
            func f() {}

            :[test]
            :[disabled]
            func f() {}

            class C
            {
                :[ctor]
                constructor () { }
                :[getter]
                get x() { return 0 }
                :[setter]
                set x(v) { }
                :[method]
                method() { }
            }
        "#,
        MFileSource::script(),
    );

    assert_parser!(res);
}
