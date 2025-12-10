use crate::{
    AnyMAssignment, MComputedMemberAssignment, MIdentifierAssignment, MStaticMemberAssignment,
    parentheses::NeedsParentheses,
};

impl NeedsParentheses for AnyMAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        match self {
            Self::MComputedMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::MIdentifierAssignment(assignment) => assignment.needs_parentheses(),
            Self::MStaticMemberAssignment(assignment) => assignment.needs_parentheses(),
            Self::MParenthesizedAssignment(_) | Self::MBogusAssignment(_) => false,
        }
    }
}

impl NeedsParentheses for MComputedMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MIdentifierAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}

impl NeedsParentheses for MStaticMemberAssignment {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }
}
