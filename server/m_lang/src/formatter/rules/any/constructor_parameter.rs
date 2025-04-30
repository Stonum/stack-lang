//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMConstructorParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMConstructorParameter;
impl_format!(AnyMConstructorParameter, FormatAnyMConstructorParameter);

impl FormatRule<AnyMConstructorParameter> for FormatAnyMConstructorParameter {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMConstructorParameter, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMConstructorParameter::AnyMFormalParameter(node) => node.format().fmt(f),
            AnyMConstructorParameter::MRestParameter(node) => node.format().fmt(f),
        }
    }
}
