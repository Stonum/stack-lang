//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMExpression;
impl_format!(AnyMExpression, FormatAnyMExpression);

impl FormatRule<AnyMExpression> for FormatAnyMExpression {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMExpression, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMExpression::AnyMLiteralExpression(node) => node.format().fmt(f),
            AnyMExpression::MArrayExpression(node) => node.format().fmt(f),
            AnyMExpression::MAssignmentExpression(node) => node.format().fmt(f),
            AnyMExpression::MBinaryExpression(node) => node.format().fmt(f),
            AnyMExpression::MBogusExpression(node) => node.format().fmt(f),
            AnyMExpression::MCallExpression(node) => node.format().fmt(f),
            AnyMExpression::MComputedMemberExpression(node) => node.format().fmt(f),
            AnyMExpression::MConditionalExpression(node) => node.format().fmt(f),
            AnyMExpression::MFunctionExpression(node) => node.format().fmt(f),
            AnyMExpression::MIdentifierExpression(node) => node.format().fmt(f),
            AnyMExpression::MInExpression(node) => node.format().fmt(f),
            AnyMExpression::MInstanceofExpression(node) => node.format().fmt(f),
            AnyMExpression::MLogicalExpression(node) => node.format().fmt(f),
            AnyMExpression::MNewExpression(node) => node.format().fmt(f),
            AnyMExpression::MObjectExpression(node) => node.format().fmt(f),
            AnyMExpression::MParenthesizedExpression(node) => node.format().fmt(f),
            AnyMExpression::MPostUpdateExpression(node) => node.format().fmt(f),
            AnyMExpression::MPreUpdateExpression(node) => node.format().fmt(f),
            AnyMExpression::MSequenceExpression(node) => node.format().fmt(f),
            AnyMExpression::MStaticMemberExpression(node) => node.format().fmt(f),
            AnyMExpression::MSuperExpression(node) => node.format().fmt(f),
            AnyMExpression::MThisExpression(node) => node.format().fmt(f),
            AnyMExpression::MUnaryExpression(node) => node.format().fmt(f),
            AnyMExpression::MConstantExpression(node) => node.format().fmt(f),
            AnyMExpression::MHashMapExpression(node) => node.format().fmt(f),
            AnyMExpression::MHashSetExpression(node) => node.format().fmt(f),
        }
    }
}
