//! Source language tag. Ports `ts/src/language.ts`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ts,
    Tsx,
    Js,
    Jsx,
}
