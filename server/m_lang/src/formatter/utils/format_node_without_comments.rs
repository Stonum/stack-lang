use crate::formatter::prelude::*;
use crate::formatter::rules::bogus::bogus_expression::FormatMBogusExpression;
use crate::formatter::rules::expressions::array_expression::FormatMArrayExpression;
use crate::formatter::rules::expressions::assignment_expression::FormatMAssignmentExpression;
use crate::formatter::rules::expressions::binary_expression::FormatMBinaryExpression;
use crate::formatter::rules::expressions::boolean_literal_expression::FormatMBooleanLiteralExpression;
use crate::formatter::rules::expressions::call_expression::FormatMCallExpression;
use crate::formatter::rules::expressions::computed_member_expression::FormatMComputedMemberExpression;
use crate::formatter::rules::expressions::conditional_expression::FormatMConditionalExpression;
use crate::formatter::rules::expressions::function_expression::FormatMFunctionExpression;
use crate::formatter::rules::expressions::identifier_expression::FormatMIdentifierExpression;
use crate::formatter::rules::expressions::in_expression::FormatMInExpression;
use crate::formatter::rules::expressions::literal_expression::FormatMNumberLiteralExpression;
use crate::formatter::rules::expressions::literal_expression::{
    FormatMDateLiteralExpression, FormatMLongStringLiteralExpression, FormatMNullLiteralExpression,
    FormatMStringLiteralExpression, FormatMTimeLiteralExpression,
};
use crate::formatter::rules::expressions::logical_expression::FormatMLogicalExpression;
use crate::formatter::rules::expressions::new_expression::FormatMNewExpression;
use crate::formatter::rules::expressions::object_expression::FormatMObjectExpression;
use crate::formatter::rules::expressions::parenthesized_expression::FormatMParenthesizedExpression;
use crate::formatter::rules::expressions::post_update_expression::FormatMPostUpdateExpression;
use crate::formatter::rules::expressions::pre_update_expression::FormatMPreUpdateExpression;
use crate::formatter::rules::expressions::sequence_expression::FormatMSequenceExpression;
use crate::formatter::rules::expressions::static_member_expression::FormatMStaticMemberExpression;
use crate::formatter::rules::expressions::super_expression::FormatMSuperExpression;
use crate::formatter::rules::expressions::this_expression::FormatMThisExpression;
use crate::formatter::rules::expressions::unary_expression::FormatMUnaryExpression;
use crate::syntax::{AnyMExpression, AnyMLiteralExpression};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMExpressionWithoutComments;

impl FormatRule<AnyMExpression> for FormatAnyMExpressionWithoutComments {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMExpression, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMExpression::AnyMLiteralExpression(literal_expr) => match literal_expr {
                AnyMLiteralExpression::MBooleanLiteralExpression(node) => {
                    FormatMBooleanLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MNullLiteralExpression(node) => {
                    FormatMNullLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MNumberLiteralExpression(node) => {
                    FormatMNumberLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MStringLiteralExpression(node) => {
                    FormatMStringLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MLongStringLiteralExpression(node) => {
                    FormatMLongStringLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MDateLiteralExpression(node) => {
                    FormatMDateLiteralExpression.fmt_node(node, f)
                }
                AnyMLiteralExpression::MTimeLiteralExpression(node) => {
                    FormatMTimeLiteralExpression.fmt_node(node, f)
                }
            },
            AnyMExpression::MArrayExpression(node) => {
                FormatMArrayExpression::default().fmt_node(node, f)
            }
            AnyMExpression::MAssignmentExpression(node) => {
                FormatMAssignmentExpression.fmt_node(node, f)
            }
            AnyMExpression::MBinaryExpression(node) => FormatMBinaryExpression.fmt_node(node, f),
            AnyMExpression::MBogusExpression(node) => FormatMBogusExpression.fmt(node, f),
            AnyMExpression::MCallExpression(node) => FormatMCallExpression.fmt_node(node, f),
            AnyMExpression::MComputedMemberExpression(node) => {
                FormatMComputedMemberExpression.fmt_node(node, f)
            }
            AnyMExpression::MConditionalExpression(node) => {
                FormatMConditionalExpression::default().fmt_node(node, f)
            }
            AnyMExpression::MFunctionExpression(node) => {
                FormatMFunctionExpression::default().fmt_node(node, f)
            }
            AnyMExpression::MIdentifierExpression(node) => {
                FormatMIdentifierExpression.fmt_node(node, f)
            }
            AnyMExpression::MInExpression(node) => FormatMInExpression.fmt_node(node, f),
            AnyMExpression::MLogicalExpression(node) => FormatMLogicalExpression.fmt_node(node, f),
            AnyMExpression::MNewExpression(node) => FormatMNewExpression.fmt_node(node, f),
            AnyMExpression::MObjectExpression(node) => FormatMObjectExpression.fmt_node(node, f),
            AnyMExpression::MParenthesizedExpression(node) => {
                FormatMParenthesizedExpression.fmt_node(node, f)
            }
            AnyMExpression::MPostUpdateExpression(node) => {
                FormatMPostUpdateExpression.fmt_node(node, f)
            }
            AnyMExpression::MPreUpdateExpression(node) => {
                FormatMPreUpdateExpression.fmt_node(node, f)
            }
            AnyMExpression::MSequenceExpression(node) => {
                FormatMSequenceExpression.fmt_node(node, f)
            }
            AnyMExpression::MStaticMemberExpression(node) => {
                FormatMStaticMemberExpression.fmt_node(node, f)
            }
            AnyMExpression::MSuperExpression(node) => FormatMSuperExpression.fmt_node(node, f),
            AnyMExpression::MThisExpression(node) => FormatMThisExpression.fmt_node(node, f),
            AnyMExpression::MUnaryExpression(node) => FormatMUnaryExpression.fmt_node(node, f),
            _ => unimplemented!("Need format other expressions"),
        }
    }
}
