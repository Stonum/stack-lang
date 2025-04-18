use crate::syntax::MLanguage;
use biome_rowan::TreeBuilder;

mod generated;
pub mod make;
pub use generated::MSyntaxFactory;

pub type MSyntaxTreeBuilder = TreeBuilder<'static, MLanguage, MSyntaxFactory>;
