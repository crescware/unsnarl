//! Emitter trait + `IRSerializer` trait + shared option types.
//!
//! Implementations of `Emitter` live in sibling `unsnarl-emitter-*`
//! crates. The only `IRSerializer` implementation, `FlatSerializer`,
//! lives in `unsnarl-emitter-ir`.

pub mod emit_options;
pub mod emitter;
pub mod ir_serializer;
pub mod serialize_context;

pub use emit_options::EmitOptions;
pub use emitter::Emitter;
pub use ir_serializer::IRSerializer;
pub use serialize_context::{SerializeContext, SerializeSourceMeta};

use unsnarl_ir::nesting_kind::NestingDepth;

/// Per-kind depth seeded when no `--depth*` flag is given. Lives
/// in the shared emitter crate (rather than the CLI crate) because
/// the markdown emitter's `format_depth_query` needs it too, and
/// the emitter layer is the lowest common dependency that both
/// consumers already pull in.
pub const DEFAULT_DEPTH: NestingDepth = NestingDepth(10);
