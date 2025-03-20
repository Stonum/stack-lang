//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMFormalParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMFormalParameter;
impl_format!(AnyMFormalParameter, FormatAnyMFormalParameter);

impl FormatRule<AnyMFormalParameter> for FormatAnyMFormalParameter {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMFormalParameter, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMFormalParameter::MBogusParameter(node) => node.format().fmt(f),
            AnyMFormalParameter::MFormalParameter(node) => node.format().fmt(f),
        }
    }
}
