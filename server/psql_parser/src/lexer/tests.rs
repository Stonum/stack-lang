#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use super::{PsqlLexContext, PsqlLexer};
use psql_syntax::PsqlSyntaxKind::{self, EOF};
use psql_syntax::T;

use biome_parser::lexer::{BufferedLexer, Lexer};
use biome_rowan::TextRange;
use biome_rowan::TextSize;

// Макрос для проверки лексирования
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = PsqlLexer::from_str($src);
        let mut idx = 0;
        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(PsqlLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                psql_syntax::PsqlSyntaxKind::$kind,
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

            new_str.push_str(&$src[tokens[idx].1]);
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
    assert_lex! { "", }
}

#[test]
fn identifiers() {
    assert_lex! { "my_table", IDENT:8 }
    assert_lex! { "user_name", IDENT:9 }
    assert_lex! { "id", IDENT:2 }
    assert_lex! { "column1", IDENT:7 }
    assert_lex! { "CamelCase", IDENT:9 }
    assert_lex! { "with_underscore", IDENT:15 }
    assert_lex! { "_starts_with_underscore", IDENT:23 }
}

#[test]
fn keywords() {
    let keywords = vec![
        "select",
        "from",
        "where",
        "and",
        "or",
        "not",
        "insert",
        "update",
        "delete",
        "create",
        "table",
        "view",
        "index",
        "drop",
        "alter",
        "join",
        "on",
        "as",
        "distinct",
        "group_by",
        "order_by",
        "having",
        "union",
        "case",
        "when",
        "then",
        "end",
        "if",
        "else",
        "null",
        "true",
        "false",
        "between",
        "in",
        "like",
        "ilike",
        "is",
        "asc",
        "desc",
        "primary",
        "foreign",
        "key",
        "unique",
        "constraint",
        "check",
        "default",
        "integer",
        "bigint",
        "varchar",
        "text",
        "boolean",
        "date",
        "timestamp",
        "interval",
        "numeric",
        "json",
        "jsonb",
        "uuid",
        "array",
        "bytea",
    ];

    for keyword in keywords {
        let kind = PsqlSyntaxKind::from_keyword(keyword).unwrap_or_else(|| {
            panic!(
                "Expected `PsqlSyntaxKind::from_keyword` to return a kind for keyword {keyword}."
            )
        });

        let mut lexer = PsqlLexer::from_str(keyword);
        lexer.next_token(PsqlLexContext::default());

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

        assert_eq!(lexer.next_token(PsqlLexContext::default()), EOF);
    }
}

#[test]
fn punctuators() {
    assert_lex! {
        "(),.:;+-*/<!>~=&|^%",
        L_PAREN:1,
        R_PAREN:1,
        COMMA:1,
        DOT:1,
        COLON:1,
        SEMICOLON:1,
        PLUS:1,
        MINUS:1,
        STAR:1,
        SLASH:1,
        L_ANGLE:1,
        BANG:1,
        R_ANGLE:1,
        TILDE:1,
        EQ:1,
        AMP:1,
        PIPE:1,
        CARET:1,
        PERCENT:1,
    }
}

#[test]
fn comparison_operators() {
    assert_lex! { ">", R_ANGLE:1 }
    assert_lex! { "<", L_ANGLE:1 }
    assert_lex! { "=", EQ:1 }
    assert_lex! { "!=", NEQ:2 }
    assert_lex! { "<>", LTGT:2 }
    assert_lex! { ">=", GTEQ:2 }
    assert_lex! { "<=", LTEQ:2 }
    assert_lex! { "~", TILDE:1 }
    assert_lex! { "~*", RGX:2 }
    assert_lex! { "!~", NEG_TILDE:2 }
    assert_lex! { "!~*", NEG_RGX:3 }
}

#[test]
fn strings() {
    assert_lex! { r#"'simple'"#, PSQL_STRING_LITERAL:8 }
    assert_lex! { r#"''"#, PSQL_STRING_LITERAL:2 }
    assert_lex! { r#"'with spaces and words'"#, PSQL_STRING_LITERAL:23 }
    assert_lex! { r#"'with '' quotes'"#, PSQL_STRING_LITERAL:16 } // doubled quotes
    // assert_lex! { r#"$$dollar quoted$$"#, PSQL_STRING_LITERAL:16 } // dollar quoted are not supported yet
    assert_lex! { r#"'with \n escape'"#, PSQL_STRING_LITERAL:16 }
}

#[test]
fn unterminated_string() {
    assert_lex! { r#"'unterminated"#, ERROR_TOKEN:13 }
    assert_lex! { r#"'abc\"#, ERROR_TOKEN:5 }
    assert_lex! { r#"'abc''"#, ERROR_TOKEN:6 }
}

#[test]
fn numbers() {
    assert_lex! { "123", PSQL_NUMBER_LITERAL:3 }
    assert_lex! { "0", PSQL_NUMBER_LITERAL:1 }
    assert_lex! { "3.14", PSQL_NUMBER_LITERAL:4 }
    assert_lex! { "2.5e10", PSQL_NUMBER_LITERAL:6 }
    assert_lex! { ".5", DOT:1, PSQL_NUMBER_LITERAL:1 }
    assert_lex! { "1.", PSQL_NUMBER_LITERAL:2 }
}

#[test]
fn whitespace_and_newlines() {
    assert_lex! { " ", WHITESPACE:1 }
    assert_lex! { "\t", WHITESPACE:1 }
    assert_lex! { "\n", NEWLINE:1 }
    assert_lex! { "\r\n", NEWLINE:2 }
    assert_lex! { " \t\n\r\n  ", WHITESPACE:2, NEWLINE:1, NEWLINE:2, WHITESPACE:2 }
}

#[test]
fn comments() {
    assert_lex! { "-- single line comment", COMMENT:22 }
    assert_lex! { "--", COMMENT:2 }
    assert_lex! { "-- a\nSELECT", COMMENT:4, NEWLINE:1, SELECT_KW:6 }
    assert_lex! { "x -- comment", IDENT:1, WHITESPACE:1, COMMENT:10 }
    assert_lex! { "/* comment */", COMMENT:13 }
    assert_lex! { "/* comment */SELECT", COMMENT:13, SELECT_KW:6 }
    assert_lex! { "/* comment /n another */", COMMENT:24 }
}

#[test]
fn simple_select() {
    assert_lex! {
        "SELECT * FROM users WHERE age > 18;",
        SELECT_KW:6,
        WHITESPACE:1,
        STAR:1,
        WHITESPACE:1,
        FROM_KW:4,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:3,
        WHITESPACE:1,
        R_ANGLE:1,
        WHITESPACE:1,
        PSQL_NUMBER_LITERAL:2,
        SEMICOLON:1,
    }
}

#[test]
fn insert_statement() {
    assert_lex! {
        "INSERT INTO users (name, age) VALUES ('Alice', 30);",
        INSERT_KW:6,
        WHITESPACE:1,
        INTO_KW:4,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        L_PAREN:1,
        IDENT:4,
        COMMA:1,
        WHITESPACE:1,
        IDENT:3,
        R_PAREN:1,
        WHITESPACE:1,
        VALUES_KW:6,
        WHITESPACE:1,
        L_PAREN:1,
        PSQL_STRING_LITERAL:7,
        COMMA:1,
        WHITESPACE:1,
        PSQL_NUMBER_LITERAL:2,
        R_PAREN:1,
        SEMICOLON:1,
    }
}

#[test]
fn update_statement() {
    assert_lex! {
        "UPDATE users SET name = 'Bob' WHERE id = 1;",
        UPDATE_KW:6,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        SET_KW:3,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        PSQL_STRING_LITERAL:5,
        WHITESPACE:1,
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:2,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        PSQL_NUMBER_LITERAL:1,
        SEMICOLON:1,
    }
}

#[test]
fn complex_where() {
    assert_lex! {
        "WHERE name LIKE 'A%' AND active IS TRUE;",
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        LIKE_KW:4,
        WHITESPACE:1,
        PSQL_STRING_LITERAL:4,
        WHITESPACE:1,
        AND_KW:3,
        WHITESPACE:1,
        IDENT:6,
        WHITESPACE:1,
        IS_KW:2,
        WHITESPACE:1,
        TRUE_KW:4,
        SEMICOLON:1,
    }
}

#[test]
fn case_expression() {
    assert_lex! {
        "CASE WHEN x > 0 THEN 'positive' ELSE 'zero' END",
        CASE_KW:4,
        WHITESPACE:1,
        WHEN_KW:4,
        WHITESPACE:1,
        IDENT:1,
        WHITESPACE:1,
        R_ANGLE:1,
        WHITESPACE:1,
        PSQL_NUMBER_LITERAL:1,
        WHITESPACE:1,
        THEN_KW:4,
        WHITESPACE:1,
        PSQL_STRING_LITERAL:10,
        WHITESPACE:1,
        ELSE_KW:4,
        WHITESPACE:1,
        PSQL_STRING_LITERAL:6,
        WHITESPACE:1,
        END_KW:3,
    }
}

#[test]
fn lookahead_buffer() {
    use psql_syntax::PsqlSyntaxKind::{FROM_KW, IDENT, STAR, WHITESPACE};

    let lexer = PsqlLexer::from_str("SELECT * FROM t");
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(PsqlLexContext::default());
    assert_eq!(buffered.current(), T![select]);
    assert!(!buffered.has_preceding_line_break());

    {
        let lookahead: Vec<_> = buffered.lookahead_iter().map(|l| l.kind()).collect();
        assert_eq!(
            lookahead,
            vec![
                WHITESPACE, STAR, WHITESPACE, FROM_KW, WHITESPACE, IDENT, EOF
            ]
        );
    }

    buffered.next_token(PsqlLexContext::default()); // WHITESPACE
    buffered.next_token(PsqlLexContext::default()); // STAR
    buffered.next_token(PsqlLexContext::default()); // WHITESPACE
    buffered.next_token(PsqlLexContext::default()); // FROM_KW
    assert!(!buffered.has_preceding_line_break());

    buffered.next_token(PsqlLexContext::default()); // WHITESPACE
    buffered.next_token(PsqlLexContext::default()); // IDENT
    buffered.next_token(PsqlLexContext::default()); // EOF
    assert_eq!(buffered.current(), EOF);
}
