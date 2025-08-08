use crate::prelude::*;

use biome_formatter::token::number::format_number_token;
use biome_formatter::write;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    MDateLiteralExpression, MDateLiteralExpressionFields, MNullLiteralExpression,
    MNullLiteralExpressionFields, MNumberLiteralExpression, MNumberLiteralExpressionFields,
    MTimeLiteralExpression, MTimeLiteralExpressionFields,
};

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
