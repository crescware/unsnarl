//! Render the **callee** portion of an expression-statement's
//! [`SerializedHeadExpression`] -- i.e. the text that appears
//! *before* the call's argument parentheses.
//!
//! Used by the callback-argument labelling pass to build a
//! self-contained subgraph header of the form
//! `<callee>(args[<N>])<br/>L_start-end` for a function scope
//! that occupies the `arg_index`-th argument slot of an
//! [`ExpressionStatement`]-level call.
//!
//! Unlike [`render_head_expression`], which would render a
//! `Call { callee }` shape as `<callee>()` (i.e. with trailing
//! parens), this helper returns just the callee subtree so the
//! caller can append a single owned `(args[N])` suffix without
//! producing the ambiguous `run()(args[0])` shape (which would
//! parse as "the result of `run()` called with `args[0]`").
//!
//! Returns `None` for head shapes that do not have a recognisable
//! call layer at their root (e.g. bare `Identifier` / `Member`
//! references, `Assign` / `Update`). `Await { argument: Call { … } }`
//! is unwrapped exactly once so a top-level
//! `await foo(cb)` still produces `foo` as its callee. Deeper
//! nesting (`await await foo(cb)`, member chains whose outermost
//! shape is itself a call, etc.) is left as `None` for now.

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::SerializedHeadExpression;

use super::render_head_expression::render_head_expression;

pub fn render_call_callee(
    head: &SerializedHeadExpression,
    source_index: &SourceIndex<'_>,
) -> Option<String> {
    match head {
        SerializedHeadExpression::Call { callee } => {
            Some(render_head_expression(callee, source_index))
        }
        SerializedHeadExpression::New { callee } => Some(format!(
            "new {}",
            render_head_expression(callee, source_index)
        )),
        SerializedHeadExpression::Await { argument } => match argument.as_ref() {
            SerializedHeadExpression::Call { callee } => {
                Some(render_head_expression(callee, source_index))
            }
            SerializedHeadExpression::New { callee } => Some(format!(
                "new {}",
                render_head_expression(callee, source_index)
            )),
            _ => None,
        },
        _ => None,
    }
}
