use crate::formatter::prelude::*;
use crate::syntax::{MForIteratorFactory, MForIteratorFactoryFields};
use biome_formatter::{format_args, write};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMForIteratorFactory;
impl_format_with_rule!(MForIteratorFactory, FormatMForIteratorFactory);
impl FormatNodeRule<MForIteratorFactory> for FormatMForIteratorFactory {
    fn fmt_fields(&self, node: &MForIteratorFactory, f: &mut MFormatter) -> FormatResult<()> {
        let MForIteratorFactoryFields {
            name,
            l_paren_token,
            expression,
            comma_token,
            initializer,
            r_paren_token,
        } = node.as_fields();

        let name = name.format();
        let expression = expression.format();
        let initializer = initializer.format();

        write!(
            f,
            [group(&format_args!(
                name,
                l_paren_token.format(),
                expression,
                comma_token.format(),
                space(),
                initializer,
                r_paren_token.format()
            ))]
        )
    }
}
