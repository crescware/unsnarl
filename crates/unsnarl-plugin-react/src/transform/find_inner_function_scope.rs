//! Locate the inner `() => ...` / `function () {}` scope of a hook
//! call.
//!
//! Mirrors `findInnerFunctionScope` in
//! `ts/src/plugins/unsnarl-plugin-react/index.ts`. Iterates the
//! supplied sibling scope list (all scopes whose `upper` is the
//! wrapped variable's scope) and keeps the
//! `ArrowFunctionExpression` / `FunctionExpression` scope with the
//! smallest `block.span.offset` strictly greater than the
//! `CallExpression` start.

use unsnarl_ir::primitive::SourceOffset;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

pub fn find_inner_function_scope<'a>(
    siblings: &[&'a SerializedScope],
    call_offset: SourceOffset,
) -> Option<&'a SerializedScope> {
    let mut best: Option<&'a SerializedScope> = None;
    for s in siblings {
        if !matches!(
            s.block.r#type,
            AstType::ArrowFunctionExpression | AstType::FunctionExpression
        ) {
            continue;
        }
        if s.block.span.offset <= call_offset {
            continue;
        }
        match best {
            None => best = Some(s),
            Some(b) if s.block.span.offset < b.block.span.offset => best = Some(s),
            _ => {}
        }
    }
    best
}
