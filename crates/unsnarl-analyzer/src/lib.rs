//! unsnarl-specific analysis: builds the side-table annotations
//! consumed by the serializers.
//!
//! Entry point [`run_analysis`] runs three phases in order — nesting
//! depths, the scope build via `unsnarl_oxc_boundary`, and a second
//! walk driven by [`build_analysis_visitor::BuildAnalysisVisitor`] —
//! and returns an
//! [`AnalyzedSource`] whose [`AnnotationsImpl`] satisfies the
//! `unsnarl_annotations::Annotations` lookup trait that serializers
//! consume. The remaining flat-listed siblings are per-routine
//! helpers the visitor calls into (`is_unused`, `is_control_exit`,
//! `abrupt_completion_type_of`, predicate / completion / container
//! lookups, the two `format_*` diagnostic renderers, etc.).
//!
//! [`owner`] is the only sub-directory. It groups three
//! owner-resolution helpers that share the same ancestor-walk
//! contract — `BindingPattern` flattening,
//! `AssignmentTarget` flattening, and locating the nearest owner
//! slot via [`owner::OwnerLookup`] — and is exposed through the
//! sibling `owner.rs` file per `docs/code-layout.md`'s
//! sibling-module rule. No other module in the crate currently
//! benefits from that finer-granularity grouping; if a future helper
//! cluster reaches the same coupling, it can follow the same
//! pattern.
//!
//! Functions that only need a node's `(type, span, key)` triple
//! take materialised `AstNode` values from `unsnarl-ir`; functions
//! that recurse into AST children (`is_control_exit`,
//! `abrupt_completion_type_of`, etc.) take the corresponding
//! `oxc_ast` references directly so the lifetime stays inside the
//! analyzer call site rather than being baked into the IR.

pub mod abrupt_completion_type_of;
pub mod analyzed_source;
pub mod annotations_impl;
pub mod block_context_of;
pub mod build_analysis_visitor;
pub mod build_head_expression;
pub mod case_exits_function;
pub mod case_falls_through;
pub mod compute_nesting_depths;
pub mod expression_statement_container;
pub mod find_completion;
pub mod find_jsx_element_span;
pub mod find_predicate_container;
pub mod format_case_test;
pub mod format_var_diagnostic;
pub mod if_chain_root_offset;
pub mod is_control_exit;
pub mod is_unused;
pub mod owner;
pub mod path_entry;
pub mod reference_call_receiver;
pub mod run_analysis;

#[cfg(test)]
pub(crate) mod testing;

pub use abrupt_completion_type_of::abrupt_completion_type_of;
pub use analyzed_source::AnalyzedSource;
pub use annotations_impl::AnnotationsImpl;
pub use block_context_of::block_context_of;
pub use build_head_expression::build_head_expression;
pub use case_exits_function::case_exits_function;
pub use case_falls_through::case_falls_through;
pub use compute_nesting_depths::compute_nesting_depths;
pub use expression_statement_container::{
    build_expression_statement_container, nearest_expression_statement,
};
pub use find_completion::find_completion;
pub use find_jsx_element_span::find_jsx_element_span;
pub use find_predicate_container::find_predicate_container;
pub use format_case_test::format_case_test;
pub use format_var_diagnostic::format_var_diagnostic;
pub use if_chain_root_offset::if_chain_root_offset;
pub use is_control_exit::is_control_exit;
pub use is_unused::is_unused;
pub use owner::{
    all_binding_variables, assignment_target_variables, locate_reference_owner_slot, OwnerLookup,
};
pub use path_entry::{ArrowBodyInfo, PathEntry};
pub use reference_call_receiver::reference_call_receiver_flags;
pub use run_analysis::run_analysis;
