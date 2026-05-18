//! Reference-side IR contract types. Ports `ts/src/ir/reference/`.
//!
//! `Reference` / `ReferenceData` lives in this parent module rather
//! than a same-named child (`reference/reference.rs`) to avoid Rust's
//! `module_inception` shape. TS exposes `isWrite` / `isRead` /
//! `isReadOnly` / `isWriteOnly` / `isReadWrite` as methods on the
//! `Reference` interface and stores the underlying bitmask on
//! `ReferenceImpl`. The Rust IR merges those: `flags` lives on
//! `ReferenceData` directly.

use crate::ids::{ScopeId, VariableId};
use crate::primitive::AstIdentifier;

pub mod expression_statement_container;
pub mod expression_statement_head;
pub mod expression_statement_head_kind;
pub mod jsx_element_container;
pub mod predicate_container;
pub mod reference_completion;
pub mod reference_flags;

pub use expression_statement_container::ExpressionStatementContainer;
pub use expression_statement_head::{HeadExpression, HeadOperand};
pub use expression_statement_head_kind::ExpressionStatementHeadKind;
pub use jsx_element_container::JsxElementContainer;
pub use predicate_container::PredicateContainer;
pub use reference_completion::ReferenceCompletion;
pub use reference_flags::{ReferenceFlagBits, ReferenceFlags};

pub struct ReferenceData {
    pub identifier: AstIdentifier,
    pub from: ScopeId,
    pub resolved: Option<VariableId>,
    pub init: bool,
    pub flags: ReferenceFlagBits,
}

pub type Reference = ReferenceData;
