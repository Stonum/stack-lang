use crate::formatter::prelude::*;
use crate::syntax::{MInProperty, MInPropertyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMInProperty;
impl_format!(MInProperty, FormatMInProperty);
impl FormatRule<MInProperty> for FormatMInProperty {
    type Context = MFormatContext;
    fn fmt(&self, node: &MInProperty, f: &mut MFormatter) -> FormatResult<()> {
        let MInPropertyFields { any_m_expression } = node.as_fields();
        any_m_expression.format().fmt(f)
    }
}
