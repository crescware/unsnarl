//! Test-only fixture helpers shared across the mermaid emitter's
//! sibling `_test.rs` files.
//!
//! Mirrors the family of `ts/src/emitter/mermaid/testing/make-*.ts`
//! files. The TS port keeps one helper per file; the Rust port
//! collapses them into a single module because the helpers are
//! consumed only by sibling unit tests and the inline grouping keeps
//! the file count low.
//!
//! Each `base_*` constructor returns the concrete struct (rather than
//! the wrapping enum) so callers can pin individual fields with
//! struct-update syntax and then wrap with the appropriate variant.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_edge::VisualEdge;
use unsnarl_visual_graph::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use unsnarl_visual_graph::visual_graph::{VisualGraph, VisualGraphSource};
use unsnarl_visual_graph::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode,
};
use unsnarl_visual_graph::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, OwnedExtras, OwnedSubgraphKind,
    OwnedVisualSubgraph,
};

use crate::render_state::RenderState;
use crate::strategy::MermaidStrategy;
use crate::theme::DARK_THEME;

// ---- Nodes ----------------------------------------------------------------

/// `ConstBinding` variant. Mirrors `baseNode()` in
/// `ts/.../testing/make-node.ts`.
pub fn base_const_binding() -> BindingVisualNode {
    BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_v".to_string(),
        name: "x".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    }
}

/// `VarBinding` variant. Mirrors `baseVarBindingNode()`.
pub fn base_var_binding() -> BindingVisualNode {
    BindingVisualNode {
        kind: BindingNodeKind::VarBinding,
        ..base_const_binding()
    }
}

/// `LetBinding` variant. Mirrors `baseLetBindingNode()`.
pub fn base_let_binding() -> BindingVisualNode {
    BindingVisualNode {
        kind: BindingNodeKind::LetBinding,
        ..base_const_binding()
    }
}

/// `WriteReference` synthetic node. Mirrors `baseWriteOpNode()`.
pub fn base_write_op() -> SyntheticVisualNode {
    SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_v".to_string(),
        kind: SyntheticNodeKind::WriteReference,
        name: "x".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::WriteOp {
            declaration_kind: None,
        },
    }
}

/// Base shape for synthetic kinds with no extras tail. The TS port's
/// `baseSimpleNode` mixes binding and synthetic kinds because the
/// TS `VisualNode` union is flat; the Rust port splits them across
/// two struct shapes, so the helper splits accordingly.
pub fn base_simple_synthetic(kind: SyntheticNodeKind) -> SyntheticVisualNode {
    SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_v".to_string(),
        kind,
        name: "x".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    }
}

/// Base shape for binding kinds that carry no extras tail
/// (FunctionDeclaration / ClassDeclaration / FormalParameter /
/// CatchParameter / SyntheticImplicitGlobal /
/// DefaultImportBinding / NamespaceImportBinding).
///
/// For Var / Const / Let / NamedImport, prefer the dedicated
/// constructors below — those carry the correct extras shape.
pub fn base_simple_binding(kind: BindingNodeKind) -> BindingVisualNode {
    BindingVisualNode {
        kind,
        extras: BindingExtras::None {},
        ..base_const_binding()
    }
}

/// `NamedImportBinding` with the supplied `importedName`.
pub fn base_import_binding_named(imported_name: &str) -> BindingVisualNode {
    BindingVisualNode {
        kind: BindingNodeKind::NamedImportBinding,
        extras: BindingExtras::NamedImport {
            imported_name: imported_name.to_string(),
        },
        ..base_const_binding()
    }
}

/// `DefaultImportBinding`. Mirrors `baseImportBindingDefault()`.
pub fn base_import_binding_default() -> BindingVisualNode {
    base_simple_binding(BindingNodeKind::DefaultImportBinding)
}

/// `NamespaceImportBinding`. Mirrors `baseImportBindingNamespace()`.
pub fn base_import_binding_namespace() -> BindingVisualNode {
    base_simple_binding(BindingNodeKind::NamespaceImportBinding)
}

// ---- Subgraphs ------------------------------------------------------------

/// `Function` subgraph (owned shape). Mirrors `baseSubgraph()` in
/// `ts/.../testing/make-subgraph.ts`.
pub fn base_function_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_x".to_string(),
        kind: OwnedSubgraphKind::Function,
        line: 1,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "owner".to_string(),
        },
        elements: Vec::new(),
    }
}

/// `Case` subgraph (control shape) with `caseTest: null`.
pub fn base_case_subgraph() -> ControlVisualSubgraph {
    ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_x".to_string(),
        line: 1,
        end_line: None,
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::Case,
        extras: ControlExtras::Case { case_test: None },
    }
}

/// `Class` subgraph (owned shape) with `className: null`.
pub fn base_class_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_x".to_string(),
        kind: OwnedSubgraphKind::Class,
        line: 1,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::Class { class_name: None },
        elements: Vec::new(),
    }
}

/// `IfElseContainer` subgraph (owned shape) with `hasElse: false`.
pub fn base_if_else_container_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_x".to_string(),
        kind: OwnedSubgraphKind::IfElseContainer,
        line: 1,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::IfElseContainer { has_else: false },
        elements: Vec::new(),
    }
}

/// Subgraphs whose shape uses the "control" field order (elements
/// before kind) and no extras tail — `Switch` / `If` / `Else` /
/// `Try` / `Catch` / `Finally` / `For` / `While` / `DoWhile` /
/// `Block`. Mirrors `basePlainSubgraph(kind)`.
pub fn base_plain_subgraph(kind: ControlSubgraphKind) -> ControlVisualSubgraph {
    ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_x".to_string(),
        line: 1,
        end_line: None,
        direction: Direction::RL,
        elements: Vec::new(),
        kind,
        extras: ControlExtras::None {},
    }
}

// ---- Edges / Graphs ------------------------------------------------------

/// Mirrors `baseEdge()`.
pub fn base_edge() -> VisualEdge {
    VisualEdge {
        from: "a".to_string(),
        to: "b".to_string(),
        label: "read".to_string(),
    }
}

/// Mirrors `baseGraph()` — empty `elements` / `edges` / `boundary_edges`
/// with `pruning = None`, language Ts, direction RL.
pub fn base_graph() -> VisualGraph {
    VisualGraph {
        version: SERIALIZED_IR_VERSION,
        source: VisualGraphSource {
            path: "input.ts".to_string(),
            language: Language::Ts,
        },
        direction: Direction::RL,
        elements: Vec::new(),
        edges: Vec::new(),
        boundary_edges: Vec::new(),
        pruning: None,
    }
}

// ---- RenderState ---------------------------------------------------------

/// Mirrors `baseRenderState()` — dark theme, dagre strategy, every
/// collection empty.
///
/// `'a` stays free here so the caller can populate `node_map` with
/// borrows that live in the test scope; the empty `HashMap` does not
/// pin the lifetime.
pub fn base_render_state<'a>() -> RenderState<'a> {
    RenderState {
        lines: Vec::new(),
        node_map: HashMap::new(),
        wrapped_owner_ids: HashSet::new(),
        placeholder_ids: Vec::new(),
        nest_class_map: HashMap::new(),
        strategy: MermaidStrategy::Dagre,
        theme: &DARK_THEME,
        debug: false,
    }
}
