//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMRoot;
impl_format!(AnyMRoot, FormatAnyMRoot);
impl FormatRule<AnyMRoot> for FormatAnyMRoot {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMRoot, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMRoot::MExpressionSnipped(node) => node.format().fmt(f),
            AnyMRoot::MModule(node) => node.format().fmt(f),
            AnyMRoot::MScript(node) => node.format().fmt(f),
            AnyMRoot::MReportFile(node) => node.format().fmt(f),
        }
    }
}
