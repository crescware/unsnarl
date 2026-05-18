//! ImportBinding subkind. Ports `ts/src/serializer/import-kind.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportKind {
    Default,
    Named,
    Namespace,
}
