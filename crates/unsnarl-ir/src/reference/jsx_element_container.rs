//! JSX-element wrapping info for a `Reference`. Ports
//! `ts/src/ir/reference/jsx-element-container.ts`.

pub struct JsxElementContainer {
    pub start_offset: u32,
    pub end_offset: u32,
}
