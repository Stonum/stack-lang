//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMFunctionBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMFunctionBody;
impl_format!(AnyMFunctionBody, FormatAnyMFunctionBody);

impl FormatRule<AnyMFunctionBody> for FormatAnyMFunctionBody {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMFunctionBody, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMFunctionBody::AnyMExpression(node) => node.format().fmt(f),
            AnyMFunctionBody::MFunctionBody(node) => node.format().fmt(f),
        }
    }
}
