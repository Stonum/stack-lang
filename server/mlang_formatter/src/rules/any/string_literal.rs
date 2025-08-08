use crate::prelude::*;

use mlang_syntax::AnyMStringLiteralExpression;
impl_format!(
    AnyMStringLiteralExpression,
    FormatAnyMStringLiteralExpression
);

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMStringLiteralExpression;
impl FormatRule<AnyMStringLiteralExpression> for FormatAnyMStringLiteralExpression {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMStringLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMStringLiteralExpression::MLongStringLiteralExpression(node) => node.format().fmt(f),
            AnyMStringLiteralExpression::MStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
