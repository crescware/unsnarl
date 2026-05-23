//! JSX-element wrapping info for a `Reference`.

use crate::primitive::Utf8ByteOffset;

pub struct JsxElementContainer {
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
}
