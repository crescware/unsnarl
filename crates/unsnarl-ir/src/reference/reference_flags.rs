//! Read / Write bitmask.
//!
//! `ReferenceFlagBits` is a newtype over `u32` so a flag bitset
//! cannot be confused with other 32-bit IR scalars (source offsets,
//! depth counters, schema version, ...). `BitOr` / `BitAnd` and
//! their `Assign` siblings are forwarded to the inner `u32` so
//! callers can compose flags with `READ | WRITE` ergonomically.
//! `#[serde(transparent)]` keeps the on-disk JSON shape a bare
//! number.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ReferenceFlagBits(pub u32);

impl BitOr for ReferenceFlagBits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ReferenceFlagBits {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for ReferenceFlagBits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for ReferenceFlagBits {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

pub struct ReferenceFlags;

impl ReferenceFlags {
    pub const NONE: ReferenceFlagBits = ReferenceFlagBits(0);
    pub const READ: ReferenceFlagBits = ReferenceFlagBits(1 << 0);
    pub const WRITE: ReferenceFlagBits = ReferenceFlagBits(1 << 1);
}
