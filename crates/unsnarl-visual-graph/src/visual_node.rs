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

impl BindingVisualNode {
    fn base(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
        kind: BindingNodeKind,
        extras: BindingExtras,
    ) -> Self {
        Self {
            r#type: NodeTypeTag::Node,
            id: id.into(),
            name: name.into(),
            line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            kind,
            extras,
        }
    }

    pub fn const_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::ConstBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn var_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::VarBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn let_binding(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::LetBinding,
            BindingExtras::Variable {
                init_is_function: false,
            },
        )
    }

    pub fn function_declaration(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::FunctionDeclaration,
            BindingExtras::None {},
        )
    }

    pub fn class_declaration(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::ClassDeclaration,
            BindingExtras::None {},
        )
    }

    pub fn formal_parameter(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::FormalParameter,
            BindingExtras::None {},
        )
    }

    pub fn catch_parameter(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::CatchParameter,
            BindingExtras::None {},
        )
    }

    pub fn named_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        imported_name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::NamedImportBinding,
            BindingExtras::NamedImport {
                imported_name: imported_name.into(),
            },
        )
    }

    pub fn default_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::DefaultImportBinding,
            BindingExtras::None {},
        )
    }

    pub fn namespace_import_binding(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            BindingNodeKind::NamespaceImportBinding,
            BindingExtras::None {},
        )
    }
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

impl SyntheticVisualNode {
    fn base(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
        kind: SyntheticNodeKind,
        extras: SyntheticExtras,
    ) -> Self {
        Self {
            r#type: NodeTypeTag::Node,
            id: id.into(),
            kind,
            name: name.into(),
            line,
            end_line: None,
            is_jsx_element: false,
            unused: false,
            extras,
        }
    }

    pub fn write_reference(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::WriteReference,
            SyntheticExtras::WriteOp {
                declaration_kind: None,
            },
        )
    }

    pub fn return_argument_reference(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::ReturnArgumentReference,
            SyntheticExtras::None {},
        )
    }

    pub fn throw_argument_reference(
        id: impl Into<String>,
        name: impl Into<String>,
        line: u32,
    ) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::ThrowArgumentReference,
            SyntheticExtras::None {},
        )
    }

    pub fn module_source(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticModuleSource,
            SyntheticExtras::None {},
        )
    }

    pub fn module_sink(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticModuleSink,
            SyntheticExtras::None {},
        )
    }

    pub fn import_intermediate(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticImportIntermediate,
            SyntheticExtras::None {},
        )
    }

    pub fn expression_statement(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticExpressionStatement,
            SyntheticExtras::None {},
        )
    }

    pub fn implicit_global(id: impl Into<String>, name: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            name,
            line,
            SyntheticNodeKind::SyntheticBeyondDepth,
            SyntheticExtras::None {},
        )
    }

    pub fn if_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "if-test",
            line,
            SyntheticNodeKind::SyntheticIfStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn switch_discriminant(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "switch-discriminant",
            line,
            SyntheticNodeKind::SyntheticSwitchStatementDiscriminant,
            SyntheticExtras::None {},
        )
    }

    pub fn while_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "while-test",
            line,
            SyntheticNodeKind::SyntheticWhileStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn do_while_statement_test(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "do-while-test",
            line,
            SyntheticNodeKind::SyntheticDoWhileStatementTest,
            SyntheticExtras::None {},
        )
    }

    pub fn for_statement_header(id: impl Into<String>, line: u32, kind: SyntheticNodeKind) -> Self {
        Self::base(id, "for-test", line, kind, SyntheticExtras::None {})
    }

    pub fn beyond_depth(id: impl Into<String>, line: u32) -> Self {
        Self::base(
            id,
            "...",
            line,
            SyntheticNodeKind::SyntheticBeyondDepth,
            SyntheticExtras::None {},
        )
    }
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

    /// Returns the logical [`NodeKind`] discriminator.
    ///
    /// The two underlying shapes (binding / synthetic) carry their
    /// own subset enums so the on-disk JSON field order can differ;
    /// this projects them back onto a single flat [`NodeKind`] so
    /// consumers (mermaid emitter, markdown emitter, ...) can match
    /// in one switch.
    pub fn kind(&self) -> crate::node_kind::NodeKind {
        use crate::node_kind::NodeKind;
        match self {
            Self::Binding(n) => match n.kind {
                BindingNodeKind::VarBinding => NodeKind::VarBinding,
                BindingNodeKind::ConstBinding => NodeKind::ConstBinding,
                BindingNodeKind::LetBinding => NodeKind::LetBinding,
                BindingNodeKind::FunctionDeclaration => NodeKind::FunctionDeclaration,
                BindingNodeKind::ClassDeclaration => NodeKind::ClassDeclaration,
                BindingNodeKind::FormalParameter => NodeKind::FormalParameter,
                BindingNodeKind::CatchParameter => NodeKind::CatchParameter,
                BindingNodeKind::NamedImportBinding => NodeKind::NamedImportBinding,
                BindingNodeKind::DefaultImportBinding => NodeKind::DefaultImportBinding,
                BindingNodeKind::NamespaceImportBinding => NodeKind::NamespaceImportBinding,
                BindingNodeKind::SyntheticImplicitGlobal => NodeKind::SyntheticImplicitGlobal,
            },
            Self::Synthetic(n) => match n.kind {
                SyntheticNodeKind::WriteReference => NodeKind::WriteReference,
                SyntheticNodeKind::ReturnArgumentReference => NodeKind::ReturnArgumentReference,
                SyntheticNodeKind::ThrowArgumentReference => NodeKind::ThrowArgumentReference,
                SyntheticNodeKind::SyntheticIfStatementTest => NodeKind::SyntheticIfStatementTest,
                SyntheticNodeKind::SyntheticSwitchStatementDiscriminant => {
                    NodeKind::SyntheticSwitchStatementDiscriminant
                }
                SyntheticNodeKind::SyntheticWhileStatementTest => {
                    NodeKind::SyntheticWhileStatementTest
                }
                SyntheticNodeKind::SyntheticDoWhileStatementTest => {
                    NodeKind::SyntheticDoWhileStatementTest
                }
                SyntheticNodeKind::SyntheticForStatementHeader => {
                    NodeKind::SyntheticForStatementHeader
                }
                SyntheticNodeKind::SyntheticForInStatementHeader => {
                    NodeKind::SyntheticForInStatementHeader
                }
                SyntheticNodeKind::SyntheticForOfStatementHeader => {
                    NodeKind::SyntheticForOfStatementHeader
                }
                SyntheticNodeKind::SyntheticModuleSink => NodeKind::SyntheticModuleSink,
                SyntheticNodeKind::SyntheticModuleSource => NodeKind::SyntheticModuleSource,
                SyntheticNodeKind::SyntheticImportIntermediate => {
                    NodeKind::SyntheticImportIntermediate
                }
                SyntheticNodeKind::SyntheticExpressionStatement => {
                    NodeKind::SyntheticExpressionStatement
                }
                SyntheticNodeKind::SyntheticBeyondDepth => NodeKind::SyntheticBeyondDepth,
            },
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Binding(n) => &n.name,
            Self::Synthetic(n) => &n.name,
        }
    }

    pub fn line(&self) -> u32 {
        match self {
            Self::Binding(n) => n.line,
            Self::Synthetic(n) => n.line,
        }
    }

    pub fn end_line(&self) -> Option<u32> {
        match self {
            Self::Binding(n) => n.end_line,
            Self::Synthetic(n) => n.end_line,
        }
    }

    pub fn is_jsx_element(&self) -> bool {
        match self {
            Self::Binding(n) => n.is_jsx_element,
            Self::Synthetic(n) => n.is_jsx_element,
        }
    }

    pub fn unused(&self) -> bool {
        match self {
            Self::Binding(n) => n.unused,
            Self::Synthetic(n) => n.unused,
        }
    }

    /// `importedName` (JSON field), present only on
    /// `NamedImportBinding`.
    pub fn imported_name(&self) -> Option<&str> {
        match self {
            Self::Binding(n) => match &n.extras {
                BindingExtras::NamedImport { imported_name } => Some(imported_name.as_str()),
                _ => None,
            },
            Self::Synthetic(_) => None,
        }
    }

    /// `declarationKind` (JSON field), present only on
    /// `WriteReference`. Serialised as `null` when the underlying
    /// variable is not a declaration (e.g. an implicit-global
    /// reassignment); that maps to `Some(None)` at the JSON level
    /// but the caller only cares whether a concrete kind is
    /// present, so the flattened `Option<...>` form is returned here.
    pub fn declaration_kind(&self) -> Option<VariableDeclarationKind> {
        match self {
            Self::Synthetic(n) => match &n.extras {
                SyntheticExtras::WriteOp { declaration_kind } => declaration_kind.clone(),
                _ => None,
            },
            Self::Binding(_) => None,
        }
    }

    /// `initIsFunction` (JSON field), present only on
    /// `VarBinding` / `ConstBinding` / `LetBinding`. Returns `false`
    /// for every other kind so callers can branch without a separate
    /// presence check.
    pub fn init_is_function(&self) -> bool {
        match self {
            Self::Binding(n) => match &n.extras {
                BindingExtras::Variable { init_is_function } => *init_is_function,
                _ => false,
            },
            Self::Synthetic(_) => false,
        }
    }
}

#[cfg(test)]
#[path = "visual_node_test.rs"]
mod visual_node_test;
