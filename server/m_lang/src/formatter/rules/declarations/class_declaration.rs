use crate::formatter::prelude::*;
use crate::formatter::utils::format_class::FormatClass;

use crate::syntax::MClassDeclaration;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMClassDeclaration;
impl_format_with_rule!(MClassDeclaration, FormatMClassDeclaration);

impl FormatNodeRule<MClassDeclaration> for FormatMClassDeclaration {
    fn fmt_fields(&self, node: &MClassDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        FormatClass::from(&node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &MClassDeclaration, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted as part of `FormatClass`
        Ok(())
    }
}
