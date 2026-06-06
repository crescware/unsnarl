//! UTF-16 offsets and lines of a `Call` / `New` head node.

use unsnarl_ir::serialized::SerializedHeadExpression;

/// UTF-16 start/end offsets and start/end lines of a `Call` / `New`
/// head node. `None` for any other kind.
pub fn call_node_extent(node: &SerializedHeadExpression) -> Option<(u32, u32, u32, Option<u32>)> {
    let (start_span, end_span) = match node {
        SerializedHeadExpression::Call {
            start_span,
            end_span,
            ..
        }
        | SerializedHeadExpression::New {
            start_span,
            end_span,
            ..
        } => (start_span, end_span),
        _ => return None,
    };
    let start_line = start_span.line.0;
    let end_line = (end_span.line.0 != start_line).then_some(end_span.line.0);
    Some((start_span.offset.0, end_span.offset.0, start_line, end_line))
}
