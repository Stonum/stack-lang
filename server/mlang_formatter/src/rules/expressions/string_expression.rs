use crate::prelude::*;
use crate::utils::{
    FormatLiteralStringToken, FormatSqlStringToken, StringLiteralParentKind,
};
use biome_formatter::FormatRuleWithOptions;
use biome_rowan::declare_node_union;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    MLongStringLiteralExpression, MLongStringLiteralExpressionFields, MStringLiteralExpression,
    MStringLiteralExpressionFields,
};

declare_node_union! {
    pub(crate) FormatString =
        MStringLiteralExpression |
        MLongStringLiteralExpression
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct FormatStringLiteralOptions {
    pub is_query_like_string: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStringLiteralExpression {
    options: FormatStringLiteralOptions,
}
impl_format_with_rule!(MStringLiteralExpression, FormatMStringLiteralExpression);

impl FormatRuleWithOptions<MStringLiteralExpression> for FormatMStringLiteralExpression {
    type Options = FormatStringLiteralOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<MStringLiteralExpression> for FormatMStringLiteralExpression {
    fn fmt_fields(&self, node: &MStringLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        if self.options.is_query_like_string {
            FormatSqlStringToken::new(&value_token).fmt(f)
        } else {
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression).fmt(f)
        }
    }

    fn needs_parentheses(&self, item: &MStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLongStringLiteralExpression {
    options: FormatStringLiteralOptions,
}
impl_format_with_rule!(
    MLongStringLiteralExpression,
    FormatMLongStringLiteralExpression
);

impl FormatRuleWithOptions<MLongStringLiteralExpression> for FormatMLongStringLiteralExpression {
    type Options = FormatStringLiteralOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<MLongStringLiteralExpression> for FormatMLongStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &MLongStringLiteralExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MLongStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        if self.options.is_query_like_string {
            FormatSqlStringToken::new(&value_token).fmt(f)
        } else {
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression).fmt(f)
        }
    }

    fn needs_parentheses(&self, item: &MLongStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
