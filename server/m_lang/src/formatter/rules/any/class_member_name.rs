//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMClassMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMClassMemberName;
impl_format!(AnyMClassMemberName, FormatAnyMClassMemberName);

impl FormatRule<AnyMClassMemberName> for FormatAnyMClassMemberName {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMClassMemberName, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMClassMemberName::MComputedMemberName(node) => node.format().fmt(f),
            AnyMClassMemberName::MLiteralMemberName(node) => node.format().fmt(f),
        }
    }
}
