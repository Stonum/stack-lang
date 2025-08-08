//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use mlang_syntax::AnyMArrayElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMArrayElement;
impl_format!(AnyMArrayElement, FormatAnyMArrayElement);

impl FormatRule<AnyMArrayElement> for FormatAnyMArrayElement {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMArrayElement, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMArrayElement::AnyMExpression(node) => node.format().fmt(f),
            AnyMArrayElement::MArrayHole(node) => node.format().fmt(f),
            AnyMArrayElement::MSpread(node) => node.format().fmt(f),
        }
    }
}
