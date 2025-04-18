//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMFunction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMFunction;
impl_format!(AnyMFunction, FormatAnyMFunction);

impl FormatRule<AnyMFunction> for FormatAnyMFunction {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMFunction, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMFunction::MFunctionDeclaration(node) => node.format().fmt(f),
            AnyMFunction::MFunctionExpression(node) => node.format().fmt(f),
        }
    }
}
