use biome_rowan::{declare_node_union, SyntaxResult};

use crate::syntax::{AnyMExpression, MComputedMemberAssignment, MStaticMemberAssignment};

declare_node_union! {
    pub AnyMMemberAssignment = MComputedMemberAssignment | MStaticMemberAssignment
}

impl AnyMMemberAssignment {
    pub fn object(&self) -> SyntaxResult<Option<AnyMExpression>> {
        match self {
            AnyMMemberAssignment::MComputedMemberAssignment(assignment) => Ok(assignment.object()),
            AnyMMemberAssignment::MStaticMemberAssignment(assignment) => {
                assignment.object().map(Some)
            }
        }
    }
}
