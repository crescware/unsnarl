//! On-disk schema version of `SerializedIR`. Bump every time the
//! serialized shape changes so downstream consumers can switch on it.

pub const SERIALIZED_IR_VERSION: u32 = 1;

pub type SerializedIrVersion = u32;
