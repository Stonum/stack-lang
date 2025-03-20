use crate::formatter::prelude::*;

use crate::syntax::MArrayHole;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayHole;
impl_format_with_rule!(MArrayHole, FormatMArrayHole);

impl FormatNodeRule<MArrayHole> for FormatMArrayHole {
    fn fmt_fields(&self, _: &MArrayHole, _: &mut MFormatter) -> FormatResult<()> {
        Ok(())
    }
}
