//! Owning storage for the builder's mutable nodes and subgraphs.
//!
//! Naively recording a subgraph and later pushing into its elements
//! from a different call site collides with the borrow checker and
//! the project's ban on `Rc<RefCell<T>>`. Every node and subgraph
//! instead lives once in the arena's flat `Vec`s, addressed by
//! [`NodeIdx`] / [`SubgraphIdx`] handles, and the nested tree
//! (subgraph elements) is recorded as parallel `Vec<ElementHandle>`
//! lists that the builder rebuilds into a real `Vec<VisualElement>`
//! at finalize time. Each subgraph's `descriptor.elements` therefore
//! stays empty until `finalize_root` walks the handle tree.
//!
//! `Container` is the write-end handle: builders push either to the
//! root list (the eventual `VisualGraph.elements`) or to a subgraph
//! slot in the arena. This avoids passing two distinct shapes
//! (`&mut Vec<VisualElement>` vs `&mut VisualSubgraph`) through
//! every helper.

use crate::visual_element::VisualElement;
use crate::visual_node::VisualNode;
use crate::visual_subgraph::VisualSubgraph;

/// Index of a node in [`BuildArena::nodes`].
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct NodeIdx(pub usize);

/// Index of a subgraph slot in [`BuildArena::subgraphs`].
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SubgraphIdx(pub usize);

/// One element in the arena-side nested tree: either a node handle
/// or a subgraph handle. The corresponding owned values live in
/// [`BuildArena::nodes`] / [`BuildArena::subgraphs`].
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ElementHandle {
    Node(NodeIdx),
    Subgraph(SubgraphIdx),
}

/// Write-end handle for "push this element into that container".
/// `Root` targets [`BuildArena::root_children`]; `Subgraph(idx)`
/// targets the slot at `idx` in [`BuildArena::subgraphs`].
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Container {
    Root,
    Subgraph(SubgraphIdx),
}

/// A single owned subgraph instance plus its handle list. The
/// `descriptor.elements` field stays empty for the entire build;
/// `finalize_root` is the only place that rehydrates it.
pub struct SubgraphSlot {
    pub descriptor: VisualSubgraph,
    pub children: Vec<ElementHandle>,
}

/// Flat arena holding every node, subgraph, and the root-level
/// handle list. Cleared once per `build_visual_graph` invocation.
#[derive(Default)]
pub struct BuildArena {
    pub nodes: Vec<VisualNode>,
    pub subgraphs: Vec<SubgraphSlot>,
    pub root_children: Vec<ElementHandle>,
}

impl BuildArena {
    pub fn new() -> Self {
        Self::default()
    }

    /// Park `node` and return the freshly-minted handle.
    pub fn push_node(&mut self, node: VisualNode) -> NodeIdx {
        let idx = NodeIdx(self.nodes.len());
        self.nodes.push(node);
        idx
    }

    /// Park `descriptor` as a new subgraph slot with an empty child
    /// list. The descriptor's own `elements` field must already be
    /// empty (callers set it that way in `describe_subgraph`).
    pub fn push_subgraph(&mut self, descriptor: VisualSubgraph) -> SubgraphIdx {
        let idx = SubgraphIdx(self.subgraphs.len());
        self.subgraphs.push(SubgraphSlot {
            descriptor,
            children: Vec::new(),
        });
        idx
    }

    pub fn node(&self, idx: NodeIdx) -> &VisualNode {
        &self.nodes[idx.0]
    }

    pub fn node_mut(&mut self, idx: NodeIdx) -> &mut VisualNode {
        &mut self.nodes[idx.0]
    }

    pub fn subgraph(&self, idx: SubgraphIdx) -> &SubgraphSlot {
        &self.subgraphs[idx.0]
    }

    pub fn subgraph_mut(&mut self, idx: SubgraphIdx) -> &mut SubgraphSlot {
        &mut self.subgraphs[idx.0]
    }

    /// Append `handle` to the given container's child list (root or
    /// subgraph slot).
    pub fn append_child(&mut self, container: Container, handle: ElementHandle) {
        match container {
            Container::Root => self.root_children.push(handle),
            Container::Subgraph(idx) => self.subgraphs[idx.0].children.push(handle),
        }
    }

    /// Prepend `handle` to the given container's child list. Used by
    /// the if-test anchor path that places the test anchor at the
    /// start of the consequent's element list.
    pub fn prepend_child(&mut self, container: Container, handle: ElementHandle) {
        match container {
            Container::Root => self.root_children.insert(0, handle),
            Container::Subgraph(idx) => self.subgraphs[idx.0].children.insert(0, handle),
        }
    }

    /// Move every node + subgraph into `Vec<VisualElement>`s rooted
    /// at `root_children` and the per-subgraph handle lists. The
    /// arena is consumed because each `VisualNode` /
    /// `VisualSubgraph` is owned and the call sites discard the
    /// arena afterwards.
    pub fn finalize_root(self) -> Vec<VisualElement> {
        let BuildArena {
            mut nodes,
            mut subgraphs,
            root_children,
        } = self;
        // The node `Vec` is consumed via `mem::take`d slot moves; we
        // wrap each value in `Option` so finalize can move a node
        // out without leaving a typed hole behind. Same trick for
        // the subgraph descriptor and children list.
        let mut node_slots: Vec<Option<VisualNode>> = nodes.drain(..).map(Some).collect();
        let mut sg_slots: Vec<Option<(VisualSubgraph, Vec<ElementHandle>)>> = subgraphs
            .drain(..)
            .map(|s| Some((s.descriptor, s.children)))
            .collect();
        let mut out = Vec::with_capacity(root_children.len());
        for handle in root_children {
            out.push(finalize_handle(handle, &mut node_slots, &mut sg_slots));
        }
        out
    }
}

fn finalize_handle(
    handle: ElementHandle,
    nodes: &mut [Option<VisualNode>],
    subgraphs: &mut [Option<(VisualSubgraph, Vec<ElementHandle>)>],
) -> VisualElement {
    match handle {
        ElementHandle::Node(NodeIdx(i)) => {
            let node = nodes[i]
                .take()
                .expect("BuildArena: node handle visited twice during finalize");
            VisualElement::Node(node)
        }
        ElementHandle::Subgraph(SubgraphIdx(i)) => {
            let (mut descriptor, children) = subgraphs[i]
                .take()
                .expect("BuildArena: subgraph handle visited twice during finalize");
            let mut rendered = Vec::with_capacity(children.len());
            for child in children {
                rendered.push(finalize_handle(child, nodes, subgraphs));
            }
            *descriptor.elements_mut() = rendered;
            VisualElement::Subgraph(descriptor)
        }
    }
}

#[cfg(test)]
#[path = "arena_test.rs"]
mod arena_test;
