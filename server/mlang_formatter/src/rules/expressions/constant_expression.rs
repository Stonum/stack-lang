use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MConstantExpression;
use mlang_syntax::MConstantExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMConstantExpression;
impl_format_with_rule!(MConstantExpression, FormatMConstantExpression);

impl FormatNodeRule<MConstantExpression> for FormatMConstantExpression {
    fn fmt_fields(&self, node: &MConstantExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MConstantExpressionFields { token, constant } = node.as_fields();

        write![f, [token.format(), constant.format()]]
    }
}
