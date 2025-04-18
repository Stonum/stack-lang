use crate::formatter::prelude::*;

use crate::syntax::MConstructorParameters;

use super::parameters::FormatAnyMParameters;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMConstructorParameters;
impl_format_with_rule!(MConstructorParameters, FormatMConstructorParameters);

impl FormatNodeRule<MConstructorParameters> for FormatMConstructorParameters {
    fn fmt_fields(&self, node: &MConstructorParameters, f: &mut MFormatter) -> FormatResult<()> {
        FormatAnyMParameters::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        _: &MConstructorParameters,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `FormatMAnyParameters`
        Ok(())
    }
}
