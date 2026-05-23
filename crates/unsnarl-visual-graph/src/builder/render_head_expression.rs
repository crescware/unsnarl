//! Renders the structural head mini-AST to a compact one-line
//! display label. `Raw` segments fall back to slicing the original
//! source so non-recognised shapes appear verbatim. `Elided`
//! segments collapse the corresponding operand to `"..."` so a
//! shape like `C.z = 1` shows up as `C.z = ...` without dragging
//! the literal RHS into the diagram.

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::SerializedHeadExpression;

pub fn render_head_expression(
    head: &SerializedHeadExpression,
    source_index: &SourceIndex<'_>,
) -> String {
    match head {
        SerializedHeadExpression::Identifier { name } => name.clone(),
        SerializedHeadExpression::Member { object, property } => {
            format!(
                "{}.{property}",
                render_head_expression(object, source_index)
            )
        }
        SerializedHeadExpression::Call { callee } => {
            format!("{}()", render_head_expression(callee, source_index))
        }
        SerializedHeadExpression::New { callee } => {
            format!("new {}()", render_head_expression(callee, source_index))
        }
        SerializedHeadExpression::Await { argument } => {
            format!("await {}", render_head_expression(argument, source_index))
        }
        SerializedHeadExpression::Assign {
            operator,
            left,
            right,
        } => {
            let left_text = render_head_expression(&left.head, source_index);
            let right_text = render_head_expression(&right.head, source_index);
            format!("{left_text} {} {right_text}", operator.as_str())
        }
        SerializedHeadExpression::Update {
            operator,
            prefix,
            argument,
        } => {
            let arg = render_head_expression(&argument.head, source_index);
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
        } => source_index
            .slice_utf16(start_span.offset, end_span.offset)
            .to_string(),
    }
}

#[cfg(test)]
#[path = "render_head_expression_test.rs"]
mod render_head_expression_test;
