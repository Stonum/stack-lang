//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMAssignment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMAssignment;
impl_format!(AnyMAssignment, FormatAnyMAssignment);

impl FormatRule<AnyMAssignment> for FormatAnyMAssignment {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMAssignment, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMAssignment::MBogusAssignment(node) => node.format().fmt(f),
            AnyMAssignment::MComputedMemberAssignment(node) => node.format().fmt(f),
            AnyMAssignment::MIdentifierAssignment(node) => node.format().fmt(f),
            AnyMAssignment::MParenthesizedAssignment(node) => node.format().fmt(f),
            AnyMAssignment::MStaticMemberAssignment(node) => node.format().fmt(f),
        }
    }
}
