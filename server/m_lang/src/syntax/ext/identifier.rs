use crate::syntax::{MIdentifierAssignment, MReferenceIdentifier, MSyntaxToken};
use biome_rowan::{declare_node_union, SyntaxResult};

declare_node_union! {
    pub AnyMIdentifierUsage = MReferenceIdentifier | MIdentifierAssignment
}

impl AnyMIdentifierUsage {
    pub fn value_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMIdentifierUsage::MReferenceIdentifier(node) => node.value_token(),
            AnyMIdentifierUsage::MIdentifierAssignment(node) => node.name_token(),
        }
    }
}
