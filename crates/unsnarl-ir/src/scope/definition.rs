//! Definition record.
//!
//! `node` / `parent` carry the materialised `AstNode` (type + span)
//! rather than parser-owned references: the IR outlives the parser
//! allocation. Richer parser-side data is re-derived at boundary
//! time rather than carried through the IR.
//!
//! The four `Option` extras (`init`, `declaration_kind`,
//! `import_source`, `imported_name`) carry serializer-required data
//! that the boundary reads off the AST at declaration time and which
//! cannot be reconstructed from `(node.type, span)` alone:
//!
//! * `init` — for `DefinitionType::Variable`, the
//!   `VariableDeclarator.init` initializer node's `(type, span)`.
//!   `None` when the declarator has no initializer (e.g. `let x;`).
//! * `declaration_kind` — for `DefinitionType::Variable`, the parent
//!   `VariableDeclaration.kind` (`var` / `let` / `const`).
//! * `import_source` — for `DefinitionType::ImportBinding`, the parent
//!   `ImportDeclaration.source.value` module string.
//! * `imported_name` — for named `DefinitionType::ImportBinding`
//!   specifiers, the original imported symbol name
//!   (`ImportSpecifier.imported.name` / `.value`).
//!
//! The fields stay flat (rather than collapsing into a
//! `DefinitionExtras` enum that mirrors `DefinitionType`) because
//! each is independently optional: for example, a `Variable`
//! declarator may have no initializer but still has a kind, so the
//! variants would not eliminate the `Option`s.

use unsnarl_oxc_parity::VariableDeclarationKind;

use crate::definition_type::DefinitionType;
use crate::primitive::{AstIdentifier, AstNode};

pub struct DefinitionData {
    pub r#type: DefinitionType,
    pub name: AstIdentifier,
    pub node: AstNode,
    pub parent: Option<AstNode>,
    pub init: Option<AstNode>,
    pub declaration_kind: Option<VariableDeclarationKind>,
    pub import_source: Option<String>,
    pub imported_name: Option<String>,
}

pub type Definition = DefinitionData;
