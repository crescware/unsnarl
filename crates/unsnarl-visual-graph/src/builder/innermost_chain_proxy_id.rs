//! The id of the innermost `CallProxy` actually created for a method
//! chain.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedHeadExpression;

use super::arena::{BuildArena, SubgraphIdx};
use super::call_node_extent::call_node_extent;
use super::receiver_call_chain::receiver_call_chain;

/// The id of the innermost `CallProxy` actually created for a method
/// chain -- the deepest call in `head`'s receiver chain that a callback
/// landed in. The chain's input (the innermost receiver, e.g. `arr`) is
/// re-pointed here so the backbone reads `arr -> arr.map() -> ...` rather
/// than skipping straight to the outermost call. Returns the host proxy
/// id for a single (non-chained) call, which makes the caller's re-point
/// a no-op there.
pub fn innermost_chain_proxy_id(
    arena: &BuildArena,
    head: &SerializedHeadExpression,
    nested_call_proxy: &HashMap<(u32, u32), SubgraphIdx>,
) -> Option<String> {
    receiver_call_chain(head).iter().rev().find_map(|node| {
        let (start_offset, end_offset, _, _) = call_node_extent(node)?;
        let idx = nested_call_proxy.get(&(start_offset, end_offset))?;
        Some(arena.subgraph(*idx).descriptor.id().to_string())
    })
}
