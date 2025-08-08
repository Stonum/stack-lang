use crate::{AnyMRoot, MSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MLanguage;

impl Language for MLanguage {
    type Kind = MSyntaxKind;
    type Root = AnyMRoot;
}

pub type MSyntaxNode = biome_rowan::SyntaxNode<MLanguage>;
pub type MSyntaxToken = biome_rowan::SyntaxToken<MLanguage>;
pub type MSyntaxElement = biome_rowan::SyntaxElement<MLanguage>;
pub type MSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<MLanguage>;
pub type MSyntaxElementChildren = biome_rowan::SyntaxElementChildren<MLanguage>;
pub type MSyntaxList = biome_rowan::SyntaxList<MLanguage>;
pub type MSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<MLanguage>;
