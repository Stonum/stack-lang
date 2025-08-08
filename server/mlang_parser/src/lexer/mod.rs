mod errors;
mod tests;

use mlang_syntax::MSyntaxKind;
use mlang_syntax::MSyntaxKind::*;
use mlang_syntax::T;

use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::{
    LexContext, Lexer, LexerCheckpoint, LexerWithCheckpoint, ReLexer, TokenFlags,
};
use biome_rowan::SyntaxKind;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_unicode_table::{
    Dispatch::{self, *},
    is_js_id_continue, is_js_id_start, lookup_byte,
};
use chrono::{NaiveDate, NaiveTime};

use self::errors::invalid_digits_after_unicode_escape_sequence;

// The first utf8 byte of every valid unicode whitespace char, used for short circuiting whitespace checks
const UNICODE_WHITESPACE_STARTS: [u8; 5] = [
    // NBSP
    0xC2, // BOM
    0xEF, // Ogham space mark
    0xE1, // En quad .. Hair space, narrow no break space, mathematical space
    0xE2, // Ideographic space
    0xE3,
];

// Unicode spaces, designated by the `Zs` unicode property
const UNICODE_SPACES: [char; 19] = [
    '\u{0020}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}',
    '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}', '\u{202F}',
    '\u{205F}', '\u{3000}', '\u{FEFF}',
];

/// Context in which the lexer should lex the next token
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct MLexContext;

impl LexContext for MLexContext {
    /// Returns true if this is [MLexContext::Regular]
    fn is_regular(&self) -> bool {
        true
    }
}

/// Context in which the [MLexContext]'s current should be re-lexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MReLexContext {
    /// Re-lexes
    /// * `> =` as '>='
    BinaryOperator,

    /// Re-lexes . name to .name
    GlobalIdentifier,

    /// Relexed 10:55 to  10 : 55
    KeyValue,
}

/// An extremely fast, lookup table based, lossless ECMAScript lexer
#[derive(Debug)]
pub struct MLexer<'src> {
    /// Source text
    source: &'src str,

    /// The start byte position in the source text of the next token.
    position: usize,

    /// `true` if there has been a line break between the last non-trivia token and the next non-trivia token.
    after_newline: bool,

    /// If the source starts with a Unicode BOM, this is the number of bytes for that token.
    unicode_bom_length: usize,

    /// Byte offset of the current token from the start of the source
    /// The range of the current token can be computed by `self.position - self.current_start`
    current_start: TextSize,

    /// The kind of the current token
    current_kind: MSyntaxKind,

    /// Flags for the current token
    current_flags: TokenFlags,

    diagnostics: Vec<ParseDiagnostic>,

    // 'true' if the line starts from ff token
    after_ff: bool,
}

impl<'src> Lexer<'src> for MLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = MSyntaxKind;
    type LexContext = MLexContext;
    type ReLexContext = MReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_range(&self) -> TextRange {
        TextRange::new(self.current_start, TextSize::from(self.position as u32))
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.position += c.len_utf8();
    }

    #[inline]
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
        // test_err M M_rewind_at_eof_token
        // (([zAgRvz=[=(e{V{

        let LexerCheckpoint {
            position,
            current_start,
            current_flags,
            current_kind,
            after_line_break,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        let new_pos = u32::from(position) as usize;

        self.position = new_pos;
        self.current_kind = current_kind;
        self.current_start = current_start;
        self.current_flags = current_flags;
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

    /// Consume one newline or all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline_or_whitespaces(&mut self) -> MSyntaxKind {
        if self.consume_newline() {
            self.after_newline = true;
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }
}

impl<'src> ReLexer<'src> for MLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let re_lexed_kind = match context {
            MReLexContext::BinaryOperator => self.re_lex_binary_operator(),
            MReLexContext::GlobalIdentifier => self.re_lex_global_identifier(),
            MReLexContext::KeyValue => self.re_lex_key_value(),
        };

        if self.current() == re_lexed_kind {
            // Didn't re-lex anything. Return existing token again
            self.position = old_position;
        } else {
            self.current_kind = re_lexed_kind;
        }

        re_lexed_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for MLexer<'src> {
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

impl<'src> MLexer<'src> {
    /// Make a new lexer from a str, this is safe because strs are valid utf8
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            after_newline: false,
            unicode_bom_length: 0,
            current_kind: TOMBSTONE,
            current_start: TextSize::from(0),
            current_flags: TokenFlags::empty(),
            position: 0,
            diagnostics: vec![],
            after_ff: false,
        }
    }

    fn re_lex_binary_operator(&mut self) -> MSyntaxKind {
        if self.current_byte() == Some(b'>') {
            match self.next_byte() {
                Some(b'=') => self.eat_byte(T![>=]),
                _ => T![>],
            }
        } else {
            self.current_kind
        }
    }

    fn re_lex_global_identifier(&mut self) -> MSyntaxKind {
        if self.current_byte() == Some(b'.') {
            self.next_byte();
            if self.cur_ident_part(false).is_some() {
                self.consume_ident();
                return T![ident];
            }
            return T![ident];
        }
        self.current_kind
    }

    fn re_lex_key_value(&mut self) -> MSyntaxKind {
        if self.byte_at(2) == Some(b':') {
            if let (Some(b'0'..=b'9'), Some(b'0'..=b'9')) = (self.current_byte(), self.peek_byte())
            {
                self.advance(2);
                return M_NUMBER_LITERAL;
            }
        }
        self.current_kind
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind
    fn eat_byte(&mut self, tok: MSyntaxKind) -> MSyntaxKind {
        self.next_byte();
        tok
    }

    fn prev_byte(&self, offset: usize) -> Option<u8> {
        if offset > self.position {
            return None;
        }

        self.source.as_bytes().get(self.position - offset).copied()
    }

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_newline(&mut self) -> bool {
        self.assert_current_char_boundary();

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

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_whitespaces(&mut self) {
        self.assert_current_char_boundary();

        while let Some(chr) = self.current_byte() {
            match lookup_byte(chr) {
                Dispatch::WHS => {
                    if let b'\r' | b'\n' = chr {
                        break;
                    } else {
                        self.next_byte();
                    }
                }
                Dispatch::UNI => {
                    let chr = self.current_char_unchecked();

                    if UNICODE_SPACES.contains(&chr) {
                        self.advance(chr.len_utf8());
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn consume_ff(&mut self) -> MSyntaxKind {
        self.assert_current_char_boundary();

        self.after_ff = true;
        if self.peek_byte() == Some(12) {
            self.advance(2);
            return T![ff2];
        }
        self.eat_byte(T![ff])
    }

    /// Returns the current byte without checking if the lexer is at the end of the file.
    ///
    /// ## Safety
    /// Calling this function if the lexer is at or passed the end of file is undefined behaviour.
    #[inline]
    unsafe fn current_unchecked(&self) -> u8 {
        unsafe {
            self.assert_current_char_boundary();
            *self.source.as_bytes().get_unchecked(self.position)
        }
    }

    /// Advances the position by one and returns the next byte value
    #[inline]
    fn next_byte(&mut self) -> Option<u8> {
        self.advance(1);
        self.current_byte()
    }

    /// Get the next byte but only advance the index if there is a next byte.
    /// This is really just a hack for certain methods like escapes
    #[inline]
    fn next_byte_bounded(&mut self) -> Option<u8> {
        if let Some(b) = self.source.as_bytes().get(self.position + 1) {
            self.advance(1);
            Some(*b)
        } else {
            if !self.is_eof() {
                // Move the cursor by one to position the Lexer at the EOF token
                self.advance(1);
            }
            None
        }
    }

    /// Advances the current position by `n` bytes.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    #[inline]
    fn advance_byte_or_char(&mut self, chr: u8) {
        if chr.is_ascii() {
            self.advance(1);
        } else {
            self.advance_char_unchecked();
        }
    }

    // Read a `\u{000...}` escape sequence, this expects the cur char to be the `{`
    fn read_codepoint_escape_char(&mut self) -> Result<char, ()> {
        let start = self.position + 1;
        self.read_hexnumber();

        let current_byte = self.current_byte();

        // Abort on EOF
        if current_byte.is_none() {
            return Err(());
        }

        if current_byte != Some(b'}') {
            // We should not yield diagnostics on a unicode char boundary. That wont make codespan panic
            // but it may cause a panic for other crates which just consume the diagnostics
            let invalid = self.current_char_unchecked();
            let err = ParseDiagnostic::new(
                "expected hex digits for a unicode code point escape, but encountered an invalid character",
                self.position..self.position + invalid.len_utf8(),
            );
            self.push_diagnostic(err);
            self.position -= 1;
            return Err(());
        }

        // Safety: We know for a fact this is in bounds because we must be on the possible char after the } at this point
        // which means its impossible for the range of the digits to be out of bounds.
        // We also know we cant possibly be indexing a unicode char boundary because a unicode char (which cant be a hexdigit)
        // would have triggered the if statement above. We also know this must be valid utf8, both because of read_hexnumber's behavior
        // and because input to the lexer must be valid utf8
        let digits_str = unsafe {
            debug_assert!(self.source.as_bytes().get(start..self.position).is_some());
            debug_assert!(
                std::str::from_utf8(self.source.as_bytes().get_unchecked(start..self.position))
                    .is_ok()
            );

            std::str::from_utf8_unchecked(
                self.source.as_bytes().get_unchecked(start..self.position),
            )
        };

        match u32::from_str_radix(digits_str, 16) {
            Ok(digits) if digits <= 0x10_FFFF => {
                let res = std::char::from_u32(digits);
                if let Some(chr) = res {
                    Ok(chr)
                } else {
                    let err = ParseDiagnostic::new(
                        "invalid codepoint for unicode escape",
                        start..self.position,
                    );
                    self.push_diagnostic(err);
                    Err(())
                }
            }

            _ => {
                let err = ParseDiagnostic::new(
                    "out of bounds codepoint for unicode codepoint escape sequence",
                    start..self.position,
                )
                .with_hint("Codepoints range from 0 to 0x10FFFF (1114111)");
                self.push_diagnostic(err);
                Err(())
            }
        }
    }

    /// Reads a `\u0000` escape sequence.
    ///
    /// This expects the current char to be the `u`. Afterwards, the current
    /// char is the last hex digit.
    ///
    /// This returns a `u32` since not all escape sequences produce valid
    /// Unicode characters.
    fn read_unicode_escape(&mut self) -> Result<u32, ()> {
        self.assert_byte(b'u');

        for _ in 0..4 {
            match self.next_byte_bounded() {
                None => {
                    let err = invalid_digits_after_unicode_escape_sequence(
                        self.position - 1,
                        self.position + 1,
                    );
                    self.push_diagnostic(err);
                    return Err(());
                }
                Some(b) if !b.is_ascii_hexdigit() => {
                    let err = invalid_digits_after_unicode_escape_sequence(
                        self.position - 1,
                        self.position + 1,
                    );
                    self.push_diagnostic(err);
                    return Err(());
                }
                _ => {}
            }
        }

        // Safety: input to the lexer is guaranteed to be valid utf8 and so is
        // the range since we return if there is a wrong amount of digits
        // beforehand.
        let digits_str = unsafe {
            std::str::from_utf8_unchecked(
                self.source
                    .as_bytes()
                    .get_unchecked((self.position - 3)..(self.position + 1)),
            )
        };
        if let Ok(digits) = u32::from_str_radix(digits_str, 16) {
            Ok(digits)
        } else {
            // Safety: we know this is unreachable because 4 hexdigits cannot
            // make an out of bounds char, and we make sure that the chars are
            // actually hex digits.
            unsafe { core::hint::unreachable_unchecked() };
        }
    }

    // Validate a `\x00 escape sequence, this expects the current char to be the `x`, it also does not skip over the escape sequence
    // The pos after this method is the last hex digit
    fn validate_hex_escape(&mut self) -> bool {
        self.assert_byte(b'x');

        let diagnostic = ParseDiagnostic::new(
            "invalid digits after hex escape sequence",
            (self.position - 1)..(self.position + 1),
        )
        .with_hint("Expected 2 hex digits following this");

        for _ in 0..2 {
            match self.next_byte_bounded() {
                None => {
                    self.push_diagnostic(diagnostic);
                    return false;
                }
                Some(b) if !b.is_ascii_hexdigit() => {
                    self.push_diagnostic(diagnostic);
                    return false;
                }
                _ => {}
            }
        }

        true
    }

    /// Consume a `\..` escape sequence.
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_escape_sequence(&mut self) -> bool {
        self.assert_current_char_boundary();
        self.assert_byte(b'\\');
        let cur = self.position;
        self.advance(1); // eats '\'

        if let Some(chr) = self.current_byte() {
            match chr {
                b'\\' | b'n' | b'r' | b't' | b'b' | b'v' | b'f' | b'\'' | b'"' => {
                    self.advance(1);
                    true
                }
                b'u' if self.peek_byte() == Some(b'{') => {
                    self.advance(1); // eats '{'
                    self.read_codepoint_escape_char().is_ok()
                }
                b'u' => self.read_unicode_escape().is_ok(),
                b'x' => self.validate_hex_escape(),
                b'\r' => {
                    if let Some(b'\n') = self.next_byte() {
                        self.advance(1);
                    }
                    true
                }
                chr => {
                    self.advance_byte_or_char(chr);
                    true
                }
            }
        } else {
            self.diagnostics
                .push(ParseDiagnostic::new("", cur..cur + 1).with_hint(
                    "expected an escape sequence following a backslash, but found none",
                ));
            false
        }
    }

    // Consume an identifier by recursively consuming IDENTIFIER_PART kind chars
    #[inline]
    fn consume_ident(&mut self) {
        loop {
            if self.next_byte_bounded().is_none() || self.cur_ident_part(false).is_none() {
                break;
            }
        }
    }

    /// Consumes the identifier at the current position, and fills the given buf with the UTF-8
    /// encoded identifier that got consumed.
    ///
    /// Returns the number of bytes written into the buffer, and if any char was escaped.
    /// This method will stop writing into the buffer if the buffer is too small to
    /// fit the whole identifier.
    #[inline]
    fn consume_and_get_ident(&mut self, buf: &mut [u8]) -> usize {
        let mut idx = 0;
        while self.next_byte_bounded().is_some() {
            if let Some(c) = self.cur_ident_part(false) {
                if let Some(buf) = buf.get_mut(idx..idx + 4) {
                    let res = c.encode_utf8(buf);
                    idx += res.len();
                }
            } else {
                return idx;
            }
        }

        idx
    }

    /// Consume a string literal and advance the lexer, and returning a list of errors that occurred when reading the string
    /// This could include unterminated string and invalid escape sequences
    ///
    /// ## Safety
    /// Must be called at a valid UT8 char boundary
    fn consume_str_literal(&mut self) -> bool {
        self.assert_current_char_boundary();
        let quote = unsafe { self.current_unchecked() };
        let start = self.position;
        let mut valid = true;

        self.advance(1); // eats the start quote
        while let Some(chr) = self.current_byte() {
            match chr {
                b'\\' => {
                    valid &= self.consume_escape_sequence();
                }
                chr if chr == quote => {
                    self.advance(1);
                    return valid;
                }
                chr => {
                    if chr.is_ascii() {
                        self.advance(1);
                    } else {
                        self.advance_char_unchecked();
                    }
                }
            }
        }

        let unterminated =
            ParseDiagnostic::new("unterminated string literal", self.position..self.position)
                .with_detail(self.position..self.position, "input ends here")
                .with_detail(start..start + 1, "string literal starts here");
        self.push_diagnostic(unterminated);

        false
    }

    /// Returns `Some(x)` if the current position is an identifier, with the character at
    /// the position.
    ///
    /// Boolean states if there are escaped characters.
    ///
    /// The character may be a char that was generated from a unicode escape sequence,
    /// e.g. `t` is returned, the actual source code is `\u{74}`
    #[inline]
    fn cur_ident_part(&mut self, start_with_quote: bool) -> Option<char> {
        debug_assert!(!self.is_eof());

        // Safety: we always call this method on a char
        let b = unsafe { self.current_unchecked() };

        match lookup_byte(b) {
            IDT | DOL | DIG | ZER | AT_ => Some(b as char),
            UNI => {
                let chr = self.current_char_unchecked();
                let res = is_id_continue(chr);
                if res {
                    self.advance(chr.len_utf8() - 1);
                    Some(chr)
                } else {
                    None
                }
            }
            QOT => None,
            _ => {
                if start_with_quote && b.is_ascii() {
                    return Some(b as char);
                }
                None
            }
        }
    }

    /// Returns the identifier token at the current position, or the keyword token if
    /// the identifier is a keyword.
    ///
    /// `first` is a pair of a character that was already consumed,
    /// but is still part of the identifier, and the characters position.
    #[inline]
    fn resolve_identifier(&mut self, first: char) -> MSyntaxKind {
        use MSyntaxKind::*;

        // Note to keep the buffer large enough to fit every possible keyword that
        // the lexer can return
        let mut buf = [0u8; 36];
        let len = first.encode_utf8(&mut buf).len();
        let count = self.consume_and_get_ident(&mut buf[len..]);

        if self.current_byte() == Some(b'\\') {
            let backslash = ParseDiagnostic::new("syntax error", self.position..self.position)
                .with_detail(
                    self.position..self.position,
                    "eof was expected, not backslash",
                );
            self.push_diagnostic(backslash);
            self.consume_ident();
            return ERROR_TOKEN;
        }

        match std::str::from_utf8(&buf[..count + len]) {
            Ok(s) => match s.to_lowercase().as_str() {
                "and" | "и" => AND_KW,
                "break" | "прервать" => BREAK_KW,
                "case" | "выбор" => CASE_KW,
                "catch" | "исключение" | "перехват" => CATCH_KW,
                "class" | "класс" => CLASS_KW,
                "constructor" => CONSTRUCTOR_KW,
                "continue" | "продолжить" => CONTINUE_KW,
                "debug" | "отладить" => DEBUG_KW,
                "delete" | "удалить" => DELETE_KW,
                "else" | "иначе" => ELSE_KW,
                "extends" | "расширяет" => EXTENDS_KW,
                "false" | "ложь" => FALSE_KW,
                "finally" | "заключение" => FINALLY_KW,
                "for" | "для" => FOR_KW,
                "forall" | "длявсех" => FORALL_KW,
                "func" | "функция" => FUNCTION_KW,
                "get" | "получить" => GET_KW,
                "if" | "если" => IF_KW,
                "in" => IN_KW,
                "в" => IN2_KW,
                "входитв" => INCLUDE_IN_KW,
                "new" | "новый" => NEW_KW,
                "null" | "nil" | "нуль" => NULL_KW,
                "return" | "вернуть" => RETURN_KW,
                "super" | "базовый" => SUPER_KW,
                "switch" | "выборпо" => SWITCH_KW,
                "set" | "установить" => SET_KW,
                "static" => STATIC_KW,
                "or" | "или" => OR_KW,
                "this" | "этот" => THIS_KW,
                "throw" | "вызватьисключение" => THROW_KW,
                "try" | "попытка" => TRY_KW,
                "true" | "истина" => TRUE_KW,
                "while" | "пока" => WHILE_KW,
                "var" | "перем" => VAR_KW,
                "k" | "к" => K_KW,
                _ => T![ident],
            },
            Err(_) => ERROR_TOKEN,
        }
    }

    #[inline]
    fn resolve_long_identifier(&mut self) -> MSyntaxKind {
        use MSyntaxKind::*;

        let start = self.position;

        loop {
            if self.next_byte_bounded().is_none() || self.cur_ident_part(true).is_none() {
                break;
            }
        }

        // check unterminated quote
        if self.current_byte() != Some(b'\'') {
            let unterminated =
                ParseDiagnostic::new("unterminated identifier", self.position..self.position)
                    .with_detail(self.position..self.position, "input ends here")
                    .with_detail(start..start + 1, "identifier starts here");
            self.push_diagnostic(unterminated);
            return ERROR_TOKEN;
        }
        self.next_byte_bounded();

        T![ident]
    }

    #[inline]
    fn resolve_report_identifier(&mut self) -> MSyntaxKind {
        loop {
            if self.next_byte_bounded().is_none()
                || self.cur_ident_part(true).is_none()
                || self.current_byte() == Some(b'#')
            {
                break;
            }
            if let Some(byte) = self.current_byte() {
                if is_linebreak(byte as char) {
                    break;
                }
            }
        }
        self.after_ff = false;
        T![ident]
    }

    #[inline]
    fn special_number_start<F: Fn(char) -> bool>(&mut self, func: F) -> bool {
        if self.byte_at(2).is_some_and(|b| func(b as char)) {
            self.advance(1);
            true
        } else {
            false
        }
    }

    #[inline]
    fn read_hexnumber(&mut self) {
        while let Some(byte) = self.next_byte() {
            match byte {
                b'_' => self.handle_numeric_separator(16),
                b if char::from(b).is_ascii_hexdigit() => {}
                _ => break,
            }
        }
    }

    #[inline]
    fn handle_numeric_separator(&mut self, radix: u8) {
        self.assert_byte(b'_');

        let err_diag = ParseDiagnostic::new(
            "numeric separators are only allowed between two digits",
            self.position..self.position + 1,
        );

        let peeked = self.peek_byte();

        if peeked.is_none() || !char::from(peeked.unwrap()).is_digit(u32::from(radix)) {
            self.push_diagnostic(err_diag);
            return;
        }

        let forbidden = |c: Option<u8>| {
            if c.is_none() {
                return true;
            }
            let c = c.unwrap();

            if radix == 16 {
                matches!(c, b'.' | b'X' | b'_' | b'x')
            } else {
                matches!(c, b'.' | b'B' | b'E' | b'O' | b'_' | b'b' | b'e' | b'o')
            }
        };

        let prev = self.source.as_bytes().get(self.position - 1).copied();

        if forbidden(prev) || forbidden(peeked) {
            self.push_diagnostic(err_diag);
            return;
        }

        self.next_byte_bounded();
    }

    #[inline]
    fn resolve_number(&mut self) -> MSyntaxKind {
        let leading_zero = self.peek_byte() == Some(b'0');
        let start = self.position;
        let mut checkpoint = None;
        loop {
            match self.next_byte_bounded() {
                Some(b'0') if leading_zero => {
                    if let Some(b'x' | b'X') = self.peek_byte() {
                        if self.special_number_start(|c| c.is_ascii_hexdigit()) {
                            self.read_hexnumber();
                        } else {
                            self.next_byte();
                        }
                    }
                }
                Some(b'0'..=b'9') => {}
                Some(b'.') => {
                    checkpoint = Some(self.checkpoint());
                    if self.position - start == 2 && self.byte_at(3) == Some(b'.') {
                        // try parse date - or return number
                        if self.read_date().is_none() {
                            self.rewind(checkpoint.unwrap());
                            return M_NUMBER_LITERAL;
                        } else {
                            return M_DATE_LITERAL;
                        }
                    } else {
                        self.read_float();
                    }
                    break;
                }
                Some(b':') => {
                    if self.position - start != 2 {
                        break;
                    }

                    if self.read_time().is_none() {
                        return M_NUMBER_LITERAL;
                    } else {
                        return M_TIME_LITERAL;
                    }
                }
                Some(b'e' | b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-' | b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
                                self.read_exponent();
                                break;
                            } else {
                                break;
                            }
                        }
                        Some(b'0'..=b'9') => {
                            self.read_exponent();
                            break;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Some(b'_') => {
                    if Some(b'i' | b'I') == self.peek_byte() {
                        self.next_byte();
                        // 1_i64
                        if Some(b'6') == self.peek_byte() && Some(b'4') == self.byte_at(2) {
                            self.advance(3);
                            break;
                        }
                        // 1_i32
                        if Some(b'3') == self.peek_byte() && Some(b'2') == self.byte_at(2) {
                            self.advance(3);
                            break;
                        }
                        break;
                    }
                    break;
                }
                Some(b'i' | b'I') => {
                    self.next_byte();
                    if self.cur_is_ws() {
                        break;
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        if self.is_eof() || self.cur_is_ws() {
            return M_NUMBER_LITERAL;
        }

        if self.current_byte() == Some(b'\\') {
            let err_start = self.position;
            self.consume_ident();
            let err = ParseDiagnostic::new(
                "numbers cannot be followed by backslash",
                err_start..self.position,
            );

            self.push_diagnostic(err);
            return MSyntaxKind::ERROR_TOKEN;
        }

        if self.cur_ident_part(false).is_some() {
            // We have identifier chaining after number
            if let Some(checkpoint) = checkpoint {
                self.rewind(checkpoint);
                return M_NUMBER_LITERAL;
            }

            self.consume_ident();
            T![ident]
        } else {
            M_NUMBER_LITERAL
        }
    }

    #[inline]
    fn read_float(&mut self) {
        loop {
            match self.next_byte_bounded() {
                // LLVM has a hard time optimizing inclusive patterns, perhaps we should check if it makes llvm sad,
                // and optimize this into a lookup table
                Some(b'0'..=b'9') => {}
                Some(b'e' | b'E') => {
                    // At least one digit is required
                    match self.peek_byte() {
                        Some(b'-' | b'+') => {
                            if let Some(b'0'..=b'9') = self.byte_at(2) {
                                self.next_byte();
                                self.read_exponent();
                                return;
                            } else {
                                return;
                            }
                        }
                        Some(b'0'..=b'9') => {
                            self.read_exponent();
                            return;
                        }
                        _ => {
                            return;
                        }
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    #[inline]
    fn read_date(&mut self) -> Option<()> {
        let (d1, d2) = self.prev_byte(2).zip(self.prev_byte(1))?;
        let (m1, m2) = self.byte_at(1).zip(self.byte_at(2))?;

        let (mut y1, mut y2) = (b'0', b'0');
        let (y3, y4) = self.byte_at(4).zip(self.byte_at(5))?;

        let mut size = 6;
        if let Some(years) = self.byte_at(6).zip(self.byte_at(7)) {
            (y1, y2) = years;
            size = 8;
        }

        // check - all are digits
        let bytes = [d1, d2, m1, m2, y1, y2, y3, y4];
        if !bytes.iter().all(|b| b.is_ascii_digit()) {
            return None;
        }

        // all are zeroes
        let chars = bytes.map(|b| b as char);
        if chars.iter().all(|c| *c == '0') {
            self.advance(size);
            return Some(()); // null date literal
        }

        let [d1, d2, m1, m2, y1, y2, y3, y4] = chars;
        let size =
            NaiveDate::parse_from_str(&format!("{d1}{d2}.{m1}{m2}.{y1}{y2}{y3}{y4}"), "%d.%m.%Y")
                .and(Ok(size))
                .ok();

        if let Some(size) = size {
            self.advance(size);
            Some(())
        } else {
            None
        }
    }

    fn read_time(&mut self) -> Option<()> {
        let (h1, h2) = self.prev_byte(2).zip(self.prev_byte(1))?;
        let (m1, m2) = self.byte_at(1).zip(self.byte_at(2))?;
        let (mut s1, mut s2) = (b'0', b'0');

        let mut size = 3;
        if self.byte_at(3) == Some(b':') {
            if let Some(seconds) = self.byte_at(4).zip(self.byte_at(5)) {
                (s1, s2) = seconds;
                size = 6;
            }
        }

        // check - all are digits
        let bytes = [h1, h2, m1, m2, s1, s2];
        if !bytes.iter().all(|b| b.is_ascii_digit()) {
            return None;
        }

        // all are zeroes
        let chars = bytes.map(|b| b as char);
        if chars.iter().all(|c| *c == '0') {
            self.advance(size);
            return Some(()); // null time literal
        }

        if chars[..2] == ['2', '4'] && chars[2..].iter().all(|c| *c == '0') {
            self.advance(size);
            return Some(()); // 24 hours like 24:00 or 24:00:00
        }

        let [h1, h2, m1, m2, s1, s2] = chars;
        let size = NaiveTime::parse_from_str(&format!("{h1}{h2}:{m1}{m2}:{s1}{s2}"), "%H:%M:%S")
            .and(Ok(size))
            .or_else(|_| {
                // try parse without seconds
                NaiveTime::parse_from_str(&format!("{h1}{h2}:{m1}{m2}"), "%H:%M").and(Ok(size - 3))
            })
            .ok();

        if let Some(size) = size {
            self.advance(size);
            Some(())
        } else {
            None
        }
    }

    #[inline]
    fn read_exponent(&mut self) {
        if let Some(b'-' | b'+') = self.peek_byte() {
            self.next_byte();
        }

        loop {
            match self.next_byte() {
                Some(b'0'..=b'9') => {}
                _ => {
                    return;
                }
            }
        }
    }

    #[inline]
    fn cur_is_ws(&mut self) -> bool {
        matches!(self.current_byte(), Some(b'\t' | b'\n' | b'\r' | b' '))
    }

    #[inline]
    fn read_slash(&mut self) -> MSyntaxKind {
        match self.peek_byte() {
            Some(b'=') => {
                self.advance(2); // eats /=
                SLASHEQ
            }
            _ => self.eat_byte(T![/]),
        }
    }

    #[inline]
    fn read_hash(&mut self) -> MSyntaxKind {
        self.advance(1); // eats #
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

    #[inline]
    fn bin_or_assign(&mut self, bin: MSyntaxKind, assign: MSyntaxKind) -> MSyntaxKind {
        if let Some(b'=') = self.next_byte() {
            self.next_byte();
            assign
        } else {
            bin
        }
    }

    #[inline]
    fn resolve_bang(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                NEQ
            }
            _ => T![!],
        }
    }

    #[inline]
    fn resolve_amp(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'&') => {
                self.next_byte();
                AMP2
            }
            _ => T![&],
        }
    }

    #[inline]
    fn resolve_plus(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'+') => {
                self.next_byte();
                PLUS2
            }
            Some(b'=') => {
                self.next_byte();
                PLUSEQ
            }
            _ => T![+],
        }
    }

    #[inline]
    fn resolve_minus(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'-') => {
                self.next_byte();
                MINUS2
            }
            Some(b'=') => {
                self.next_byte();
                MINUSEQ
            }
            _ => T![-],
        }
    }

    #[inline]
    fn resolve_less_than(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                LTEQ
            }
            _ => T![<],
        }
    }

    #[inline]
    fn resolve_eq(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                EQ2
            }
            _ => T![=],
        }
    }

    #[inline]
    fn resolve_pipe(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'|') => {
                self.next_byte();
                PIPE2
            }
            _ => T![|],
        }
    }

    #[inline]
    fn resolve_star(&mut self) -> MSyntaxKind {
        match self.next_byte() {
            Some(b'=') => {
                self.next_byte();
                STAREQ
            }
            _ => T![*],
        }
    }

    /// Lex the next token
    fn lex_token(&mut self) -> MSyntaxKind {
        // Safety: we always call lex_token when we are at a valid char
        let byte = unsafe { self.current_unchecked() };
        let start = self.position;

        // A lookup table of `byte -> fn(l: &mut Lexer) -> Token` is exponentially slower than this approach
        // The speed difference comes from the difference in table size, a 2kb table is easily fit into cpu cache
        // While a 16kb table will be ejected from cache very often leading to slowdowns, this also allows LLVM
        // to do more aggressive optimizations on the match regarding how to map it to instructions
        let dispatched = lookup_byte(byte);
        match dispatched {
            WHS => {
                let kind = match byte {
                    12 => self.consume_ff(),
                    _ => self.consume_newline_or_whitespaces(),
                };

                kind
            }
            EXL => self.resolve_bang(),
            PRC => self.bin_or_assign(T![%], T![%=]),
            Dispatch::AMP => self.resolve_amp(),
            PNO => self.eat_byte(T!['(']),
            PNC => self.eat_byte(T![')']),
            MUL => self.resolve_star(),
            PLS => self.resolve_plus(),
            COM => self.eat_byte(T![,]),
            MIN => self.resolve_minus(),
            SLH => self.read_slash(),
            HAS => self.read_hash(),
            PRD => {
                if self.peek_byte() == Some(b'.') && self.byte_at(2) == Some(b'.') {
                    self.advance(3);
                    return DOT3;
                }
                self.eat_byte(T![.])
            }
            QOT => match byte {
                b'\'' => self.resolve_long_identifier(),
                _ => {
                    if self.consume_str_literal() {
                        M_STRING_LITERAL
                    } else {
                        ERROR_TOKEN
                    }
                }
            },
            TPL => {
                if self.consume_str_literal() {
                    M_LONG_STRING_LITERAL
                } else {
                    ERROR_TOKEN
                }
            }
            IDT | DIG | ZER if self.after_ff => self.resolve_report_identifier(),
            IDT | DOL => self.resolve_identifier(byte as char),
            DIG | ZER => self.resolve_number(),
            COL => self.eat_byte(T![:]),
            SEM => self.eat_byte(T![;]),
            LSS => self.resolve_less_than(),
            EQL => self.resolve_eq(),
            // `>>`, `>=` etc handled by `ReLex::BinaryOperator`
            MOR => self.eat_byte(T![>]),
            QST => self.eat_byte(T![?]),
            BTO => self.eat_byte(T!('[')),
            BTC => self.eat_byte(T![']']),
            CRT => self.eat_byte(T![^]),
            BEO => self.eat_byte(T!['{']),
            BEC => self.eat_byte(T!['}']),
            PIP => self.resolve_pipe(),

            UNI => {
                // A BOM can only appear at the start of a file, so if we haven't advanced at all yet,
                // perform the check. At any other position, the BOM is just considered plain whitespace.
                if self.position == 0 {
                    if let Some((bom, bom_size)) = self.consume_potential_bom(UNICODE_BOM) {
                        self.unicode_bom_length = bom_size;
                        return bom;
                    }
                }

                let chr = self.current_char_unchecked();
                if is_linebreak(chr)
                    || (UNICODE_WHITESPACE_STARTS.contains(&byte) && UNICODE_SPACES.contains(&chr))
                {
                    self.consume_newline_or_whitespaces()
                } else {
                    self.advance(chr.len_utf8() - 1);
                    if is_id_start(chr) {
                        if self.after_ff {
                            self.resolve_report_identifier()
                        } else {
                            self.resolve_identifier(chr)
                        }
                    } else {
                        let err = ParseDiagnostic::new(
                            format!("Unexpected token `{chr}`"),
                            start..self.position + 1,
                        );
                        self.push_diagnostic(err);
                        self.next_byte();

                        MSyntaxKind::ERROR_TOKEN
                    }
                }
            }
            AT_ => {
                let next_chr = self.peek_byte();
                if let Some(next_byte) = next_chr {
                    // if it start of collections - then eat it
                    if matches!(next_byte, b'[' | b'{' | b'(') {
                        self.eat_byte(T![@])
                    } else {
                        let chr = self.current_char_unchecked();
                        self.resolve_identifier(chr)
                    }
                } else {
                    self.eat_byte(T![@])
                }
            }
            _ => {
                let err = ParseDiagnostic::new(
                    format!("unexpected token `{}`", byte as char),
                    start..self.position + 1,
                );
                self.push_diagnostic(err);
                self.next_byte();

                MSyntaxKind::ERROR_TOKEN
            }
        }
    }
}

/// Check if a char is a linebreak
fn is_linebreak(chr: char) -> bool {
    matches!(chr, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

fn is_id_continue(chr: char) -> bool {
    chr == '@' || is_js_id_continue(chr)
}

fn is_id_start(chr: char) -> bool {
    chr == '@' || is_js_id_start(chr)
}
