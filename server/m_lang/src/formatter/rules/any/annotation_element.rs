//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_m_syntax::AnyMAnnotationElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMAnnotationElement;
impl FormatRule<AnyMAnnotationElement> for FormatAnyMAnnotationElement {
    type Context = MFormatterContext;
    fn fmt(&self, node: &AnyMAnnotationElement, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMAnnotationElement::MAnnotationBinding(node) => node.format().fmt(f),
            AnyMAnnotationElement::MAnnotationElement(node) => node.format().fmt(f),
        }
    }
}
