mod lexer;
mod rewrite;
mod rewrite_parser;
mod single_token_parse_recovery;
pub mod span;
mod state;
mod syntax_rules;
pub mod token_source;

use std::marker::PhantomData;

pub use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::event::Event;
use biome_parser::token_source::Trivia;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{AnyParse, Parser, ParserContext};
use biome_parser::{ParserContextCheckpoint, prelude::*};
use biome_rowan::{AstNode, NodeCache};

use mlang_factory::MSyntaxFactory;
use mlang_syntax::{AnyMRoot, MSyntaxNode};
use mlang_syntax::{MFileSource, MLanguage, MSyntaxKind, T, TextRange};

use lexer::MReLexContext;
use state::{ChangeParserState, MParserState, MParserStateCheckpoint};
use token_source::MTokenSource;
use token_source::MTokenSourceCheckpoint;

pub type MLosslessTreeSink<'source> = LosslessTreeSink<'source, MLanguage, MSyntaxFactory>;
pub struct MParser<'source> {
    pub(self) state: MParserState,
    pub source_type: MFileSource,
    context: ParserContext<MSyntaxKind>,
    source: MTokenSource<'source>,
}

impl<'source> MParser<'source> {
    pub fn new(source: &'source str, source_type: MFileSource) -> Self {
        Self {
            state: MParserState::new(),
            source_type,
            context: ParserContext::default(),
            source: MTokenSource::new_from_str(source),
        }
    }

    pub(self) fn state(&self) -> &MParserState {
        &self.state
    }

    pub(self) fn state_mut(&mut self) -> &mut MParserState {
        &mut self.state
    }

    pub fn source_type(&self) -> MFileSource {
        self.source_type
    }

    /// Whether the code we are parsing is a module
    pub const fn is_module(&self) -> bool {
        self.source_type.module_kind().is_module()
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: MReLexContext) -> MSyntaxKind {
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
        F: FnOnce(&mut MParser) -> R,
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
        F: FnOnce(&mut MParser) -> R,
    {
        let snapshot = change.apply(self.state_mut());
        let result = func(self);
        C::restore(self.state_mut(), snapshot);
        result
    }

    pub fn checkpoint(&self) -> MParserCheckpoint {
        MParserCheckpoint {
            context: self.context.checkpoint(),
            source: self.source.checkpoint(),
            state: self.state.checkpoint(),
        }
    }

    pub fn rewind(&mut self, checkpoint: MParserCheckpoint) {
        let MParserCheckpoint {
            context,
            source,
            state,
        } = checkpoint;

        self.context.rewind(context);
        self.source.rewind(source);
        self.state.restore(state);
    }

    pub fn finish(self) -> (Vec<Event<MSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
        let (trivia, _) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        (events, parse_diagnostics, trivia)
    }
}

impl<'source> Parser for MParser<'source> {
    type Kind = MSyntaxKind;
    type Source = MTokenSource<'source>;

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

pub struct MParserCheckpoint {
    pub context: ParserContextCheckpoint,
    pub source: MTokenSourceCheckpoint,
    state: MParserStateCheckpoint,
}

#[derive(Debug)]
pub struct Parse<T> {
    root: MSyntaxNode,
    errors: Vec<ParseDiagnostic>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: MSyntaxNode, errors: Vec<ParseDiagnostic>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = MLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> MSyntaxNode {
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

impl<T: AstNode<Language = MLanguage>> Parse<T> {
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

pub fn parse(text: &str, source_type: MFileSource) -> Parse<AnyMRoot> {
    let mut cache = NodeCache::default();
    parse_m_with_cache(text, source_type, &mut cache)
}

fn parse_m_with_cache(
    text: &str,
    source_type: MFileSource,
    cache: &mut NodeCache,
) -> Parse<AnyMRoot> {
    tracing::debug_span!("parse").in_scope(move || {
        let (events, errors, tokens) = parse_common(text, source_type);
        let mut tree_sink = MLosslessTreeSink::with_cache(text, &tokens, cache);
        biome_parser::event::process(&mut tree_sink, events, errors);
        let (green, parse_errors) = tree_sink.finish();
        Parse::new(green, parse_errors)
    })
}

fn parse_common(
    text: &str,
    source_type: MFileSource,
) -> (Vec<Event<MSyntaxKind>>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut p = MParser::new(text, source_type);
    syntax_rules::program::parse(&mut p);

    let (events, errors, trivia) = p.finish();

    (events, errors, trivia)
}
