//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMObjectMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMObjectMember;
impl_format!(AnyMObjectMember, FormatAnyMObjectMember);
impl FormatRule<AnyMObjectMember> for FormatAnyMObjectMember {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMObjectMember, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMObjectMember::MBogusMember(node) => node.format().fmt(f),
            AnyMObjectMember::MPropertyObjectMember(node) => node.format().fmt(f),
            AnyMObjectMember::MShorthandPropertyObjectMember(node) => node.format().fmt(f),
            AnyMObjectMember::MSpread(node) => node.format().fmt(f),
        }
    }
}
