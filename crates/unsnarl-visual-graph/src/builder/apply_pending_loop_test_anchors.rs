//! Flush pending loop-test anchor nodes into their host subgraphs at
//! the recorded position (first / last child).

use super::arena::{BuildArena, Container, ElementHandle};
use super::state::{BuildState, LoopTestAnchorPosition};

pub fn apply_pending_loop_test_anchors(arena: &mut BuildArena, state: &BuildState) {
    for pending in &state.pending_loop_test_anchors {
        let container = Container::Subgraph(pending.subgraph);
        let handle = ElementHandle::Node(pending.node);
        match pending.position {
            LoopTestAnchorPosition::First => arena.prepend_child(container, handle),
            LoopTestAnchorPosition::Last => arena.append_child(container, handle),
        }
    }
}
