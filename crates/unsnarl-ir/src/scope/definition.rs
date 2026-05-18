//! Definition record. Ports `ts/src/ir/scope/definition.ts`.
//!
//! TS holds raw `AstNode` references in `node` / `parent`; the Rust
//! IR can't keep the parser-owned objects alive, so we store the
//! materialised `AstNode` (type + span). Richer parser-side data is
//! re-derived at boundary time rather than carried through the IR.

use crate::definition_type::DefinitionType;
use crate::primitive::{AstIdentifier, AstNode};

pub struct DefinitionData {
    pub r#type: DefinitionType,
    pub name: AstIdentifier,
    pub node: AstNode,
    pub parent: Option<AstNode>,
}

/// Public alias used by analyzers that read definitions out of the
/// arena. Matches the TS `Definition` type name.
pub type Definition = DefinitionData;
