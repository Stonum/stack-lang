use crate::prelude::*;
use crate::utils::member_chain::MemberChain;

use biome_formatter::write;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{AnyMExpression, MCallExpression, MCallExpressionFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCallExpression;
impl_format_with_rule!(MCallExpression, FormatMCallExpression);

impl FormatNodeRule<MCallExpression> for FormatMCallExpression {
    fn fmt_fields(&self, node: &MCallExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MCallExpressionFields { callee, arguments } = node.as_fields();

        let callee = callee?;

        if matches!(
            callee,
            AnyMExpression::MStaticMemberExpression(_)
                | AnyMExpression::MComputedMemberExpression(_)
        ) && !callee.needs_parentheses()
        {
            let member_chain = MemberChain::from_call_expression(
                node.clone(),
                f.comments(),
                f.options().tab_width(),
            )?;

            member_chain.fmt(f)
        } else {
            let format_inner = format_with(|f| write!(f, [callee.format(), arguments.format()]));

            if matches!(callee, AnyMExpression::MCallExpression(_)) {
                write!(f, [group(&format_inner)])
            } else {
                write!(f, [format_inner])
            }
        }
    }

    fn needs_parentheses(&self, item: &MCallExpression) -> bool {
        item.needs_parentheses()
    }
}
