//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMDeclaration;
impl_format!(AnyMDeclaration, FormatAnyMDeclaration);

impl FormatRule<AnyMDeclaration> for FormatAnyMDeclaration {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMDeclaration, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMDeclaration::MClassDeclaration(node) => node.format().fmt(f),
            AnyMDeclaration::MFunctionDeclaration(node) => node.format().fmt(f),
            AnyMDeclaration::MVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
