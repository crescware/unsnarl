//! Resolve the `[[Type]]` Completion category that carries a
//! reference's value.
//!
//! Walks the ancestor chain leaf -> root and reports the nearest
//! enclosing `ReturnStatement` / `ThrowStatement`, or `Normal` if a
//! function / class boundary is hit first (function / class
//! internals never feed the enclosing function's completion). Arrow
//! functions with an expression body (no explicit `return`) classify
//! the body expression itself as the implicit return target.

use unsnarl_ir::reference::ReferenceCompletion;
use unsnarl_ir::SourceOffset;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;

pub fn find_completion(path: &[PathEntry]) -> ReferenceCompletion {
    for entry in path.iter().rev() {
        match entry.node.r#type {
            AstType::ReturnStatement => {
                return ReferenceCompletion::Return {
                    start_offset: SourceOffset(entry.node.span.start),
                    end_offset: SourceOffset(entry.node.span.end),
                };
            }
            AstType::ThrowStatement => {
                return ReferenceCompletion::Throw {
                    start_offset: SourceOffset(entry.node.span.start),
                    end_offset: SourceOffset(entry.node.span.end),
                };
            }
            AstType::ArrowFunctionExpression => {
                // Block-body arrows defer to an inner ReturnStatement (already
                // handled by the deeper path entry). Expression-body arrows
                // have no explicit return: the body expression itself is the
                // implicit return target.
                if let Some(body) = entry.arrow_body {
                    if !body.is_block {
                        return ReferenceCompletion::Return {
                            start_offset: SourceOffset(body.span.start),
                            end_offset: SourceOffset(body.span.end),
                        };
                    }
                }
                return ReferenceCompletion::Normal;
            }
            AstType::FunctionExpression | AstType::FunctionDeclaration => {
                return ReferenceCompletion::Normal;
            }
            AstType::ClassExpression | AstType::ClassDeclaration => {
                return ReferenceCompletion::Normal;
            }
            _ => {}
        }
    }
    ReferenceCompletion::Normal
}

#[cfg(test)]
#[path = "find_completion_test.rs"]
mod find_completion_test;
