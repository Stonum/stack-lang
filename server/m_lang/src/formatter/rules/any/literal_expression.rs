//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMLiteralExpression;
impl_format!(AnyMLiteralExpression, FormatAnyMLiteralExpression);
impl FormatRule<AnyMLiteralExpression> for FormatAnyMLiteralExpression {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMLiteralExpression::MBooleanLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MNullLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MNumberLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MStringLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MDateLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MTimeLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MLongStringLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
