//! `VisualNode`: a single rendered node in the visual graph.
//!
//! Mirrors `ts/src/visual-graph/visual-node.ts` and the construction
//! sites under `ts/src/visual-graph/builder/`. The TS file declares
//! a single discriminated union, but the on-disk JSON field order
//! depends on the construction site: `makeVariableNode` builds the
//! object as `{ type, id, name, line, endLine, isJsxElement, unused,
//! kind, ...extras }` while every synthetic builder
//! (`ensureExpressionStatementNode`, the loop / switch / if
//! anchors, the module / intermediate / sink builders, the
//! WriteOp builder, `ensureReturnUseNode` / `ensureThrowUseNode` /
//! `ensureBeyondDepthStub`) writes `{ type, id, kind, name, line,
//! endLine, isJsxElement, unused, ...extras }`. To preserve those
//! two orders we host two struct shapes and pick at construction.

use serde::Serialize;
use unsnarl_oxc_parity::VariableDeclarationKind;

use crate::visual_element_type::NodeTypeTag;

/// `kind` for a [`BindingVisualNode`]. The set is the subset of
/// `NodeKind` that `makeVariableNode` can produce (variable-shaped
/// declarations + the implicit-global synthetic).
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BindingNodeKind {
    VarBinding,
    ConstBinding,
    LetBinding,
    FunctionDeclaration,
    ClassDeclaration,
    FormalParameter,
    CatchParameter,
    NamedImportBinding,
    DefaultImportBinding,
    NamespaceImportBinding,
    SyntheticImplicitGlobal,
}

/// Extras tail of a [`BindingVisualNode`]. The TS shape spreads
/// these on top of the common fields immediately after `kind`, so
/// they end up at the tail of the JSON object.
///
/// `#[serde(untagged)]` so serde picks the variant by structure and
/// flattens its fields without emitting a tag of its own.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum BindingExtras {
    /// FunctionDeclaration / ClassDeclaration / FormalParameter /
    /// CatchParameter / DefaultImportBinding /
    /// NamespaceImportBinding / SyntheticImplicitGlobal: no tail.
    None {},
    /// Var / Const / Let binding: carries `initIsFunction`.
    Variable {
        #[serde(rename = "initIsFunction")]
        init_is_function: bool,
    },
    /// `NamedImportBinding`: carries `importedName`.
    NamedImport {
        #[serde(rename = "importedName")]
        imported_name: String,
    },
}

/// Node shape produced by `makeVariableNode` in TS — the common
/// fields come first, then `kind`, then the kind-specific tail.
#[derive(Clone, Serialize)]
pub struct BindingVisualNode {
    #[serde(rename = "type")]
    pub r#type: NodeTypeTag,
    pub id: String,
    pub name: String,
    pub line: u32,
    #[serde(rename = "endLine")]
    pub end_line: Option<u32>,
    #[serde(rename = "isJsxElement")]
    pub is_jsx_element: bool,
    pub unused: bool,
    pub kind: BindingNodeKind,
    #[serde(flatten)]
    pub extras: BindingExtras,
}

/// `kind` for a [`SyntheticVisualNode`]. The set is the subset of
/// `NodeKind` that the various `ensure-*` / anchor / WriteOp
/// builders produce (everything that is *not* a variable
/// declaration).
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SyntheticNodeKind {
    WriteReference,
    ReturnArgumentReference,
    ThrowArgumentReference,
    SyntheticIfStatementTest,
    SyntheticSwitchStatementDiscriminant,
    SyntheticWhileStatementTest,
    SyntheticDoWhileStatementTest,
    SyntheticForStatementHeader,
    SyntheticForInStatementHeader,
    SyntheticForOfStatementHeader,
    SyntheticModuleSink,
    SyntheticModuleSource,
    SyntheticImportIntermediate,
    SyntheticExpressionStatement,
    SyntheticBeyondDepth,
}

/// Extras tail for [`SyntheticVisualNode`]. The only synthetic node
/// that carries one is `WriteReference`, which mirrors the
/// underlying variable's `declarationKind`.
#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum SyntheticExtras {
    None {},
    WriteOp {
        #[serde(rename = "declarationKind")]
        declaration_kind: Option<VariableDeclarationKind>,
    },
}

/// Node shape produced by every synthetic builder — `kind` sits
/// directly after `id`, then the rest of the common fields, then
/// the optional tail.
#[derive(Clone, Serialize)]
pub struct SyntheticVisualNode {
    #[serde(rename = "type")]
    pub r#type: NodeTypeTag,
    pub id: String,
    pub kind: SyntheticNodeKind,
    pub name: String,
    pub line: u32,
    #[serde(rename = "endLine")]
    pub end_line: Option<u32>,
    #[serde(rename = "isJsxElement")]
    pub is_jsx_element: bool,
    pub unused: bool,
    #[serde(flatten)]
    pub extras: SyntheticExtras,
}

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum VisualNode {
    Binding(BindingVisualNode),
    Synthetic(SyntheticVisualNode),
}

impl VisualNode {
    pub fn id(&self) -> &str {
        match self {
            Self::Binding(n) => &n.id,
            Self::Synthetic(n) => &n.id,
        }
    }

    pub fn set_unused(&mut self, unused: bool) {
        match self {
            Self::Binding(n) => n.unused = unused,
            Self::Synthetic(n) => n.unused = unused,
        }
    }
}

#[cfg(test)]
#[path = "visual_node_test.rs"]
mod visual_node_test;
