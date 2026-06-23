use biome_rowan::{SyntaxKind, SyntaxNode, SyntaxSlot};

macro_rules! assert_parser {
    ($res:expr) => {
        assert!($res.try_tree().is_some());
        assert!(!$res.has_errors());
        assert!(!helper::has_bogus_nodes_or_empty_slots(&$res.syntax()));
    };

    ($res:expr, debug) => {
        dbg!(&$res.syntax());

        assert!($res.try_tree().is_some());
        dbg!($res.try_tree(), $res.diagnostics());

        assert!(!$res.has_errors());
        assert!(!helper::has_bogus_nodes_or_empty_slots(&$res.syntax()));
    };
}

pub(crate) fn has_bogus_nodes_or_empty_slots<L: biome_rowan::Language>(
    node: &SyntaxNode<L>,
) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty { .. }));
        }

        false
    })
}
