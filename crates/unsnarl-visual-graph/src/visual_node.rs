//! `VisualNode`: a single rendered node in the visual graph.
//!
//! The on-disk JSON field order depends on the construction site:
//! `make_variable_node` writes
//! `{ type, id, name, line, end_line, is_jsx_element, unused,
//! kind, ...extras }`, while every synthetic builder
//! (`ensure_expression_statement_node`, the loop / switch / if
//! anchors, the module / intermediate / sink builders, the
//! WriteOp builder, `ensure_return_use_node` /
//! `ensure_throw_use_node` / `ensure_beyond_depth_stub`) writes
//! `{ type, id, kind, name, line, end_line, is_jsx_element,
//! unused, ...extras }`. Two struct shapes preserve both orders
//! and the construction site picks the right one.

use serde::Serialize;
use unsnarl_oxc_parity::VariableDeclarationKind;

use crate::visual_element_type::NodeTypeTag;

mod accessors;
mod binding_constructors;
mod synthetic_constructors;

/// `kind` for a [`BindingVisualNode`]. The set is the subset of
/// `NodeKind` that `makeVariableNode` can produce (variable-shaped
/// declarations + the implicit-global synthetic).
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BindingNodeKind {
    VarBinding,
    ConstBinding,
    LetBinding,
    UsingBinding,
    AwaitUsingBinding,
    FunctionDeclaration,
    ClassDeclaration,
    FormalParameter,
    CatchParameter,
    NamedImportBinding,
    DefaultImportBinding,
    NamespaceImportBinding,
    SyntheticImplicitGlobal,
}

/// Extras tail of a [`BindingVisualNode`]. Flattened on top of the
/// common fields immediately after `kind`, so they end up at the
/// tail of the JSON object.
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

/// Node shape produced by `make_variable_node` — the common fields
/// come first, then `kind`, then the kind-specific tail.
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
    SyntheticConditionalTest,
    SyntheticWhileStatementTest,
    SyntheticDoWhileStatementTest,
    SyntheticForStatementHeader,
    SyntheticForInStatementHeader,
    SyntheticForOfStatementHeader,
    SyntheticModuleSink,
    SyntheticImportIntermediate,
    SyntheticExpressionStatement,
    SyntheticBeyondDepth,
    SyntheticBreakStatement,
    SyntheticContinueStatement,
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

impl From<BindingVisualNode> for VisualNode {
    fn from(n: BindingVisualNode) -> Self {
        Self::Binding(n)
    }
}

impl From<SyntheticVisualNode> for VisualNode {
    fn from(n: SyntheticVisualNode) -> Self {
        Self::Synthetic(n)
    }
}

#[cfg(test)]
#[path = "visual_node_test.rs"]
mod visual_node_test;
