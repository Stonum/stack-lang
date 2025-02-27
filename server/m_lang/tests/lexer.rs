#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use m_lang::lexer::{MLexContext, MLexer};

use m_lang::parser::span::Span;
use m_lang::syntax::MSyntaxKind::{self, EOF};
use m_lang::syntax::MSyntaxKind::{M_NUMBER_LITERAL, NEWLINE, WHITESPACE};
use m_lang::syntax::T;

use biome_parser::lexer::{BufferedLexer, Lexer};
use biome_rowan::TextRange;
use biome_rowan::TextSize;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = MLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(MLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                m_lang::syntax::MSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0,
            );

            assert_eq!(
                tokens[idx].1.len(),
                TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].1.len(),
                tokens[idx].0,
            );

            new_str.push_str(&$src[tokens[idx].1.as_range()]);
            tok_idx += tokens[idx].1.len();

            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].0
            );
        } else {
            assert_eq!(idx, tokens.len());
        }

        assert_eq!($src, new_str, "Failed to reconstruct input");
    }};
}

#[test]
fn empty() {
    assert_lex! {
        "",
    }
}

#[test]
fn identifier() {
    assert_lex! {
        "Abcdefg",
        IDENT:7
    }
}

#[test]
fn identifier_with_at() {
    assert_lex! {
        "@Дата",
        IDENT:9
    }
    assert_lex! {
        "Дата@",
        IDENT:9
    }
    assert_lex! {
        "Д@т@",
        IDENT:6
    }
}

#[test]
fn identifier_with_single_quotes() {
    assert_lex! {
        "'Список лицевых'",
        IDENT:29
    }
    assert_lex! {
        "'Тип-Договор'",
        IDENT:23
    }
    assert_lex! {
        "'оВыборка.Тип'",
        IDENT:25
    }
    assert_lex! {
        "'оВыборка.Тип-Договор'",
        IDENT:40
    }
    assert_lex! {
        "'оВыборка.Тип>Договор'",
        IDENT:40
    }
}

#[test]
fn identifier_with_keyword() {
    assert_lex! {
        "SuperClass",
        IDENT:10
    }
    assert_lex! {
        "БазовыйКласс",
        IDENT:24
    }
}

#[test]
fn punctuators() {
    assert_lex! {
        "!%%&()*+,-.:;<=>?[]^{}|~",
        BANG:1,
        PERCENT:1,
        PERCENT:1,
        AMP:1,
        L_PAREN:1,
        R_PAREN:1,
        STAR:1,
        PLUS:1,
        COMMA:1,
        MINUS:1,
        DOT:1,
        COLON:1,
        SEMICOLON:1,
        LTEQ:2,
        R_ANGLE:1,
        QUESTION:1,
        L_BRACK:1,
        R_BRACK:1,
        CARET:1,
        L_CURLY:1,
        R_CURLY:1,
        PIPE:1,
        TILDE:1,
    }
}

#[test]
fn bang() {
    assert_lex!(
        r#"!/a/"#,
        BANG:1,
        SLASH:1,
        IDENT:1,
        SLASH:1
    );

    assert_lex!(
        r#"a!/a/"#,
        IDENT:1,
        BANG:1,
        SLASH:1,
        IDENT:1,
        SLASH:1
    );

    assert_lex!(
        "1 && !2",
        M_NUMBER_LITERAL:1,
        WHITESPACE:1,
        AMP2:2,
        WHITESPACE:1,
        BANG:1
        M_NUMBER_LITERAL:1
    );
}

#[test]
fn consecutive_punctuators() {
    assert_lex! {
        "&&&&^^^||",
        AMP2:2,
        AMP2:2,
        CARET:1,
        CARET:1,
        CARET:1,
        PIPE2:2,
    }
}

#[test]
fn unicode_whitespace() {
    assert_lex! {
        " \u{00a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}",
        WHITESPACE:48
    }
}

#[test]
fn unicode_whitespace_ident_part() {
    assert_lex! {
        "Abcd\u{2006}",
        IDENT:4,
        WHITESPACE:3 // length is in bytes
    }
}

#[test]
fn all_whitespace() {
    assert_lex! {
        "\n\t\t",
        NEWLINE:1
        WHITESPACE:2
    }
    assert_lex! {
        "\r\n\t\t",
        NEWLINE:2
        WHITESPACE:2
    }
    assert_lex! {
        "\n\n",
        NEWLINE:1
        NEWLINE:1
    }
    assert_lex! {
        "\r\n\r\n",
        NEWLINE:2
        NEWLINE:2
    }
    assert_lex! {
        "\r\r\r\r",
        NEWLINE:1
        NEWLINE:1
        NEWLINE:1
        NEWLINE:1
    }
    assert_lex! {
        "\r\r\n\n\u{2028}\u{2029}",
        NEWLINE:1
        NEWLINE:2
        NEWLINE:1
        NEWLINE:3
        NEWLINE:3
    }
}

#[test]
fn empty_string() {
    assert_lex! {
        r#""""#,
        M_STRING_LITERAL:2
    }
    assert_lex! {
        r#"``"#,
        M_LONG_STRING_LITERAL:2
    }
}

#[test]
fn simple_string() {
    assert_lex! {
        r#""abcdefghijklmnopqrstuvwxyz123456789\"10🦀""#,
        M_STRING_LITERAL:45
    }
    assert_lex! {
        r#"`abcdefghijklmnopqrstuvwxyz123456789\`10🦀`"#,
        M_LONG_STRING_LITERAL:45
    }
}

#[test]
fn multiline_long_string() {
    assert_lex! {
        r#"`abcdef
            ghijklmnopqrst
            uvwxyz123456789\`10🦀`"#,
        M_LONG_STRING_LITERAL:71
    }
}

#[test]
fn string_unicode_escape_invalid() {
    assert_lex! {
        r#""abcd\u21""#,
        ERROR_TOKEN:10
    }

    assert_lex! {
        r"`abcd\u21`",
        ERROR_TOKEN:10
    }
}

#[test]
fn string_unicode_escape_valid() {
    assert_lex! {
        r#""abcd\u2000a""#,
        M_STRING_LITERAL:13
    }

    assert_lex! {
        r"`abcd\u2000a`",
        M_LONG_STRING_LITERAL:13
    }
}

#[test]
fn string_unicode_escape_surrogates() {
    assert_lex! {
        r#""\uD83D\uDCA9""#,
        M_STRING_LITERAL:14
    }

    assert_lex! {
        r#""\uD83D""#,
        M_STRING_LITERAL:8
    }
}

#[test]
fn string_unicode_escape_valid_resolving_to_endquote() {
    assert_lex! {
        r#""abcd\u0022a""#,
        M_STRING_LITERAL:13
    }

    assert_lex! {
        r"`abcd\u0027a`",
        M_LONG_STRING_LITERAL:13
    }
}

#[test]
fn string_hex_escape_invalid() {
    assert_lex! {
        r#""abcd \xZ0 \xGH""#,
        ERROR_TOKEN:16
    }

    assert_lex! {
        r"`abcd \xZ0 \xGH`",
        ERROR_TOKEN:16
    }
}

#[test]
fn string_hex_escape_valid() {
    assert_lex! {
        r#""abcd \x00 \xAB""#,
        M_STRING_LITERAL:16
    }

    assert_lex! {
        r"`abcd \x00 \xAB`",
        M_LONG_STRING_LITERAL:16
    }
}

#[test]
fn unterminated_string() {
    assert_lex! {
        r#""abcd"#,
        ERROR_TOKEN:5
    }

    assert_lex! {
        r#"`abcd"#,
        ERROR_TOKEN:5
    }
}

#[test]
fn string_all_escapes() {
    assert_lex! {
        r#""\x\u2004\u20\ux\xNN""#,
        ERROR_TOKEN:21
    }

    assert_lex! {
        r"`\x\u2004\u20\ux\xNN`",
        ERROR_TOKEN:21
    }
}

#[test]
fn complex_string_1() {
    assert_lex! {
        r#" _this += "str'n\u200bg";"#,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        PLUSEQ:2,
        WHITESPACE:1,
        M_STRING_LITERAL:14,
        SEMICOLON:1
    }

    assert_lex! {
        r#" _this += `str"n\u200bg`;"#,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        PLUSEQ:2,
        WHITESPACE:1,
        M_LONG_STRING_LITERAL:14,
        SEMICOLON:1
    }
}

#[test]
fn unterminated_string_length() {
    assert_lex! {
        "`abc",
        ERROR_TOKEN:4
    }
}

#[test]
fn unterminated_ident_with_escape_len() {
    assert_lex! {
        "'abc\\",
        ERROR_TOKEN:5
    }

    assert_lex! {
        r"'abc\x",
        ERROR_TOKEN:6
    }

    assert_lex! {
        r"'abc\x4",
        ERROR_TOKEN:7
    }

    assert_lex! {
        r"'abc\x45",
        ERROR_TOKEN:8
    }

    assert_lex! {
        r"'abc\u",
        ERROR_TOKEN:6
    }

    assert_lex! {
        r"'abc\u20",
        ERROR_TOKEN:8
    }
}

#[test]
fn dollarsign_underscore_idents() {
    assert_lex! {
        "$a",
        IDENT:2
    }
}

#[test]
fn labels_b() {
    assert_lex! {
        "break",
        BREAK_KW:5
    }

    assert_lex! {
        "breaking speed records",
        IDENT:8,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        IDENT:7
    }
}

#[test]
fn labels_c() {
    assert_lex! {
        "continue, class, catch, case",
        CONTINUE_KW:8,
        COMMA:1,
        WHITESPACE:1,
        CLASS_KW:5,
        COMMA:1,
        WHITESPACE:1,
        CATCH_KW:5,
        COMMA:1,
        WHITESPACE:1,
        CASE_KW:4
    }

    assert_lex! {
        "classy crabs",
        IDENT:6,
        WHITESPACE:1,
        IDENT:5
    }
}

#[test]
fn labels_d() {
    assert_lex! {
        "debug delete",
        DEBUG_KW:5,
        WHITESPACE:1,
        DELETE_KW:6,
    }

    assert_lex! {
        "derive doot d",
        IDENT:6,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        IDENT:1
    }
}

#[test]
fn labels_e() {
    assert_lex! {
        "else ",
        ELSE_KW:4,
        WHITESPACE:1,
    }

    assert_lex! {
        "e exports elsey",
        IDENT:1,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IDENT:5
    }
}

#[test]
fn labels_f() {
    assert_lex! {
        "finally for func",
        FINALLY_KW:7,
        WHITESPACE:1,
        FOR_KW:3,
        WHITESPACE:1,
        FUNCTION_KW:4
    }

    assert_lex! {
        "finally, foreign food!",
        FINALLY_KW:7,
        COMMA:1,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IDENT:4,
        BANG:1
    }
}

#[test]
fn labels_i() {
    assert_lex! {
        "i in if",
        IDENT:1,
        WHITESPACE:1,
        IN_KW: 2,
        WHITESPACE:1,
        IF_KW:2,
    }

    assert_lex! {
        "icecream interesting, innit?",
        IDENT:8,
        WHITESPACE:1,
        IDENT:11,
        COMMA:1,
        WHITESPACE:1,
        IDENT:5,
        QUESTION:1
    }
}

#[test]
fn labels_n() {
    assert_lex! {
        "new",
        NEW_KW:3
    }

    assert_lex! {
        "newly n",
        IDENT:5,
        WHITESPACE:1,
        IDENT:1
    }
}

#[test]
fn labels_r() {
    assert_lex! {
        "return",
        RETURN_KW:6
    }

    assert_lex! {
        "returning",
        IDENT:9
    }
}

#[test]
fn labels_s() {
    assert_lex! {
        "switch super",
        SWITCH_KW:6,
        WHITESPACE:1,
        SUPER_KW:5
    }

    assert_lex! {
        "superb switching",
        IDENT:6,
        WHITESPACE:1,
        IDENT:9
    }
}

#[test]
fn labels_t() {
    assert_lex! {
        "this try throw t",
        THIS_KW:4,
        WHITESPACE:1,
        TRY_KW:3,
        WHITESPACE:1,
        THROW_KW:5,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "thistle throwing tea",
        IDENT:7,
        WHITESPACE:1,
        IDENT:8,
        WHITESPACE:1,
        IDENT:3
    }
}

#[test]
fn labels_v() {
    assert_lex! {
        "var v",
        VAR_KW:3,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "variable voiding bad",
        IDENT:8,
        WHITESPACE:1,
        IDENT:7,
        WHITESPACE:1,
        IDENT:3
    }
}

#[test]
fn labels_w() {
    assert_lex! {
        "while w",
        WHILE_KW:5,
        WHITESPACE:1,
        IDENT:1
    }

    assert_lex! {
        "whiley withow",
        IDENT:6,
        WHITESPACE:1,
        IDENT:6
    }
}

#[test]
fn labels_0_9() {
    assert_lex! {
        "123ii",
        IDENT:5
    }
    assert_lex! {
        "_i64",
        IDENT:4
    }
    assert_lex! {
        "_i32",
        IDENT:4
    }
}

#[test]
fn number_basic() {
    assert_lex! {
        "1",
        M_NUMBER_LITERAL:1
    }

    assert_lex! {
        "123456 ",
        M_NUMBER_LITERAL:6,
        WHITESPACE:1
    }

    assert_lex! {
        "90",
        M_NUMBER_LITERAL:2
    }

    assert_lex! {
        ".13",
        DOT:1,
        M_NUMBER_LITERAL:2
    }
    assert_lex! {
        "13.",
        M_NUMBER_LITERAL:3
    }
}

#[test]
fn number_with_int_suffix() {
    assert_lex! {
        "123i 123I",
        M_NUMBER_LITERAL:4
        WHITESPACE:1
        M_NUMBER_LITERAL:4
    }
    assert_lex! {
        "123_i32",
        M_NUMBER_LITERAL:7
    }
    assert_lex! {
        "123_i64",
        M_NUMBER_LITERAL:7
    }
}

#[test]
fn number_basic_err() {
    assert_lex! {
        r"25\u0046abcdef",
        ERROR_TOKEN:14
    }

    assert_lex! {
        r"25\uFEFFb",
        ERROR_TOKEN:9
    }

    assert_lex! {
        r".32\u0046abde",
        DOT:1,
        ERROR_TOKEN:12
    }

    assert_lex! {
        r#"10e_1"#,
        IDENT:5
    }
}

#[test]
fn number_leading_zero() {
    assert_lex! {
        r#"01.1"#,
        M_NUMBER_LITERAL:4,
    }
}

#[test]
fn number_complex() {
    assert_lex! {
        "3e-5 123e+56",
        M_NUMBER_LITERAL:4,
        WHITESPACE:1,
        M_NUMBER_LITERAL:7
    }

    assert_lex! {
        "3.14159e+1",
        M_NUMBER_LITERAL:10
    }

    assert_lex! {
        ".0e34",
        DOT:1,
        M_NUMBER_LITERAL:4
    }

    assert_lex! {
        "0e00",
        M_NUMBER_LITERAL:4
    }
}

#[test]
fn dot_number_disambiguation() {
    assert_lex! {
        ".e+5",
        DOT:1,
        IDENT:1,
        PLUS:1,
        M_NUMBER_LITERAL:1
    }

    assert_lex! {
        ".0e+5",
        DOT:1,
        M_NUMBER_LITERAL:4
    }
}

#[test]
fn int32_literals() {
    assert_lex! {
        "0_i32 1743642_i32 1_i32",
        M_NUMBER_LITERAL:5,
        WHITESPACE:1,
        M_NUMBER_LITERAL:11,
        WHITESPACE:1,
        M_NUMBER_LITERAL:5
    }
}

#[test]
fn int64_literals() {
    assert_lex! {
        "0i 1743642I 1_i64",
        M_NUMBER_LITERAL:2,
        WHITESPACE:1,
        M_NUMBER_LITERAL:8,
        WHITESPACE:1,
        M_NUMBER_LITERAL:5
    }
}

#[test]
fn single_line_comments() {
    assert_lex! {
        "# abc
    ",
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:4
    }

    assert_lex! {
        "# a",
        COMMENT:3
    }
}

#[test]
fn division() {
    assert_lex! {
        "var a = 5 / 6",
        VAR_KW:3,
        WHITESPACE:1,
        IDENT:1,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        M_NUMBER_LITERAL:1,
        WHITESPACE:1,
        SLASH:1,
        WHITESPACE:1,
        M_NUMBER_LITERAL:1
    }
}

#[test]
fn fuzz_fail_1() {
    assert_lex! {
        "$\\u",
        ERROR_TOKEN:3,
    }
}

#[test]
fn fuzz_fail_2() {
    assert_lex! {
        "..",
        DOT:1,
        DOT:1
    }
}

#[test]
fn fuzz_fail_3() {
    assert_lex! {
        "0e",
        IDENT:2
    }
}

#[test]
fn fuzz_fail_4() {
    assert_lex! {
        "0o 0b 0x",
        IDENT:2,
        WHITESPACE:1,
        IDENT:2,
        WHITESPACE:1,
        IDENT:2
    }
}

#[test]
fn fuzz_fail_5() {
    assert_lex! {
        "#\u{2028}",
        COMMENT:1,
        NEWLINE:3
    }
}

#[test]
fn fuzz_fail_6() {
    assert_lex! {
        "#\u{200a}",
        COMMENT:4
    }
}

#[test]
fn unicode_ident_start_handling() {
    assert_lex! {
        "αβeta_tester",
        IDENT:14
    }
}

#[test]
fn unicode_ident_start_numbers() {
    assert_lex! {
        "1234_Тест",
        IDENT:13
    }
}

#[test]
fn unicode_ident_separated_by_unicode_whitespace() {
    assert_lex! {
        "β\u{FEFF}α",
        IDENT:2,
        WHITESPACE:3,
        IDENT:2
    }
}

#[test]
fn issue_30() {
    assert_lex! {
        "var foo = { α: true }",
        VAR_KW:3,
        WHITESPACE:1,
        IDENT:3,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        L_CURLY:1,
        WHITESPACE:1,
        IDENT:2,
        COLON:1,
        WHITESPACE:1,
        TRUE_KW:4,
        WHITESPACE:1,
        R_CURLY:1
    }
}
#[test]
fn at_token() {
    assert_lex! {
        "@",
        AT:1
    }

    assert_lex! {
        "@foo",
        IDENT:4
    }
}

#[test]
fn object_expr_getter() {
    assert_lex! {
        "({ get [foo]() {} })",
        L_PAREN:1
        L_CURLY:1
        WHITESPACE:1
        GET_KW:3
        WHITESPACE:1
        L_BRACK:1
        IDENT:3
        R_BRACK:1
        L_PAREN:1
        R_PAREN:1
        WHITESPACE:1
        L_CURLY:1
        R_CURLY:1
        WHITESPACE:1
        R_CURLY:1
        R_PAREN:1
    }
}

#[test]
fn newline_space_must_be_two_tokens() {
    assert_lex! {
        "\n ",
        NEWLINE:1
        WHITESPACE:1
    }
    assert_lex! {
        " \n",
        WHITESPACE:1
        NEWLINE:1
    }
    assert_lex! {
        " \n ",
        WHITESPACE:1
        NEWLINE:1
        WHITESPACE:1
    }

    assert_lex! {
        " a\n b \n ",
        WHITESPACE:1
        IDENT:1
        NEWLINE:1
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        NEWLINE:1
        WHITESPACE:1
    }

    //Now with CR
    assert_lex! {
        "\r\n ",
        NEWLINE:2
        WHITESPACE:1
    }

    assert_lex! {
        " \r\n",
        WHITESPACE:1
        NEWLINE:2
    }
    assert_lex! {
        " \r\n ",
        WHITESPACE:1
        NEWLINE:2
        WHITESPACE:1
    }

    assert_lex! {
        " a\r\n b \r\n ",
        WHITESPACE:1
        IDENT:1
        NEWLINE:2
        WHITESPACE:1
        IDENT:1
        WHITESPACE:1
        NEWLINE:2
        WHITESPACE:1
    }
}

#[test]
fn numbers() {
    assert_lex! {
        "0(",
        M_NUMBER_LITERAL:1,
        L_PAREN:1
    }
}

#[test]
fn keywords() {
    #[rustfmt::skip]
    let keywords = vec![
        "and", "и",
        "break", "прервать",
        "case", "выбор",
        "catch", "исключение", "перехват",
        "class", "класс",
        "continue", "продолжить",
        "debug", "отладить",
        "delete", "удалить",
        "else", "иначе",
        "extends", "расширяет",
        "false", "ложь",
        "finally", "заключение",
        "for", "для",
        "forall", "длявсех",
        "func", "функция",
        "if", "если",
        "in", "в",
        "new", "новый",
        "null", "nil", "нуль",
        "return", "вернуть",
        "super", "базовый",
        "switch", "выборпо",
        "or", "или",
        "this", "этот",
        "throw", "вызватьисключение",
        "try", "попытка",
        "true", "истина",
        "var", "перем",
        "while", "пока",
        // contextual keywords
        "constructor", 
        "get", "получить",
        "set", "установить",
    ];

    for keyword in keywords {
        let kind = MSyntaxKind::from_keyword(keyword).expect(
            format!("Expected `MSyntaxKind::from_keyword` to return a kind for keyword {keyword}.")
                .as_str(),
        );

        let mut lexer = MLexer::from_str(keyword);
        lexer.next_token(MLexContext::default());

        let lexed_kind = lexer.current();
        assert_eq!(
            lexed_kind, kind,
            "Expected token '{keyword}' to be of kind {kind:?} but is {lexed_kind:?}."
        );

        let lexed_range = lexer.current_range();
        assert_eq!(
            lexed_range.len(),
            TextSize::from(keyword.len() as u32),
            "Expected lexed keyword to be of len {} but has length {:?}",
            keyword.len(),
            lexed_range.len()
        );

        assert_eq!(lexer.next_token(MLexContext::default()), EOF);
    }
}

#[test]
fn without_lookahead() {
    let lexer = MLexer::from_str("var a\n = 5");
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(MLexContext::default());
    assert_eq!(buffered.current(), T![var]);
    assert!(!buffered.has_preceding_line_break());
    assert_eq!(
        buffered.current_range(),
        TextRange::at(TextSize::from(0), TextSize::from(3))
    );

    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(MLexContext::default()), T![ident]);
    assert_eq!(buffered.next_token(MLexContext::default()), NEWLINE);
    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(MLexContext::default()), T![=]);
    assert!(buffered.has_preceding_line_break());
    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);
    assert_eq!(
        buffered.next_token(MLexContext::default()),
        M_NUMBER_LITERAL
    );
    assert_eq!(buffered.next_token(MLexContext::default()), T![EOF]);
}

#[test]
fn lookahead() {
    let lexer = MLexer::from_str("var a\n = 5");
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(MLexContext::default());
    assert_eq!(buffered.current(), T![var]);
    assert!(!buffered.has_preceding_line_break());
    assert_eq!(
        buffered.current_range(),
        TextRange::at(TextSize::from(0), TextSize::from(3))
    );

    {
        let lookahead = buffered
            .lookahead_iter()
            .map(|l| l.kind())
            .collect::<Vec<_>>();

        assert_eq!(
            lookahead,
            vec![
                WHITESPACE,
                T![ident],
                NEWLINE,
                WHITESPACE,
                T![=],
                WHITESPACE,
                M_NUMBER_LITERAL,
                T![EOF]
            ]
        );
    }

    assert_eq!(buffered.current(), T![var]);
    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);

    {
        let mut lookahead = buffered.lookahead_iter();
        let nth1 = lookahead.next().unwrap();
        let nth2 = lookahead.next().unwrap();
        let nth3 = lookahead.next().unwrap();
        let nth4 = lookahead.next().unwrap();

        assert_eq!(nth1.kind(), T![ident]);
        assert_eq!(nth2.kind(), NEWLINE);
        assert_eq!(nth3.kind(), WHITESPACE);
        assert_eq!(nth4.kind(), T![=]);
        assert!(nth4.has_preceding_line_break());
    }

    assert_eq!(buffered.next_token(MLexContext::default()), T![ident]);
    assert_eq!(buffered.next_token(MLexContext::default()), NEWLINE);
    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);
    assert_eq!(buffered.next_token(MLexContext::default()), T![=]);
    assert!(buffered.has_preceding_line_break());
    assert_eq!(buffered.next_token(MLexContext::default()), WHITESPACE);
    assert_eq!(
        buffered.next_token(MLexContext::default()),
        M_NUMBER_LITERAL
    );
    assert_eq!(buffered.next_token(MLexContext::default()), T![EOF]);
}
