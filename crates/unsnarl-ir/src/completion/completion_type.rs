//! ECMA §6.2.4 Completion Record `[[Type]]` discriminator. Ports
//! `ts/src/ir/completion/completion-type.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CompletionType {
    Normal,
    Return,
    Throw,
    Break,
    Continue,
}
