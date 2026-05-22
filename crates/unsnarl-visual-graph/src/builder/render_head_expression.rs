//! Renders the structural head mini-AST to a compact one-line
//! display label. `Raw` segments fall back to slicing the original
//! source so non-recognised shapes appear verbatim. `Elided`
//! segments collapse the corresponding operand to `"..."` so a
//! shape like `C.z = 1` shows up as `C.z = ...` without dragging
//! the literal RHS into the diagram.

use unsnarl_ir::serialized::SerializedHeadExpression;

pub fn render_head_expression(head: &SerializedHeadExpression, raw: &str) -> String {
    match head {
        SerializedHeadExpression::Identifier { name } => name.clone(),
        SerializedHeadExpression::Member { object, property } => {
            format!("{}.{property}", render_head_expression(object, raw))
        }
        SerializedHeadExpression::Call { callee } => {
            format!("{}()", render_head_expression(callee, raw))
        }
        SerializedHeadExpression::New { callee } => {
            format!("new {}()", render_head_expression(callee, raw))
        }
        SerializedHeadExpression::Await { argument } => {
            format!("await {}", render_head_expression(argument, raw))
        }
        SerializedHeadExpression::Assign {
            operator,
            left,
            right,
        } => {
            let left_text = render_head_expression(&left.head, raw);
            let right_text = render_head_expression(&right.head, raw);
            format!("{left_text} {} {right_text}", operator.as_str())
        }
        SerializedHeadExpression::Update {
            operator,
            prefix,
            argument,
        } => {
            let arg = render_head_expression(&argument.head, raw);
            if *prefix {
                format!("{}{arg}", operator.as_str())
            } else {
                format!("{arg}{}", operator.as_str())
            }
        }
        SerializedHeadExpression::Elided => "...".to_string(),
        SerializedHeadExpression::Raw {
            start_span,
            end_span,
        } => raw_slice_utf16(raw, start_span.offset.0, end_span.offset.0),
    }
}

/// Slice `raw` between two UTF-16 offsets. The IR carries offsets
/// in UTF-16 code units (see `unsnarl_ir::primitive::span_from_offset`),
/// so naive UTF-8 byte slicing would mis-cut when the source
/// contains multi-byte characters.
fn raw_slice_utf16(raw: &str, start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut buf = [0u16; 2];
    let mut consumed: u32 = 0;
    for ch in raw.chars() {
        let unit_count = ch.encode_utf16(&mut buf).len() as u32;
        let next = consumed + unit_count;
        if next <= start {
            consumed = next;
            continue;
        }
        if consumed >= end {
            break;
        }
        result.push(ch);
        consumed = next;
    }
    result
}

#[cfg(test)]
#[path = "render_head_expression_test.rs"]
mod render_head_expression_test;
