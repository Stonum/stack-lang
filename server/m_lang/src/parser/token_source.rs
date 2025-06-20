use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::{ParseDiagnostic, TokenSource};
use biome_parser::token_source::{
    BumpWithContext, TokenSourceCheckpoint, TokenSourceWithBufferedLexer, Trivia,
};
use biome_rowan::{TextRange, TextSize, TriviaPieceKind};

use crate::lexer::{MLexContext, MLexer, MReLexContext};
use crate::syntax::MSyntaxKind::{self, EOF};

/// Token source for the parser that skips over any non-trivia token.
pub struct MTokenSource<'l> {
    lexer: BufferedLexer<MSyntaxKind, MLexer<'l>>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia_list: Vec<Trivia>,
}

pub(crate) type MTokenSourceCheckpoint = TokenSourceCheckpoint<MSyntaxKind>;

impl<'l> MTokenSource<'l> {
    /// Creates a new token source.
    pub(crate) fn new(lexer: BufferedLexer<MSyntaxKind, MLexer<'l>>) -> MTokenSource<'l> {
        MTokenSource {
            lexer,
            trivia_list: vec![],
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(source: &'l str) -> MTokenSource<'l> {
        let lexer = MLexer::from_str(source);
        let buffered = BufferedLexer::new(lexer);
        let mut source = MTokenSource::new(buffered);

        source.next_non_trivia_token(MLexContext::default(), true);
        source
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: MLexContext, first_token: bool) {
        let mut trailing = !first_token;

        loop {
            let kind = self.lexer.next_token(context);

            let trivia_kind = TriviaPieceKind::try_from(kind);

            match trivia_kind {
                Err(_) => break,
                Ok(trivia_kind) => {
                    // Trivia after and including the newline is considered the leading trivia of the next token
                    if trivia_kind.is_newline() {
                        trailing = false;
                    }

                    self.trivia_list
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
        }
    }

    #[inline(always)]
    pub fn has_unicode_escape(&self) -> bool {
        self.lexer.has_unicode_escape()
    }

    /// Returns `true` if the next token has any preceding trivia (either trailing trivia of the current
    /// token or leading trivia of the next token)
    pub fn has_next_preceding_trivia(&mut self) -> bool {
        let next_token_trivia = self
            .lexer
            .lookahead_iter()
            .next()
            .and_then(|lookahead| TriviaPieceKind::try_from(lookahead.kind()).ok());
        next_token_trivia.is_some()
    }

    pub fn re_lex(&mut self, mode: MReLexContext) -> MSyntaxKind {
        self.lexer.re_lex(mode)
    }

    /// Creates a checkpoint to which it can later return using [Self::rewind].
    pub fn checkpoint(&self) -> MTokenSourceCheckpoint {
        MTokenSourceCheckpoint {
            trivia_len: self.trivia_list.len() as u32,
            lexer_checkpoint: self.lexer.checkpoint(),
        }
    }

    /// Restores the token source to a previous state
    pub fn rewind(&mut self, checkpoint: MTokenSourceCheckpoint) {
        assert!(self.trivia_list.len() >= checkpoint.trivia_len as usize);
        self.trivia_list.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer_checkpoint);
    }
}

impl<'source> TokenSource for MTokenSource<'source> {
    type Kind = MSyntaxKind;

    /// Returns the kind of the current non-trivia token
    #[inline(always)]
    fn current(&self) -> MSyntaxKind {
        self.lexer.current()
    }

    /// Returns the range of the current non-trivia token
    #[inline(always)]
    fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    #[inline(always)]
    fn text(&self) -> &'source str {
        self.lexer.source()
    }

    #[inline(always)]
    fn position(&self) -> TextSize {
        self.current_range().start()
    }

    /// Returns true if the current token is preceded by a line break
    #[inline(always)]
    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    #[inline(always)]
    fn bump(&mut self) {
        self.bump_with_context(MLexContext)
    }

    fn skip_as_trivia(&mut self) {
        self.skip_as_trivia_with_context(MLexContext)
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia_list, self.lexer.finish())
    }
}

impl BumpWithContext for MTokenSource<'_> {
    type Context = MLexContext;

    #[inline(always)]
    fn bump_with_context(&mut self, context: Self::Context) {
        if self.current() != EOF {
            self.next_non_trivia_token(context, false);
        }
    }

    /// Skips the current token as skipped token trivia
    fn skip_as_trivia_with_context(&mut self, context: MLexContext) {
        if self.current() != EOF {
            self.trivia_list.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(context, true)
        }
    }
}

impl<'source> TokenSourceWithBufferedLexer<MLexer<'source>> for MTokenSource<'source> {
    fn lexer(&mut self) -> &mut BufferedLexer<MSyntaxKind, MLexer<'source>> {
        &mut self.lexer
    }
}
