use crate::MParser;
use enumflags2::{BitFlags, bitflags, make_bitflags};

use std::ops::{BitOr, BitOrAssign, Deref, DerefMut, Sub};

/// State kept by the parser while parsing.
#[derive(Debug)]
pub struct MParserState {
    parsing_context: ParsingContextFlags,

    /// If set, the parser reports bindings with identical names. The option stores the name of the
    /// node that disallows duplicate bindings, for example `let`, `const` or `import`.
    pub duplicate_binding_parent: Option<&'static str>,
}

impl MParserState {
    pub fn new() -> Self {
        MParserState {
            parsing_context: ParsingContextFlags::TOP_LEVEL,
            duplicate_binding_parent: None,
        }
    }

    pub fn in_function(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_FUNCTION)
    }

    pub fn in_constructor(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_CONSTRUCTOR)
    }

    pub fn continue_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::CONTINUE_ALLOWED)
    }
    pub fn break_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::BREAK_ALLOWED)
    }

    pub fn checkpoint(&self) -> MParserStateCheckpoint {
        MParserStateCheckpoint::snapshot(self)
    }

    pub fn restore(&mut self, checkpoint: MParserStateCheckpoint) {
        checkpoint.rewind(self);
    }
}

/// Stores a checkpoint of the [MParserState].
/// Allows rewinding the state to its previous state.
///
/// It's important that creating and rewinding a snapshot is cheap. Consider the performance implications
/// before adding new unscoped state.
#[derive(Debug)]
pub struct MParserStateCheckpoint {
    /// Additional data that we only want to store in debug mode
    #[cfg(debug_assertions)]
    debug_checkpoint: MDebugParserStateCheckpoint,
}

impl MParserStateCheckpoint {
    /// Creates a snapshot of the passed in state.
    #[cfg(debug_assertions)]
    fn snapshot(state: &MParserState) -> Self {
        Self {
            debug_checkpoint: MDebugParserStateCheckpoint::snapshot(state),
        }
    }

    #[cfg(not(debug_assertions))]
    fn snapshot(_: &MParserState) -> Self {
        Self {}
    }

    /// Restores the `state values` to the time when this snapshot was created.
    #[cfg(debug_assertions)]
    fn rewind(self, state: &mut MParserState) {
        self.debug_checkpoint.rewind(state);
    }

    #[cfg(not(debug_assertions))]
    fn rewind(self, _: &MParserState) {}
}

/// Most of the [MParserState] is scoped state. It should, therefore, not be necessary to rewind
/// that state because that's already taken care of by `with_state` and `with_scoped_state`.
/// But, you can never no and better be safe than sorry. That's why we use some heuristics
/// to verify that non of the scoped state did change and assert for it when rewinding.
#[derive(Debug, Clone)]
#[cfg(debug_assertions)]
pub struct MDebugParserStateCheckpoint {
    parsing_context: ParsingContextFlags,
    duplicate_binding_parent: Option<&'static str>,
    // name_map_len: usize,
}

#[cfg(debug_assertions)]
impl MDebugParserStateCheckpoint {
    fn snapshot(state: &MParserState) -> Self {
        Self {
            parsing_context: state.parsing_context,
            duplicate_binding_parent: state.duplicate_binding_parent,
        }
    }

    fn rewind(self, state: &mut MParserState) {
        assert_eq!(state.parsing_context, self.parsing_context);
        assert_eq!(
            state.duplicate_binding_parent,
            self.duplicate_binding_parent
        );
    }
}

/// Reverts state changes to their previous value when it goes out of scope.
/// Can be used like a regular parser.
pub struct ParserStateGuard<'parser, 't, C>
where
    C: ChangeParserState,
{
    snapshot: C::Snapshot,
    inner: &'parser mut MParser<'t>,
}

impl<C: ChangeParserState> Drop for ParserStateGuard<'_, '_, C> {
    fn drop(&mut self) {
        let snapshot = std::mem::take(&mut self.snapshot);

        C::restore(self.inner.state_mut(), snapshot);
    }
}

impl<'t, C: ChangeParserState> Deref for ParserStateGuard<'_, 't, C> {
    type Target = MParser<'t>;

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
    fn apply(self, state: &mut MParserState) -> Self::Snapshot;

    /// Restores the state to its previous value
    fn restore(state: &mut MParserState, value: Self::Snapshot);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum SignatureFlag {
    Constructor = 1 << 0,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SignatureFlags(BitFlags<SignatureFlag>);

impl SignatureFlags {
    /// Is the function a constructor (or constructor context)
    pub const CONSTRUCTOR: Self = Self(make_bitflags!(SignatureFlag::{Constructor}));

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<SignatureFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl From<SignatureFlags> for ParsingContextFlags {
    fn from(flags: SignatureFlags) -> Self {
        let mut parsing_context = ParsingContextFlags::empty();

        if flags.contains(SignatureFlags::CONSTRUCTOR) {
            parsing_context |= ParsingContextFlags::IN_CONSTRUCTOR;
        }

        parsing_context
    }
}

impl BitOr for SignatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SignatureFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for SignatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ParsingContextFlag {
    InFunction = 1 << 0,
    InConstructor = 1 << 1,
    TopLevel = 1 << 2,
    BreakAllowed = 1 << 3,
    ContinueAllowed = 1 << 4,
}

/// Flags representing the parsing state.
/// The reasons to use flags instead of individual boolean fields on `ParserState` are:
/// * It's possible to use bit masks to define what state should be inherited. For example,
///   functions inherit whether they're defined inside a parameter but override the `in_async` flag
/// * It's easier to snapshot the previous state. Individual boolean fields would require that a change
///   snapshots each individual boolean field to allow restoring the previous state. With bitflags, all that
///   is needed is to copy away the flags field and restore it after.
#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub struct ParsingContextFlags(BitFlags<ParsingContextFlag>);

impl ParsingContextFlags {
    /// Whether the parser is inside a function
    const IN_FUNCTION: Self = Self(make_bitflags!(ParsingContextFlag::{InFunction}));
    /// Whatever the parser is inside a constructor
    const IN_CONSTRUCTOR: Self = Self(make_bitflags!(ParsingContextFlag::{InConstructor}));

    /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
    const TOP_LEVEL: Self = Self(make_bitflags!(ParsingContextFlag::{TopLevel}));

    /// Whether the parser is in an iteration or switch statement and
    /// `break` is allowed.
    const BREAK_ALLOWED: Self = Self(make_bitflags!(ParsingContextFlag::{BreakAllowed}));

    /// Whether the parser is in an iteration statement and `continue` is allowed.
    const CONTINUE_ALLOWED: Self = Self(make_bitflags!(ParsingContextFlag::{ContinueAllowed}));

    /// Bitmask of all the flags that must be reset (shouldn't be inherited) when the parser enters a function
    const FUNCTION_RESET_MASK: Self = Self(
        make_bitflags!(ParsingContextFlag::{BreakAllowed | ContinueAllowed | InConstructor | TopLevel }),
    );

    /// Bitmask of all the flags that must be reset (shouldn't be inherited) when entering parameters.
    const PARAMETER_RESET_MASK: Self =
        Self(make_bitflags!(ParsingContextFlag::{InConstructor | InFunction | TopLevel }));

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

#[derive(Debug, Default, Copy, Clone)]
pub struct ParsingContextFlagsSnapshot(ParsingContextFlags);

pub trait ChangeParserStateFlags {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags;
}

impl<T: ChangeParserStateFlags> ChangeParserState for T {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut MParserState) -> Self::Snapshot {
        let new_flags = self.compute_new_flags(state.parsing_context);
        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, new_flags))
    }

    fn restore(state: &mut MParserState, value: Self::Snapshot) {
        state.parsing_context = value.0
    }
}

/// Enters the parsing of function/method parameters
pub struct EnterParameters(
    /// Whether async and yield are reserved keywords
    pub SignatureFlags,
);

impl ChangeParserStateFlags for EnterParameters {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        (existing - ParsingContextFlags::PARAMETER_RESET_MASK) | ParsingContextFlags::from(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BreakableKind {
    // Iteration statement like Do, While, For
    Iteration,

    // Switch statement
    Switch,
}

pub struct EnterBreakable(pub BreakableKind);

impl ChangeParserStateFlags for EnterBreakable {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        let mut flags = existing | ParsingContextFlags::BREAK_ALLOWED;

        if self.0 == BreakableKind::Iteration {
            flags |= ParsingContextFlags::CONTINUE_ALLOWED;
        }

        flags
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnterFunctionSnapshot {
    parsing_context: ParsingContextFlags,
}

/// Enters the parsing of a function/method. Resets the relevant parser state and sets the state
/// according to the passed [SignatureFlags]
pub struct EnterFunction(pub SignatureFlags);

impl ChangeParserState for EnterFunction {
    type Snapshot = EnterFunctionSnapshot;

    #[inline]
    fn apply(self, state: &mut MParserState) -> Self::Snapshot {
        let new_flags = (state.parsing_context - ParsingContextFlags::FUNCTION_RESET_MASK)
            | ParsingContextFlags::IN_FUNCTION
            | ParsingContextFlags::from(self.0);

        EnterFunctionSnapshot {
            parsing_context: std::mem::replace(&mut state.parsing_context, new_flags),
        }
    }

    #[inline]
    fn restore(state: &mut MParserState, value: Self::Snapshot) {
        state.parsing_context = value.parsing_context;
    }
}

pub struct EnterClassPropertyInitializer;

impl ChangeParserStateFlags for EnterClassPropertyInitializer {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        existing - ParsingContextFlags::TOP_LEVEL
    }
}

#[derive(Default, Debug, Clone)]
pub struct EnterClassStaticInitializationBlockSnapshot {
    flags: ParsingContextFlags,
}

pub struct EnterClassStaticInitializationBlock;

impl ChangeParserState for EnterClassStaticInitializationBlock {
    type Snapshot = EnterClassStaticInitializationBlockSnapshot;

    fn apply(self, state: &mut MParserState) -> Self::Snapshot {
        let flags = state.parsing_context - ParsingContextFlags::FUNCTION_RESET_MASK;
        EnterClassStaticInitializationBlockSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, flags),
        }
    }

    fn restore(state: &mut MParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
    }
}
