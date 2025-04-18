use crate::formatter::prelude::*;
use crate::formatter::rules::declarations::function_declaration::{
    FormatFunction, FormatFunctionOptions,
};

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::MFunctionExpression;
use biome_formatter::FormatRuleWithOptions;

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatMFunctionExpression {
    options: FormatFunctionOptions,
}
impl_format_with_rule!(MFunctionExpression, FormatMFunctionExpression);

impl FormatRuleWithOptions<MFunctionExpression> for FormatMFunctionExpression {
    type Options = FormatFunctionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<MFunctionExpression> for FormatMFunctionExpression {
    fn fmt_fields(&self, node: &MFunctionExpression, f: &mut MFormatter) -> FormatResult<()> {
        FormatFunction::from(node.clone()).fmt_with_options(f, &self.options)?;
        Ok(())
    }

    fn needs_parentheses(&self, item: &MFunctionExpression) -> bool {
        item.needs_parentheses()
    }
}
