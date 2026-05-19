//! Set of `[[Type]]` Completion Record values reachable at a
//! statement's termination.
//!
//! Mirrors `ts/src/analyzer/abrupt-completion-type-of.ts`. Returns
//! `None` when any path through the statement can fall through to a
//! normal completion. `LabeledStatement` is intentionally not
//! resolved (the TS source documents this limitation; the parity
//! port preserves it).
//!
//! Set membership uses `Vec` rather than a hash set because the
//! value space is fixed-size (four `CompletionType` variants) and
//! comparison happens via `contains` over at most four entries; the
//! TS source uses `Set<...>` for clarity but the analyzer never
//! observes its hashing properties.

use oxc_ast::ast::Statement;

use unsnarl_ir::completion::CompletionType;

pub fn abrupt_completion_type_of(node: &Statement<'_>) -> Option<Vec<CompletionType>> {
    match node {
        Statement::ReturnStatement(_) => Some(vec![CompletionType::Return]),
        Statement::ThrowStatement(_) => Some(vec![CompletionType::Throw]),
        Statement::BreakStatement(_) => Some(vec![CompletionType::Break]),
        Statement::ContinueStatement(_) => Some(vec![CompletionType::Continue]),
        Statement::BlockStatement(block) => block.body.last().and_then(abrupt_completion_type_of),
        Statement::IfStatement(if_stmt) => {
            let alternate = if_stmt.alternate.as_ref()?;
            let cons = abrupt_completion_type_of(&if_stmt.consequent)?;
            let alt = abrupt_completion_type_of(alternate)?;
            let mut out = cons;
            for t in alt {
                if !out.iter().any(|existing| same_completion(existing, &t)) {
                    out.push(t);
                }
            }
            Some(out)
        }
        _ => None,
    }
}

fn same_completion(a: &CompletionType, b: &CompletionType) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[cfg(test)]
#[path = "abrupt_completion_type_of_test.rs"]
mod abrupt_completion_type_of_test;
