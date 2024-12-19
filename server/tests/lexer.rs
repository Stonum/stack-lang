use stack_lang_server::{lexer_v2::*, T};

/// walks `$tokens` and compares them to the given kinds.
macro_rules! assert_tokens {
    ($tokens:ident, [$($kind:expr,)*]) => {
        {
            let mut it = $tokens.iter();
            $(
                let token = it.next().expect("not enough tokens");
                assert_eq!(token.kind, $kind);
            )*
        }
    };
}

#[test]
fn single_char_tokens() {
    let input = "+-(.):";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(
        tokens,
        [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]
    );
}

#[test]
fn with_trivia_tokens() {
    let input = r#"
        @[
            1,
            null,

            # comment
            x
        ]"#;
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    #[rustfmt::skip]
    assert_tokens!(tokens, [
        T![newline], 
        T![ws], T![@], T!['['], T![newline], 
        T![ws], T![num], T![,], T![newline], 
        T![ws], T![null], T![,], T![newline], 
        T![ws], T![comment], T![newline], 
        T![ws], T![ident], T![newline],
        T![ws], T![']'],
        T![EOF],
    ]);
}

#[test]
fn source_text_tokens() {
    let input = "x = 1 # comment";
    let lexer = Lexer::new(input);
    let strings: Vec<&str> = lexer.map(|token| token.text(&input)).collect();
    assert_eq!(strings, vec!["x", " ", "=", " ", "1", " ", "# comment", ""]);
}
