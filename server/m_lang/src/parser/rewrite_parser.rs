use super::{token_source::MTokenSourceCheckpoint, MParser};
use super::{MSyntaxKind, TextRange};

use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_rowan::TextSize;

/// Simplified parser API for when rewriting the AST structure with `rewrite_events`.
///
/// The difference from the regular [Parser] is that the [TokenSource] must be detached during
/// rewriting to avoid lexing previously lexed tokens in a different context. For example for `a[`test`] = "b"`.
/// Template literal elements get lexed in the [TemplateElement] context. However, if the rewriter
/// rewinds the token source then all tokens are lexed in the [LexContext::Regular] which yields
/// complete different results.
///
/// This is why the [RewriteParser] tracks the source offset without relying on the `TokenSource`
/// and explicitly passes the positions to [Marker] and [CompletedMarker]. This further has the
/// benefit that rewriting the events doesn't require re-lexing all tokens as well.
pub struct RewriteParser<'parser, 'source> {
    /// The byte offset of the current token from the start of the source
    offset: TextSize,

    inner: &'parser mut MParser<'source>,

    /// Offset to the next not yet processed trivia in [TokenSource::trivia_list].
    trivia_offset: usize,
}

impl<'parser, 'source> RewriteParser<'parser, 'source> {
    pub fn new(p: &'parser mut MParser<'source>, checkpoint: MTokenSourceCheckpoint) -> Self {
        Self {
            inner: p,
            offset: checkpoint.current_start(),
            trivia_offset: checkpoint.trivia_position(),
        }
    }

    /// Starts a marker for a new node.
    pub fn start(&mut self) -> RewriteMarker {
        let pos = self.inner.context().events().len() as u32;
        self.skip_trivia(false);
        self.inner.context_mut().push_event(Event::tombstone());
        RewriteMarker(Marker::new(pos, self.offset))
    }

    /// Bumps the passed in token
    pub fn bump(&mut self, token: RewriteToken) {
        self.skip_trivia(false);
        debug_assert!(self.offset < token.end);
        self.inner.context_mut().push_token(token.kind, token.end);

        // test ts ts_decorator_assignment
        // @test(--a)
        // class Test {}

        // If the parser originally skipped this token as trivia, then make sure to also consume the trivia.
        if let Some(trivia) = self.inner.source().trivia_list.get(self.trivia_offset) {
            if trivia.kind().is_skipped() && trivia.offset() == self.offset {
                self.trivia_offset += 1;
            }
        }

        self.offset = token.end;
        self.skip_trivia(true);
    }

    fn skip_trivia(&mut self, trailing: bool) {
        let remaining_trivia = &self.inner.source().trivia_list[self.trivia_offset..];
        for trivia in remaining_trivia {
            // Don't skip over any "skipped token trivia". These get consumed when bumping the token.
            if trailing != trivia.trailing()
                || self.offset != trivia.offset()
                || trivia.kind().is_skipped()
            {
                break;
            }

            self.trivia_offset += 1;
            self.offset += trivia.len();
        }
    }

    /// Finishes the rewriter
    ///
    /// ## Panics
    /// If not all tokens have been consumed or if they have been consumed out of order
    pub fn finish(mut self) {
        self.skip_trivia(false); // Skip the leading trivia up to the current token.
        assert_eq!(
            self.offset,
            self.inner.source().position(),
            "Rewrite didn't consume all tokens"
        );
    }

    pub fn err_builder(&self, message: impl Into<String>, span: TextRange) -> ParseDiagnostic {
        self.inner.err_builder(message.into(), span)
    }

    pub fn error(&mut self, diagnostic: impl ToDiagnostic<MParser<'source>>) {
        self.inner.error(diagnostic)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RewriteToken {
    pub kind: MSyntaxKind,
    end: TextSize,
}

impl RewriteToken {
    pub fn new(kind: MSyntaxKind, end: TextSize) -> Self {
        Self { kind, end }
    }
}

#[derive(Debug)]
pub struct RewriteMarker(Marker);

impl RewriteMarker {
    /// Completes the node with the specified kind
    pub fn complete(self, p: &mut RewriteParser, kind: MSyntaxKind) -> RewriteCompletedMarker {
        RewriteCompletedMarker(self.0.complete(p.inner, kind))
    }
}

#[derive(Debug)]
pub struct RewriteCompletedMarker(CompletedMarker);

impl RewriteCompletedMarker {
    /// Returns the range of the marker
    pub fn range(&self, p: &RewriteParser) -> TextRange {
        self.0.range(p.inner)
    }

    /// Returns the source text of the marker
    pub fn text<'a>(&self, p: &'a RewriteParser) -> &'a str {
        self.0.text(p.inner)
    }

    pub fn change_to_bogus(&mut self, p: &mut RewriteParser) {
        self.0.change_to_bogus(p.inner)
    }
}

impl From<RewriteCompletedMarker> for CompletedMarker {
    fn from(inner: RewriteCompletedMarker) -> Self {
        inner.0
    }
}
