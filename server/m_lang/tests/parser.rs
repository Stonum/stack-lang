use m_lang::{parser::parse, syntax::MFileSource};

#[test]
fn test_parse_function_declaration() {
    let res = parse(
        r#"
            func b() {}
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
            # TODO class a extends b {}
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
        "#,
        MFileSource::script(),
    );

    assert!(res.try_tree().is_some());
}

#[test]
fn test_parse() {
    let res = parse(
        r#"
            forall (value in @[1,2,3]) {
                println(x);
            }
        "#,
        MFileSource::script(),
    );

    dbg!(&res.syntax());
    dbg!(&res.diagnostics());
    assert!(res.try_tree().is_some());
}
