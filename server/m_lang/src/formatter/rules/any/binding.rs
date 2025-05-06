//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::{AnyMBinding, AnyMFunctionBinding};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMBinding;
impl_format!(AnyMBinding, FormatAnyMBinding);

impl FormatRule<AnyMBinding> for FormatAnyMBinding {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMBinding, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMBinding::MBogusBinding(node) => node.format().fmt(f),
            AnyMBinding::MIdentifierBinding(node) => node.format().fmt(f),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMFunctionBinding;
impl_format!(AnyMFunctionBinding, FormatAnyMFunctionBinding);

impl FormatRule<AnyMFunctionBinding> for FormatAnyMFunctionBinding {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMFunctionBinding, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMFunctionBinding::MBogusBinding(node) => node.format().fmt(f),
            AnyMFunctionBinding::MIdentifierBinding(node) => node.format().fmt(f),
            AnyMFunctionBinding::MExtendedBinding(node) => node.format().fmt(f),
        }
    }
}
