//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMForInInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMForInInitializer;
impl_format!(AnyMForInInitializer, FormatAnyMForInInitializer);

impl FormatRule<AnyMForInInitializer> for FormatAnyMForInInitializer {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMForInInitializer, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMForInInitializer::AnyMAssignment(node) => node.format().fmt(f),
            AnyMForInInitializer::MForVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
