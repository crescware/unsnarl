//! Accessors for the [`VisualNode`] enum: uniform readers that project
//! both underlying shapes onto a single interface.

use unsnarl_oxc_parity::VariableDeclarationKind;

use super::{BindingExtras, BindingNodeKind, SyntheticExtras, SyntheticNodeKind, VisualNode};

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
                BindingNodeKind::UsingBinding => NodeKind::UsingBinding,
                BindingNodeKind::AwaitUsingBinding => NodeKind::AwaitUsingBinding,
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
                SyntheticNodeKind::SyntheticImportIntermediate => {
                    NodeKind::SyntheticImportIntermediate
                }
                SyntheticNodeKind::SyntheticExpressionStatement => {
                    NodeKind::SyntheticExpressionStatement
                }
                SyntheticNodeKind::SyntheticBeyondDepth => NodeKind::SyntheticBeyondDepth,
                SyntheticNodeKind::SyntheticBreakStatement => NodeKind::SyntheticBreakStatement,
                SyntheticNodeKind::SyntheticContinueStatement => {
                    NodeKind::SyntheticContinueStatement
                }
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
