//! Options passed into `Emitter::emit`.
//!
//! Mirrors `EmitOptions` in `ts/src/pipeline/emit/emit-options.ts`.
//! The TS shape carries fields for pruned graphs, root-query
//! resolutions, highlight sets, debug flags, and depths in addition
//! to `prettyJson`. The Rust port adds only `pretty_json` for now;
//! the rest are introduced as the corresponding visual-graph / json /
//! mermaid / markdown / stats emitters (Step 13 onward) require them.

pub struct EmitOptions {
    pub pretty_json: bool,
}
