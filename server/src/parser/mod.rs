mod decl;
mod expr;
mod stmt;

use std::iter::Map;
use std::ops::Range;

use chumsky::{
    input::{SpannedInput, Stream},
    prelude::*,
};

use crate::lexer::KwLang;
use crate::lexer::Token;
use logos::{Logos, SpannedIter};

use self::decl::{parser_decl, Decl};

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

fn token_stream_from_str<'source>(
    source: &'source str,
) -> SpannedInput<
    Token<'source>,
    SimpleSpan,
    Stream<
        Map<
            SpannedIter<Token<'source>>,
            impl FnMut((Result<Token<'source>, ()>, Range<usize>)) -> (Token<'source>, SimpleSpan),
        >,
    >,
> {
    let token_iter = Token::lexer(source).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, span.into()),
        Err(()) => (Token::Error, span.into()),
    });

    Stream::from_iter(token_iter).spanned((source.len()..source.len()).into())
}

pub fn parse<'source>(
    source: &'source str,
) -> ParseResult<Vec<Decl>, chumsky::error::Rich<'_, Token<'_>>> {
    let token_stream = token_stream_from_str(source);

    parser_decl().parse(token_stream)
}
