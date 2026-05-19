//! Source language tag.

use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Ts,
    Tsx,
    Js,
    Jsx,
}
