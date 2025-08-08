//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMDeclarationClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMDeclarationClause;
impl_format!(AnyMDeclarationClause, FormatAnyMDeclarationClause);

impl FormatRule<AnyMDeclarationClause> for FormatAnyMDeclarationClause {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMDeclarationClause, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMDeclarationClause::MClassDeclaration(node) => node.format().fmt(f),
            AnyMDeclarationClause::MFunctionDeclaration(node) => node.format().fmt(f),
            AnyMDeclarationClause::MVariableDeclarationClause(node) => node.format().fmt(f),
        }
    }
}
