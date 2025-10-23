use biome_rowan::TreeBuilder;
use psql_syntax::PsqlLanguage;

mod generated;
pub mod make;
pub use generated::PsqlSyntaxFactory;

pub type PsqlSyntaxTreeBuilder = TreeBuilder<'static, PsqlLanguage, PsqlSyntaxFactory>;
