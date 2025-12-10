mod declaration;
mod info;
mod reference;

use biome_rowan::{SyntaxNode, WalkEvent};
use mlang_lsp_definition::SemanticInfo;
use rustc_hash::FxHashMap;

use line_index::LineIndex;
use mlang_syntax::{MFileSource, MLanguage};

pub use declaration::*;
pub use info::*;
pub use reference::*;

pub fn semantics(
    text: &str,
    root: SyntaxNode<MLanguage>,
    source_type: MFileSource,
) -> SemanticModel {
    let mut collector = SemanticModel {
        definitions: vec![],
        references: FxHashMap::default(),
    };

    let line_index = LineIndex::new(text);

    for event in root.preorder() {
        if let WalkEvent::Enter(node) = event {
            prepare_definitions(&mut collector, source_type, &line_index, &node);

            if let Some((rref, range)) = get_reference(&line_index, &node) {
                collector.references.entry(rref).or_default().push(range);
            }
        }
    }

    collector
}

#[derive(Debug, Default)]
pub struct SemanticModel {
    definitions: Vec<AnyMDefinition>,
    references: FxHashMap<SemanticInfo, Vec<MReferenceLocation>>,
}

impl SemanticModel {
    pub fn definitions<'a>(&'a self) -> core::slice::Iter<'a, AnyMDefinition> {
        self.definitions.iter()
    }
    pub fn references(&self) -> &FxHashMap<SemanticInfo, Vec<MReferenceLocation>> {
        &self.references
    }
}
