//! Discriminator for `BlockContext` variants. Ports
//! `ts/src/ir/scope/block-context-kind.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockContextKind {
    CaseClause,
    Other,
}
