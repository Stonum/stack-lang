mod errors;
mod tests;

use psql_syntax::PsqlSyntaxKind;
use psql_syntax::PsqlSyntaxKind::*;
use psql_syntax::T;

use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_unicode_table::{
    Dispatch::{self, *},
    lookup_byte,
};

/// Контекст лексера для PSQL
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PsqlLexContext;

impl LexContext for PsqlLexContext {
    fn is_regular(&self) -> bool {
        true
    }
}

/// Контекст повторного лексирования
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PsqlReLexContext {}

/// Лексер для PostgreSQL SQL
#[derive(Debug)]
pub struct PsqlLexer<'src> {
    source: &'src str,
    position: usize,
    after_newline: bool,
    unicode_bom_length: usize,
    current_start: TextSize,
    current_kind: PsqlSyntaxKind,
    current_flags: TokenFlags,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for PsqlLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = PsqlSyntaxKind;
    type LexContext = PsqlLexContext;
    type ReLexContext = PsqlReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = TextSize::from(self.position as u32);
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() { EOF } else { self.lex_token() };

        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);
        self.current_kind = kind;

        if !kind.is_trivia() {
            self.after_newline = false;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    fn has_unicode_escape(&self) -> bool {
        self.current_flags.has_unicode_escape()
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        self.position = u32::from(position) as usize;
        self.current_start = current_start;
        self.current_flags = current_flags;
        self.current_kind = current_kind;
        self.after_newline = after_line_break;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn position(&self) -> usize {
        self.position
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn consume_newline_or_whitespaces(&mut self) -> PsqlSyntaxKind {
        if self.consume_newline() {
            self.after_newline = true;
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }
}

impl<'src> ReLexer<'src> for PsqlLexer<'src> {
    // dummy implementation
    fn re_lex(&mut self, _context: Self::ReLexContext) -> Self::Kind {
        // let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        self.current()
        // let re_lexed_kind = match context {
        //     PsqlReLexContext::GlobalIdentifier => self.re_lex_global_identifier(),
        //     PsqlReLexContext::BinaryOperator => self.re_lex_binary_operator(),
        // };

        // if self.current() == re_lexed_kind {
        //     self.position = old_position;
        // } else {
        //     self.current_kind = re_lexed_kind;
        // }

        // re_lexed_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for PsqlLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: TextSize::from(self.position as u32),
            current_start: self.current_start,
            current_flags: self.current_flags,
            current_kind: self.current_kind,
            after_line_break: self.after_newline,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

impl<'src> PsqlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            after_newline: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            diagnostics: vec![],
        }
    }

    fn eat_byte(&mut self, kind: PsqlSyntaxKind) -> PsqlSyntaxKind {
        self.advance(1);
        kind
    }

    fn consume_newline(&mut self) -> bool {
        let start = self.position;
        match self.current_byte() {
            Some(b'\r') if self.peek_byte() == Some(b'\n') => self.advance(2),
            Some(b'\r' | b'\n') => self.advance(1),
            Some(chr) if !chr.is_ascii() => {
                let chr = self.current_char_unchecked();
                if is_linebreak(chr) {
                    self.advance(chr.len_utf8());
                }
            }
            _ => {}
        }
        self.position != start
    }

    fn consume_whitespaces(&mut self) {
        while let Some(chr) = self.current_byte() {
            match lookup_byte(chr) {
                WHS => {
                    if chr == b'\r' || chr == b'\n' {
                        break;
                    } else {
                        self.advance(1);
                    }
                }
                UNI if self.current_char_unchecked().is_whitespace() => {
                    self.advance_char_unchecked();
                }
                _ => break,
            }
        }
    }

    fn resolve_identifier(&mut self) -> PsqlSyntaxKind {
        let mut buf = [0u8; 32];
        let count = self.consume_and_get_ident(&mut buf);

        match std::str::from_utf8(&buf[..count]) {
            Ok(s) => PsqlSyntaxKind::from_keyword(s.to_lowercase().as_str()).unwrap_or(T![ident]),
            Err(_) => ERROR_TOKEN,
        }
    }

    fn consume_and_get_ident(&mut self, buf: &mut [u8]) -> usize {
        let mut idx = 0;
        while let Some(b) = self.current_byte() {
            if is_id_continue(b as char) {
                if let Some(slot) = buf.get_mut(idx) {
                    *slot = b;
                }
                self.advance(1);
                idx += 1;
            } else {
                break;
            }
        }
        idx
    }

    fn consume_str_literal(&mut self) -> bool {
        let quote = self.current_byte().unwrap();
        self.advance(1);
        while let Some(b) = self.current_byte() {
            if b == quote {
                self.advance(1);
                // if quote doubled, skip it
                if self.current_byte() == Some(quote) {
                    self.advance(1);
                    continue;
                }
                return true;
            } else if b == b'\\' {
                self.consume_escape_sequence();
            } else {
                self.advance_byte_or_char(b);
            }
        }
        self.push_diagnostic(ParseDiagnostic::new(
            "unterminated string literal",
            self.position..self.position,
        ));
        false
    }

    fn consume_escape_sequence(&mut self) -> bool {
        self.assert_byte(b'\\');
        self.advance(1);
        if let Some(b) = self.current_byte() {
            self.advance_byte_or_char(b);
            true
        } else {
            false
        }
    }

    fn resolve_number(&mut self) -> PsqlSyntaxKind {
        while let Some(b) = self.current_byte() {
            match b {
                b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-' => self.advance(1),
                _ => break,
            }
        }
        PSQL_NUMBER_LITERAL
    }

    #[inline]
    fn resolve_less_than(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'<') => {
                self.next_byte();
                T![<<]
            }
            Some(b'>') => {
                self.next_byte();
                T![<>]
            }
            Some(b'=') => {
                self.next_byte();
                T![<=]
            }
            _ => T![<],
        }
    }

    #[inline]
    fn resolve_more_than(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'>') => {
                T![>>]
            }
            Some(b'=') => {
                self.next_byte();
                T![>=]
            }
            _ => T![>],
        }
    }

    #[inline]
    fn resolve_tilde(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'*') => {
                self.next_byte();
                T![~*]
            }
            _ => T![~],
        }
    }

    #[inline]
    fn resolve_bang(&mut self) -> PsqlSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                T![!=]
            }
            Some(b'~') => match self.next_byte() {
                Some(b'*') => {
                    self.next_byte();
                    T![!~*]
                }
                _ => T![!~],
            },
            _ => T![!],
        }
    }

    fn read_minus(&mut self) -> PsqlSyntaxKind {
        self.advance(1); // eats '-'
        if self.current_byte() != Some(b'-') {
            return T![-];
        }

        while let Some(chr) = self.current_byte() {
            if let b'\r' | b'\n' = chr {
                return COMMENT;
            } else if chr.is_ascii() {
                self.advance(1);
            } else {
                let chr = self.current_char_unchecked();
                if is_linebreak(chr) {
                    return COMMENT;
                } else {
                    self.advance(chr.len_utf8());
                }
            }
        }

        COMMENT
    }

    fn read_slash(&mut self) -> PsqlSyntaxKind {
        self.advance(1); // eats '/'
        if self.current_byte() != Some(b'*') {
            return T![/];
        }

        while let Some(chr) = self.current_byte() {
            if let b'*' = chr
                && self.byte_at(1) == Some(b'/')
            {
                self.advance(2);
                return COMMENT;
            } else if chr.is_ascii() {
                self.advance(1);
            } else {
                let chr = self.current_char_unchecked();
                self.advance(chr.len_utf8());
            }
        }

        ERROR_TOKEN
    }

    fn lex_token(&mut self) -> PsqlSyntaxKind {
        let byte = self.current_byte().unwrap_or(0);
        let dispatched = lookup_byte(byte);

        match dispatched {
            WHS => self.consume_newline_or_whitespaces(),
            QOT if byte == b'\'' => {
                if self.consume_str_literal() {
                    PSQL_STRING_LITERAL
                } else {
                    ERROR_TOKEN
                }
            }
            IDT => self.resolve_identifier(),
            DIG | ZER => self.resolve_number(),
            PNO => self.eat_byte(T!['(']),
            PNC => self.eat_byte(T![')']),
            COM => self.eat_byte(T![,]),
            SEM => self.eat_byte(T![;]),
            PRD => self.eat_byte(T![.]),
            COL => self.eat_byte(T![:]),
            LSS => self.resolve_less_than(),
            MOR => self.resolve_more_than(),
            TLD => self.resolve_tilde(),
            PIP => self.eat_byte(T![|]),
            Dispatch::AMP => self.eat_byte(T![&]),
            CRT => self.eat_byte(T![^]),
            PLS => self.eat_byte(T![+]),
            MIN => self.read_minus(),
            MUL => self.eat_byte(T![*]),
            SLH => self.read_slash(),
            EQL => self.eat_byte(T![=]),
            PRC => self.eat_byte(T![%]),
            EXL => self.resolve_bang(),
            _ => {
                let err = ParseDiagnostic::new(
                    format!("unexpected token `{}`", byte as char),
                    self.position..self.position + 1,
                );
                self.push_diagnostic(err);
                self.advance(1);
                ERROR_TOKEN
            }
        }
    }
}

fn is_linebreak(c: char) -> bool {
    c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}'
}

fn is_id_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
