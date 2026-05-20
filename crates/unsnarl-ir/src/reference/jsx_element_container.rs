//! JSX-element wrapping info for a `Reference`.

use crate::primitive::SourceOffset;

pub struct JsxElementContainer {
    pub start_offset: SourceOffset,
    pub end_offset: SourceOffset,
}
