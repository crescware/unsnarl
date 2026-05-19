//! Wrapping JSXElement span lookup for a reference.
//!
//! Mirrors `ts/src/analyzer/jsx-element-span.ts`. Walks the ancestor
//! chain leaf -> root, skipping `JSXMemberExpression` segments; when
//! the immediate ancestor is a `JSXOpeningElement` and the one above
//! it is a `JSXElement`, returns the element's span. Anything else
//! short-circuits to `None`.

use unsnarl_ir::reference::JsxElementContainer;
use unsnarl_ir::SourceOffset;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;

pub fn find_jsx_element_span(path: &[PathEntry]) -> Option<JsxElementContainer> {
    if path.is_empty() {
        return None;
    }
    let mut i = path.len() - 1;
    loop {
        let entry = &path[i];
        match entry.node.r#type {
            AstType::JSXOpeningElement => {
                if i == 0 {
                    return None;
                }
                let element_entry = &path[i - 1];
                if !matches!(element_entry.node.r#type, AstType::JSXElement) {
                    return None;
                }
                return Some(JsxElementContainer {
                    start_offset: SourceOffset(element_entry.node.span.start),
                    end_offset: SourceOffset(element_entry.node.span.end),
                });
            }
            AstType::JSXMemberExpression => {
                if i == 0 {
                    return None;
                }
                i -= 1;
                continue;
            }
            _ => return None,
        }
    }
}

#[cfg(test)]
#[path = "find_jsx_element_span_test.rs"]
mod find_jsx_element_span_test;
