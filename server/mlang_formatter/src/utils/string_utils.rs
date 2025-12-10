use crate::prelude::*;

use mlang_syntax::MSyntaxKind::{M_LONG_STRING_LITERAL, M_STRING_LITERAL};
use mlang_syntax::MSyntaxToken;
use biome_formatter::{format_args, write};
use std::borrow::Cow;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variant to track tokens that are inside an expression
    Expression,
    /// Variant to track tokens that are inside a member
    Member,
}

/// Data structure of convenience to format string literals
pub(crate) struct FormatLiteralStringToken<'token> {
    token: &'token MSyntaxToken,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token MSyntaxToken, _parent_kind: StringLiteralParentKind) -> Self {
        debug_assert!(
            matches!(token.kind(), M_STRING_LITERAL | M_LONG_STRING_LITERAL),
            "Found kind {:?}",
            token.kind()
        );

        Self { token }
    }

    fn token(&self) -> &'token MSyntaxToken {
        self.token
    }

    pub fn clean_text(&self) -> CleanedStringLiteralText<'_> {
        let token = self.token();

        let content = token.text_trimmed();
        let preferred_quote = content.chars().next().unwrap_or('"');

        let mut string_cleaner = LiteralStringNormaliser::new(token, preferred_quote);

        let content = string_cleaner.normalise_text();
        let normalized_text_width = content.len();

        CleanedStringLiteralText {
            text: content,
            width: normalized_text_width,
            token,
        }
    }
}

pub(crate) struct CleanedStringLiteralText<'a> {
    token: &'a MSyntaxToken,
    text: Cow<'a, str>,
    width: usize,
}

impl CleanedStringLiteralText<'_> {
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Format<MFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl Format<MFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let cleaned = self.clean_text();

        cleaned.fmt(f)
    }
}

/// Struct of convenience used to manipulate the string. It saves some state in order to apply
/// the normalise process.
struct LiteralStringNormaliser<'token> {
    /// The current token
    token: &'token MSyntaxToken,
    preferred_quote: char,
}

impl<'token> LiteralStringNormaliser<'token> {
    pub fn new(token: &'token MSyntaxToken, preferred_quote: char) -> Self {
        Self {
            token,
            preferred_quote,
        }
    }

    fn normalise_text(&mut self) -> Cow<'token, str> {
        self.normalise_string_literal()
    }

    fn normalise_string_literal(&mut self) -> Cow<'token, str> {
        let polished_raw_content = self.normalize_string();
        let preferred_quote = self.preferred_quote;

        match polished_raw_content {
            Cow::Borrowed(raw_content) => self.swap_quotes(raw_content, preferred_quote),
            Cow::Owned(mut s) => {
                // content is owned, meaning we allocated a new string,
                // so we force replacing quotes, regardless
                s.insert(0, preferred_quote);
                s.push(preferred_quote);
                Cow::Owned(s)
            }
        }
    }

    fn normalize_string(&self) -> Cow<'token, str> {
        let raw_content = self.raw_content();
        let mut reduced_string = String::new();
        let mut copy_start = 0;
        let mut bytes = raw_content.bytes().enumerate();

        while let Some((byte_index, byte)) = bytes.next() {
            match byte {
                // If the next character is escaped
                b'\\' => {
                    if let Some((escaped_index, escaped)) = bytes.next()
                        && escaped == b'\r' {
                            // If we encounter the sequence "\r\n", then skip '\r'
                            if let Some((next_byte_index, b'\n')) = bytes.next() {
                                reduced_string.push_str(&raw_content[copy_start..escaped_index]);
                                copy_start = next_byte_index;
                            }
                        }
                }
                // If we encounter the sequence "\r\n", then skip '\r'
                b'\r' => {
                    if let Some((next_byte_index, b'\n')) = bytes.next() {
                        reduced_string.push_str(&raw_content[copy_start..byte_index]);
                        copy_start = next_byte_index;
                    }
                }
                _ => {}
            }
        }
        if copy_start == 0 && reduced_string.is_empty() {
            Cow::Borrowed(raw_content)
        } else {
            // Copy the remaining characters
            reduced_string.push_str(&raw_content[copy_start..]);
            Cow::Owned(reduced_string)
        }
    }

    /// Returns the string without its quotes.
    fn raw_content(&self) -> &'token str {
        let content = self.token.text_trimmed();
        &content[1..content.len() - 1]
    }

    fn swap_quotes(&self, content_to_use: &'token str, preferred_quote: char) -> Cow<'token, str> {
        let original = self.token.text_trimmed();

        if original.starts_with(preferred_quote) {
            Cow::Borrowed(original)
        } else {
            Cow::Owned(std::format!(
                "{preferred_quote}{content_to_use}{preferred_quote}",
            ))
        }
    }
}

pub(crate) struct FormatSqlStringToken<'token> {
    token: &'token MSyntaxToken,
}

impl<'token> FormatSqlStringToken<'token> {
    pub fn new(token: &'token MSyntaxToken) -> Self {
        Self { token }
    }

    fn token(&self) -> &'token MSyntaxToken {
        self.token
    }

    fn format_single_line_query(
        &self,
        content: Cow<'token, str>,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        write!(
            f,
            [format_replaced(
                self.token,
                &syntax_token_cow_slice(
                    content,
                    self.token,
                    self.token.text_trimmed_range().start()
                ),
            ),]
        )
    }

    fn format_multi_line_query(
        &self,
        content: Cow<'token, str>,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let start = self.token.text_trimmed_range().start();
        let quoteless = &content[1..content.len() - 1];

        // if query starts with newline - write it as is
        if quoteless.trim_start_matches([' ', '\t']).starts_with("\n") {
            return write!(
                f,
                [format_replaced(
                    self.token,
                    &syntax_token_cow_slice(content, self.token, start,),
                )]
            );
        }

        let quoteless = quoteless.trim_start_matches('\n').trim_end();

        write!(
            f,
            [format_replaced(
                self.token,
                &format_args![
                    text("`"),
                    &format_with(move |f| {
                        for (index, line) in quoteless.lines().enumerate() {
                            match index {
                                // write on new line with indent
                                0 if !line.starts_with(' ') => write!(
                                    f,
                                    [block_indent(&format_args![
                                        dynamic_text(line, start),
                                        hard_line_break(),
                                    ])]
                                )?,

                                // write on new line with dedent of parent
                                0 => write!(
                                    f,
                                    [dedent(&format_args![
                                        hard_line_break(),
                                        dynamic_text(line, start),
                                        hard_line_break(),
                                    ]),]
                                )?,

                                _ => write!(f, [dynamic_text(line, start), hard_line_break(),])?,
                            }
                        }
                        Ok(())
                    }),
                    text("`")
                ]
            )]
        )
    }
}

impl Format<MFormatContext> for FormatSqlStringToken<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let token = self.token();
        let preferred_quote = '`';

        let mut string_cleaner = LiteralStringNormaliser::new(token, preferred_quote);

        let content = string_cleaner.normalise_text();
        if content.lines().count() > 1 {
            self.format_multi_line_query(content, f)
        } else {
            self.format_single_line_query(content, f)
        }
    }
}
