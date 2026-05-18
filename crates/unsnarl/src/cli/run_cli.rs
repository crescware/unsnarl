//! Mirrors `ts/src/cli/run-cli/` (parent module).

pub mod emit_out_flag_notice;
pub mod resolve_output_path;

pub use emit_out_flag_notice::emit_out_flag_notice;
pub use resolve_output_path::derive_output_basename;
