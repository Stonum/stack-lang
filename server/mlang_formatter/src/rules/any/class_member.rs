//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMClassMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMClassMember;
impl_format!(AnyMClassMember, FormatAnyMClassMember);

impl FormatRule<AnyMClassMember> for FormatAnyMClassMember {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMClassMember, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMClassMember::MBogusMember(node) => node.format().fmt(f),
            AnyMClassMember::MConstructorClassMember(node) => node.format().fmt(f),
            AnyMClassMember::MGetterClassMember(node) => node.format().fmt(f),
            AnyMClassMember::MMethodClassMember(node) => node.format().fmt(f),
            AnyMClassMember::MSetterClassMember(node) => node.format().fmt(f),
        }
    }
}
