//! Flat serializer: walks the IR arena pre-order and emits a
//! `SerializedIR` whose scopes / variables / references are addressed
//! by string IDs (`scope#N`, `scope#N:name@offset`, `ref#N`).
//!
//! Mirrors `ts/src/serializer/flat/`.

pub mod collect_scopes_in_order;
pub mod flat_serializer;
pub mod has_declaring_def;
pub mod offset_of;
pub mod pick_variable_offset;
pub mod serialize_definition;
pub mod serialize_expression_statement_head;
pub mod serialize_reference;
pub mod serialize_scope;
pub mod serialize_variable;
pub mod span_of;

pub use flat_serializer::FlatSerializer;
