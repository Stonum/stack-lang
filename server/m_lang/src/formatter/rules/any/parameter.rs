//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMParameter;
impl_format!(AnyMParameter, FormatAnyMParameter);
impl FormatRule<AnyMParameter> for FormatAnyMParameter {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMParameter, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMParameter::AnyMFormalParameter(node) => node.format().fmt(f),
            AnyMParameter::MRestParameter(node) => node.format().fmt(f),
        }
    }
}
