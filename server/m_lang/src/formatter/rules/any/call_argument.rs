//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMCallArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMCallArgument;
impl_format!(AnyMCallArgument, FormatAnyMCallArgument);

impl FormatRule<AnyMCallArgument> for FormatAnyMCallArgument {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMCallArgument, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMCallArgument::AnyMExpression(node) => node.format().fmt(f),
            AnyMCallArgument::MSpread(node) => node.format().fmt(f),
        }
    }
}
