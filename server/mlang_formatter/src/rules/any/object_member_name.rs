//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMObjectMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMObjectMemberName;
impl_format!(AnyMObjectMemberName, FormatAnyMObjectMemberName);
impl FormatRule<AnyMObjectMemberName> for FormatAnyMObjectMemberName {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMObjectMemberName, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMObjectMemberName::MComputedMemberName(node) => node.format().fmt(f),
            AnyMObjectMemberName::MLiteralMemberName(node) => node.format().fmt(f),
        }
    }
}
