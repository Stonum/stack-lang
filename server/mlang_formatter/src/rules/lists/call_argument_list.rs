use crate::prelude::*;
use crate::utils::write_arguments_multi_line;
use mlang_syntax::MCallArgumentList;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCallArgumentList;
impl_format!(MCallArgumentList, FormatMCallArgumentList);

impl FormatRule<MCallArgumentList> for FormatMCallArgumentList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MCallArgumentList, f: &mut MFormatter) -> FormatResult<()> {
        if node.len() == 0 {
            return Ok(());
        }

        write!(
            f,
            [&group(&soft_block_indent(&format_with(|f| {
                let separated = node
                    .format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit);
                write_arguments_multi_line(separated, f)
            })))]
        )
    }
}
