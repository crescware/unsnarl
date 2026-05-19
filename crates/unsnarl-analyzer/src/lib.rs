//! unsnarl-specific analysis: builds the side-table annotations
//! consumed by the serializers.
//!
//! Ports `ts/src/analyzer/`. The TS analyzer lives in a single
//! directory with ~20 modules plus an `owner/` sub-module; the Rust
//! port keeps the same module split. Functions that only need a
//! node's `(type, span, key)` triple take materialised `AstNode`
//! values from `unsnarl-ir`; functions that recurse into AST children
//! (`is_control_exit`, `abrupt_completion_type_of`, etc.) take the
//! corresponding `oxc_ast` references directly so the lifetime stays
//! inside the analyzer call site rather than being baked into the IR.

pub mod abrupt_completion_type_of;
pub mod annotations_impl;
pub mod block_context_of;
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
pub mod skip_types;

#[cfg(test)]
pub(crate) mod testing;

pub use abrupt_completion_type_of::abrupt_completion_type_of;
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
pub use skip_types::is_type_only_subtree;
