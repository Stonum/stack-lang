use crate::PsqlParser;
use enumflags2::{BitFlags, bitflags, make_bitflags};

use std::ops::{BitOr, BitOrAssign, Deref, DerefMut, Sub};

/// State kept by the parser while parsing.
#[derive(Debug)]
pub struct PsqlParserState {
    parsing_context: ParsingContextFlags,
}

impl PsqlParserState {
    pub fn new() -> Self {
        PsqlParserState {
            parsing_context: ParsingContextFlags::TOP_LEVEL,
        }
    }

    pub fn checkpoint(&self) -> PsqlParserStateCheckpoint {
        PsqlParserStateCheckpoint::snapshot(self)
    }

    pub fn restore(&mut self, checkpoint: PsqlParserStateCheckpoint) {
        checkpoint.rewind(self);
    }
}

/// Stores a checkpoint of the [PsqlParserState].
/// Allows rewinding the state to its previous state.
///
/// It's important that creating and rewinding a snapshot is cheap. Consider the performance implications
/// before adding new unscoped state.
#[derive(Debug)]
pub struct PsqlParserStateCheckpoint {
    /// Additional data that we only want to store in debug mode
    #[cfg(debug_assertions)]
    debug_checkpoint: PsqlDebugParserStateCheckpoint,
}

impl PsqlParserStateCheckpoint {
    /// Creates a snapshot of the passed in state.
    #[cfg(debug_assertions)]
    fn snapshot(state: &PsqlParserState) -> Self {
        Self {
            debug_checkpoint: PsqlDebugParserStateCheckpoint::snapshot(state),
        }
    }

    #[cfg(not(debug_assertions))]
    fn snapshot(_: &PsqlParserState) -> Self {
        Self {}
    }

    /// Restores the `state values` to the time when this snapshot was created.
    #[cfg(debug_assertions)]
    fn rewind(self, state: &mut PsqlParserState) {
        self.debug_checkpoint.rewind(state);
    }

    #[cfg(not(debug_assertions))]
    fn rewind(self, _: &PsqlParserState) {}
}

/// Psqlost of the [PsqlParserState] is scoped state. It should, therefore, not be necessary to rewind
/// that state because that's already taken care of by `with_state` and `with_scoped_state`.
/// But, you can never no and better be safe than sorry. That's why we use some heuristics
/// to verify that non of the scoped state did change and assert for it when rewinding.
#[derive(Debug, Clone)]
#[cfg(debug_assertions)]
pub struct PsqlDebugParserStateCheckpoint {
    parsing_context: ParsingContextFlags,
}

#[cfg(debug_assertions)]
impl PsqlDebugParserStateCheckpoint {
    fn snapshot(state: &PsqlParserState) -> Self {
        Self {
            parsing_context: state.parsing_context,
        }
    }

    fn rewind(self, state: &mut PsqlParserState) {
        assert_eq!(state.parsing_context, self.parsing_context);
    }
}

/// Reverts state changes to their previous value when it goes out of scope.
/// Can be used like a regular parser.
pub struct ParserStateGuard<'parser, 't, C>
where
    C: ChangeParserState,
{
    snapshot: C::Snapshot,
    inner: &'parser mut PsqlParser<'t>,
}

impl<C: ChangeParserState> Drop for ParserStateGuard<'_, '_, C> {
    fn drop(&mut self) {
        let snapshot = std::mem::take(&mut self.snapshot);

        C::restore(self.inner.state_mut(), snapshot);
    }
}

impl<'t, C: ChangeParserState> Deref for ParserStateGuard<'_, 't, C> {
    type Target = PsqlParser<'t>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<C: ChangeParserState> DerefMut for ParserStateGuard<'_, '_, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

/// Implements a specific modification to the parser state that can later be reverted.
pub trait ChangeParserState {
    type Snapshot: Default;

    /// Applies the change to the passed in state and returns snapshot that allows restoring the previous state.
    fn apply(self, state: &mut PsqlParserState) -> Self::Snapshot;

    /// Restores the state to its previous value
    fn restore(state: &mut PsqlParserState, value: Self::Snapshot);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ParsingContextFlag {
    TopLevel = 1 << 2,
}

#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub struct ParsingContextFlags(BitFlags<ParsingContextFlag>);

impl ParsingContextFlags {
    /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
    const TOP_LEVEL: Self = Self(make_bitflags!(ParsingContextFlag::{TopLevel}));

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<ParsingContextFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl BitOr for ParsingContextFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ParsingContextFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for ParsingContextFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Sub for ParsingContextFlags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}
