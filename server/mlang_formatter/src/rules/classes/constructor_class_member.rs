use crate::prelude::*;

use super::method_class_member::FormatAnyMMethodMember;
use biome_formatter::write;
use mlang_syntax::MConstructorClassMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMConstructorClassMember;
impl_format_with_rule!(MConstructorClassMember, FormatMConstructorClassMember);

impl FormatNodeRule<MConstructorClassMember> for FormatMConstructorClassMember {
    fn fmt_fields(&self, node: &MConstructorClassMember, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [space(), FormatAnyMMethodMember::from(node.clone())]]
    }
}
