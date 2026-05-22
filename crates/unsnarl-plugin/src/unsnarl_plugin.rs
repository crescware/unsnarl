//! The plugin trait.
//!
//! `transform` takes ownership of the IR and returns the transformed
//! IR, so the spread-and-replace flow does not require a `Clone`
//! derive on `SerializedIR`.

use unsnarl_ir::serialized::SerializedIR;

pub trait UnsnarlPlugin {
    /// Canonical plugin identifier (e.g. `"unsnarl-plugin-react"`).
    fn name(&self) -> &str;

    /// Apply the plugin's IR transformation.
    fn transform(&self, ir: SerializedIR) -> SerializedIR;
}
