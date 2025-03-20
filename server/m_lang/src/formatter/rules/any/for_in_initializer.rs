//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMForInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMForInitializer;
impl_format!(AnyMForInitializer, FormatAnyMForInitializer);

impl FormatRule<AnyMForInitializer> for FormatAnyMForInitializer {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMForInitializer, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMForInitializer::AnyMExpression(node) => node.format().fmt(f),
            AnyMForInitializer::MVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
