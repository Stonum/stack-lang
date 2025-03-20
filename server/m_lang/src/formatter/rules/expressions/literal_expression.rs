use crate::formatter::prelude::*;
use crate::formatter::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use biome_formatter::token::number::format_number_token;
use biome_formatter::write;

use crate::syntax::parentheses::NeedsParentheses;
use crate::syntax::{
    MDateLiteralExpression, MDateLiteralExpressionFields, MLongStringLiteralExpression,
    MLongStringLiteralExpressionFields, MNullLiteralExpression, MNullLiteralExpressionFields,
    MNumberLiteralExpression, MNumberLiteralExpressionFields, MStringLiteralExpression,
    MStringLiteralExpressionFields, MTimeLiteralExpression, MTimeLiteralExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStringLiteralExpression;
impl_format_with_rule!(MStringLiteralExpression, FormatMStringLiteralExpression);

impl FormatNodeRule<MStringLiteralExpression> for FormatMStringLiteralExpression {
    fn fmt_fields(&self, node: &MStringLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let formatted =
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression);

        formatted.fmt(f)
    }

    fn needs_parentheses(&self, item: &MStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLongStringLiteralExpression;
impl_format_with_rule!(
    MLongStringLiteralExpression,
    FormatMLongStringLiteralExpression
);

impl FormatNodeRule<MLongStringLiteralExpression> for FormatMLongStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &MLongStringLiteralExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MLongStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let formatted =
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression);

        formatted.fmt(f)
    }

    fn needs_parentheses(&self, item: &MLongStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMTimeLiteralExpression;
impl_format_with_rule!(MTimeLiteralExpression, FormatMTimeLiteralExpression);

impl FormatNodeRule<MTimeLiteralExpression> for FormatMTimeLiteralExpression {
    fn fmt_fields(&self, node: &MTimeLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MTimeLiteralExpressionFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &MTimeLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMDateLiteralExpression;
impl_format_with_rule!(MDateLiteralExpression, FormatMDateLiteralExpression);

impl FormatNodeRule<MDateLiteralExpression> for FormatMDateLiteralExpression {
    fn fmt_fields(&self, node: &MDateLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MDateLiteralExpressionFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &MDateLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMNullLiteralExpression;
impl_format_with_rule!(MNullLiteralExpression, FormatMNullLiteralExpression);

impl FormatNodeRule<MNullLiteralExpression> for FormatMNullLiteralExpression {
    fn fmt_fields(&self, node: &MNullLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MNullLiteralExpressionFields { value_token } = node.as_fields();

        write![f, [value_token.format()]]
    }

    fn needs_parentheses(&self, item: &MNullLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMNumberLiteralExpression;
impl_format_with_rule!(MNumberLiteralExpression, FormatMNumberLiteralExpression);

impl FormatNodeRule<MNumberLiteralExpression> for FormatMNumberLiteralExpression {
    fn fmt_fields(&self, node: &MNumberLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MNumberLiteralExpressionFields { value_token } = node.as_fields();
        format_number_token(&value_token?).fmt(f)
    }

    fn needs_parentheses(&self, item: &MNumberLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
