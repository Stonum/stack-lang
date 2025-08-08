use super::prelude::*;
use super::rules::tokens::FormatMSyntaxToken;
use super::AsFormat;
use mlang_syntax::{MLanguage, MSyntaxToken};
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_formatter::FormatRefWithRule;
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct MFormatSeparatedElementRule<N>
where
    N: AstNode<Language = MLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for MFormatSeparatedElementRule<N>
where
    N: AstNode<Language = MLanguage> + AsFormat<MFormatContext> + 'static,
{
    type Context = MFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, MSyntaxToken, FormatMSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a MSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type MFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<MLanguage, Node>,
    Node,
    MFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = MLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(&self, separator: &'static str) -> MFormatSeparatedIter<Self::Node> {
        MFormatSeparatedIter::new(
            self.elements(),
            separator,
            MFormatSeparatedElementRule { node: PhantomData },
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = MLanguage> {}
