//! `Direction`: layout direction tag carried by VisualGraph / VisualSubgraph.
//!
//! The four constants (`RL` / `LR` / `TB` / `BT`) are Mermaid layout
//! keywords; the enum serializes to those bare strings.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Direction {
    RL,
    LR,
    TB,
    BT,
}

impl Direction {
    /// The bare Mermaid keyword form (`RL` / `LR` / `TB` / `BT`).
    /// Used when emitters need to splice the direction into a
    /// `flowchart <dir>` or `direction <dir>` line.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RL => "RL",
            Self::LR => "LR",
            Self::TB => "TB",
            Self::BT => "BT",
        }
    }
}
