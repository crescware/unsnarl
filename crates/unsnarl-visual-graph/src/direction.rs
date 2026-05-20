//! `Direction`: layout direction tag carried by VisualGraph / VisualSubgraph.
//!
//! Mirrors `ts/src/visual-graph/direction.ts`. The four constants
//! (`RL` / `LR` / `TB` / `BT`) are Mermaid layout keywords. The TS
//! shape is a string union; in Rust it's an enum that serializes
//! to those bare strings.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Direction {
    RL,
    LR,
    TB,
    BT,
}
