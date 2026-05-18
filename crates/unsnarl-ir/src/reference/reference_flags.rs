//! Read / Write bitmask. Ports `ts/src/ir/reference/reference-flags.ts`.

pub type ReferenceFlagBits = u32;

pub struct ReferenceFlags;

impl ReferenceFlags {
    pub const NONE: ReferenceFlagBits = 0;
    pub const READ: ReferenceFlagBits = 1 << 0;
    pub const WRITE: ReferenceFlagBits = 1 << 1;
}
