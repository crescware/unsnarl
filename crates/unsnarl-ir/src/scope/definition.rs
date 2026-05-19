//! Definition record.
//!
//! `node` / `parent` carry the materialised `AstNode` (type + span)
//! rather than parser-owned references: the IR outlives the parser
//! allocation. Richer parser-side data is re-derived at boundary
//! time rather than carried through the IR.

use crate::definition_type::DefinitionType;
use crate::primitive::{AstIdentifier, AstNode};

pub struct DefinitionData {
    pub r#type: DefinitionType,
    pub name: AstIdentifier,
    pub node: AstNode,
    pub parent: Option<AstNode>,
}

pub type Definition = DefinitionData;
