//! Discriminator for `BlockContext` variants.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BlockContextKind {
    CaseClause,
    Other,
}
