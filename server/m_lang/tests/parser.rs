use biome_rowan::{TextRange, TextSize, WalkEvent};
use m_lang::{parser::parse, syntax::MFileSource};

#[test]
fn test_parse_function_declaration() {
    let res = parse(
        r#"
            func b() {}
            func z(a, b, c = 10) {}
        "#,
        MFileSource::module(),
    );

    assert!(res.try_tree().is_some());
    assert!(!res.has_errors());
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

    assert!(res.try_tree().is_some());
    assert!(!res.has_errors());
}
#[test]
fn test_parse_expressions() {
    let res = parse(
        r#"
            var num = 1;
            x += 5 - 10;
            x++ - 5++;
            -x + (y - 5) * 6;
            var arr = @[1, null, "hello", 5.55, true, x];
            # TODO var set = @(1, null, "hello", 5.55, true, x);
            var obj = @{a: 1, b: null, c: "hello", d: 5.55, "e": true};
            var binary = 1 + 2.3 + "x";
            x == 5 && y == 10;
            params[10, 10];
            x = y < 3 ? 5 : 10;
            x.sum(x, 5)
        "#,
        MFileSource::script(),
    );

    assert!(res.try_tree().is_some());
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

    assert!(res.try_tree().is_some());
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

    assert!(res.try_tree().is_some());
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

    assert!(res.try_tree().is_some());
}

#[test]
fn test_parse_doc_string() {
    let res = parse(
        r#"
            class mega 
            `mega class documentation`
            {
                constructor() 
                `constructor docs`
                { 
                    this.a = 1; 
                }
            }
        "#,
        MFileSource::module(),
    );

    assert!(res.try_tree().is_some());
}
