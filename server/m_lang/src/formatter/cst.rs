use super::prelude::*;
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule};

use super::{AsFormat, IntoFormat, MFormatContext};
use crate::syntax::{map_syntax_node, MSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatMSyntaxNode;

impl biome_formatter::FormatRule<MSyntaxNode> for FormatMSyntaxNode {
    type Context = MFormatContext;

    fn fmt(&self, node: &MSyntaxNode, f: &mut MFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<MFormatContext> for MSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, MSyntaxNode, FormatMSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMSyntaxNode)
    }
}

impl IntoFormat<MFormatContext> for MSyntaxNode {
    type Format = FormatOwnedWithRule<MSyntaxNode, FormatMSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMSyntaxNode)
    }
}
