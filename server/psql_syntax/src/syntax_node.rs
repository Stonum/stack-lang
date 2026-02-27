use crate::{PsqlRoot, PsqlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PsqlLanguage;

impl Language for PsqlLanguage {
    type Kind = PsqlSyntaxKind;
    type Root = PsqlRoot;
}

pub type PsqlSyntaxNode = biome_rowan::SyntaxNode<PsqlLanguage>;
pub type PsqlSyntaxToken = biome_rowan::SyntaxToken<PsqlLanguage>;
pub type PsqlSyntaxElement = biome_rowan::SyntaxElement<PsqlLanguage>;
pub type PsqlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<PsqlLanguage>;
pub type PsqlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<PsqlLanguage>;
pub type PsqlSyntaxList = biome_rowan::SyntaxList<PsqlLanguage>;
pub type PsqlSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<PsqlLanguage>;
