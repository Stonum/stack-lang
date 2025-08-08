use biome_rowan::TreeBuilder;
use mlang_syntax::MLanguage;

mod generated;
pub mod make;
pub use generated::MSyntaxFactory;

pub type MSyntaxTreeBuilder = TreeBuilder<'static, MLanguage, MSyntaxFactory>;
