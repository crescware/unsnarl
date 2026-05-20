//! The plugin trait.
//!
//! Mirrors `UnsnarlPlugin` in
//! `ts/src/pipeline/plugin/unsnarl-plugin.ts`:
//!
//! ```ts
//! type UnsnarlPlugin = Readonly<{
//!   meta: Readonly<{ name: string }>;
//!   transform(ir: SerializedIR): SerializedIR;
//! }>;
//! ```
//!
//! The TS `meta.name` is folded into [`UnsnarlPlugin::name`]; the
//! `transform` function takes ownership of the IR and returns the
//! transformed IR. Ownership transfer mirrors the TS spread-and-
//! replace flow (`{ ...ir, ... }`) without forcing a `Clone` derive
//! on `SerializedIR`.

use unsnarl_ir::serialized::SerializedIR;

pub trait UnsnarlPlugin {
    /// Canonical plugin identifier. Mirrors TS `meta.name` (e.g.
    /// `"unsnarl-plugin-react"`).
    fn name(&self) -> &str;

    /// Apply the plugin's IR transformation. The TS analogue is
    /// `plugin.transform(ir)`.
    fn transform(&self, ir: SerializedIR) -> SerializedIR;
}
