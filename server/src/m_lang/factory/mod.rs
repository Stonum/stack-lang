use crate::m_lang::syntax::MLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub use generated::MSyntaxFactory;

pub type MSyntaxTreeBuilder = TreeBuilder<'static, MLanguage, MSyntaxFactory>;
