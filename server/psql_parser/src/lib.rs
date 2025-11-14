use std::marker::PhantomData;

use biome_parser::{
    AnyParse, ParserContext, ParserContextCheckpoint, event::Event, prelude::*,
    tree_sink::LosslessTreeSink,
};
use biome_rowan::{AstNode, NodeCache};
use psql_factory::PsqlSyntaxFactory;
use psql_syntax::{PsqlFileSource, PsqlLanguage, PsqlRoot, PsqlSyntaxKind, PsqlSyntaxNode};

use crate::{
    lexer::PsqlReLexContext,
    state::{ChangeParserState, PsqlParserState, PsqlParserStateCheckpoint},
    token_source::{PsqlTokenSource, PsqlTokenSourceCheckpoint},
};

mod lexer;
mod state;
mod syntax_rules;
mod token_source;

pub type PsqlLosslessTreeSink<'source> = LosslessTreeSink<'source, PsqlLanguage, PsqlSyntaxFactory>;
pub struct PsqlParser<'source> {
    pub(self) state: PsqlParserState,
    pub source_type: PsqlFileSource,
    context: ParserContext<PsqlSyntaxKind>,
    source: PsqlTokenSource<'source>,
}

impl<'source> PsqlParser<'source> {
    pub fn new(source: &'source str, source_type: PsqlFileSource) -> Self {
        Self {
            state: PsqlParserState::new(),
            source_type,
            context: ParserContext::default(),
            source: PsqlTokenSource::from_str(source),
        }
    }

    pub(self) fn state(&self) -> &PsqlParserState {
        &self.state
    }

    pub(self) fn state_mut(&mut self) -> &mut PsqlParserState {
        &mut self.state
    }

    pub fn source_type(&self) -> PsqlFileSource {
        self.source_type
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: PsqlReLexContext) -> PsqlSyntaxKind {
        self.source_mut().re_lex(context)
    }

    /// Stores the parser state and position before calling the function and restores the state
    /// and position before returning.
    ///
    /// Useful in situation where the parser must advance a few tokens to determine whatever a syntax is
    /// of one or the other kind.
    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut PsqlParser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);

        result
    }

    /// Applies the passed in change to the parser state before applying the passed `func` and
    /// restores the state to before the change before returning the result.
    #[inline]
    pub(self) fn with_state<C, F, R>(&mut self, change: C, func: F) -> R
    where
        C: ChangeParserState,
        F: FnOnce(&mut PsqlParser) -> R,
    {
        let snapshot = change.apply(self.state_mut());
        let result = func(self);
        C::restore(self.state_mut(), snapshot);
        result
    }

    pub fn checkpoint(&self) -> PsqlParserCheckpoint {
        PsqlParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            state: self.state.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: PsqlParserCheckpoint) {
        let PsqlParserCheckpoint {
            context,
            source,
            state,
        } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        self.state.restore(state);
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<PsqlSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, _) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        (events, parse_diagnostics, trivia)
    }
}

impl<'source> Parser for PsqlParser<'source> {
    type Kind = PsqlSyntaxKind;
    type Source = PsqlTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

pub struct PsqlParserCheckpoint {
    pub context: ParserContextCheckpoint,
    pub source: PsqlTokenSourceCheckpoint,
    state: PsqlParserStateCheckpoint,
}

#[derive(Debug)]
pub struct Parse<T> {
    root: PsqlSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: PsqlSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = PsqlLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> PsqlSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        self.errors.as_slice()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|diagnostic| diagnostic.is_error())
    }
}

impl<T: AstNode<Language = PsqlLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn ok(self) -> Result<T, Vec<ParseDiagnostic>> {
        if !self.errors.iter().any(|d| d.is_error()) {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<Parse<T>> for AnyParse {
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

pub fn parse(text: &str, source_type: PsqlFileSource) -> Parse<PsqlRoot> {
    let mut cache = NodeCache::default();
    parse_with_cache(text, source_type, &mut cache)
}

fn parse_with_cache(
    text: &str,
    source_type: PsqlFileSource,
    cache: &mut NodeCache,
) -> Parse<PsqlRoot> {
    let (events, errors, tokens) = parse_common(text, source_type);
    let mut tree_sink = PsqlLosslessTreeSink::with_cache(text, &tokens, cache);
    biome_parser::event::process(&mut tree_sink, events, errors);
    let (green, parse_errors) = tree_sink.finish();
    Parse::new(green, parse_errors)
}

fn parse_common(
    text: &str,
    source_type: PsqlFileSource,
) -> (
    Vec<Event<PsqlSyntaxKind>>,
    Vec<ParseDiagnostic>,
    Vec<Trivia>,
) {
    let mut p = PsqlParser::new(text, source_type);
    syntax_rules::program::parse(&mut p);

    let (events, errors, trivia) = p.finish();

    (events, errors, trivia)
}
