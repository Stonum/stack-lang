//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use biome_formatter::FormatRuleWithOptions;

use crate::formatter::prelude::*;
use crate::formatter::rules::expressions::string_expression::FormatStringLiteralOptions;
use crate::syntax::AnyMLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMLiteralExpression {
    options: FormatStringLiteralOptions,
}
impl_format!(AnyMLiteralExpression, FormatAnyMLiteralExpression);

impl FormatRuleWithOptions<AnyMLiteralExpression> for FormatAnyMLiteralExpression {
    type Options = FormatStringLiteralOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatRule<AnyMLiteralExpression> for FormatAnyMLiteralExpression {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMLiteralExpression::MBooleanLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MNullLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MNumberLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MDateLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MTimeLiteralExpression(node) => node.format().fmt(f),
            AnyMLiteralExpression::MStringLiteralExpression(node) => {
                node.format().with_options(self.options).fmt(f)
            }
            AnyMLiteralExpression::MLongStringLiteralExpression(node) => {
                node.format().with_options(self.options).fmt(f)
            }
        }
    }
}
