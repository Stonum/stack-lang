use m_lang::parser::parse;

#[test]
fn test_parse() {
    let res = parse(
        r#"
            #function b
            func b() {
                var ar = @[1,2,3];
                var ob = @{x: 1, y: 2};
                var s = @(x: 1, y: 2);
                return a() + ar;
            }
        "#,
    );
    dbg!(res);
}
