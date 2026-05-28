//! Set of `[[Type]]` Completion Record values reachable at a
//! statement's termination.
//!
//! Returns `None` when any path through the statement can fall
//! through to a normal completion.
//!
//! `LabeledStatement` follows ECMA §14.13.4 LabelledStatement
//! Runtime Semantics in full: step 2 inherits the body's
//! completion, step 3 collapses Break / Continue with a `[[Target]]`
//! matching the labelled statement's label to Normal. The internal
//! [`AbruptOutcome`] enum is the discriminator that preserves the
//! Break / Continue label until the LabelledStatement frame can
//! match it; the public surface still returns `Vec<CompletionType>`
//! (deduplicated) because downstream consumers
//! (`case_exits_function`, the visualizer's classification rows)
//! only care about the `[[Type]]` set.
//!
//! Set membership uses `Vec` rather than a hash set because the
//! value space is fixed-size (four `CompletionType` variants) and
//! comparison happens via `contains` over at most four entries.

use oxc_ast::ast::Statement;

use unsnarl_ir::completion::CompletionType;

/// Internal companion to [`CompletionType`] that keeps the Break /
/// Continue `[[Target]]` (the label name, or `None` for a bare
/// `break;` / `continue;`) until the enclosing `LabelledStatement`
/// frame either matches and collapses it or lets it propagate up.
#[derive(Clone)]
enum AbruptOutcome {
    Return,
    Throw,
    Break(Option<String>),
    Continue(Option<String>),
}

impl AbruptOutcome {
    fn to_completion_type(&self) -> CompletionType {
        match self {
            Self::Return => CompletionType::Return,
            Self::Throw => CompletionType::Throw,
            Self::Break(_) => CompletionType::Break,
            Self::Continue(_) => CompletionType::Continue,
        }
    }

    fn matches_label(&self, label: &str) -> bool {
        match self {
            Self::Break(Some(t)) | Self::Continue(Some(t)) => t == label,
            _ => false,
        }
    }
}

fn same_outcome(a: &AbruptOutcome, b: &AbruptOutcome) -> bool {
    match (a, b) {
        (AbruptOutcome::Return, AbruptOutcome::Return)
        | (AbruptOutcome::Throw, AbruptOutcome::Throw) => true,
        (AbruptOutcome::Break(x), AbruptOutcome::Break(y))
        | (AbruptOutcome::Continue(x), AbruptOutcome::Continue(y)) => x == y,
        _ => false,
    }
}

fn compute_outcomes(node: &Statement<'_>) -> Option<Vec<AbruptOutcome>> {
    match node {
        Statement::ReturnStatement(_) => Some(vec![AbruptOutcome::Return]),
        Statement::ThrowStatement(_) => Some(vec![AbruptOutcome::Throw]),
        Statement::BreakStatement(b) => Some(vec![AbruptOutcome::Break(
            b.label.as_ref().map(|l| l.name.as_str().to_string()),
        )]),
        Statement::ContinueStatement(c) => Some(vec![AbruptOutcome::Continue(
            c.label.as_ref().map(|l| l.name.as_str().to_string()),
        )]),
        Statement::BlockStatement(block) => block.body.last().and_then(compute_outcomes),
        Statement::IfStatement(if_stmt) => {
            let alternate = if_stmt.alternate.as_ref()?;
            let cons = compute_outcomes(&if_stmt.consequent)?;
            let alt = compute_outcomes(alternate)?;
            let mut out = cons;
            for o in alt {
                if !out.iter().any(|existing| same_outcome(existing, &o)) {
                    out.push(o);
                }
            }
            Some(out)
        }
        Statement::LabeledStatement(ls) => {
            let label = ls.label.name.as_str();
            let body = compute_outcomes(&ls.body)?;
            // ECMA §14.13.4 step 3: filter out Break / Continue
            // whose [[Target]] matches this labelled statement's
            // label. A bare break / continue has no target and
            // therefore never matches.
            let filtered: Vec<AbruptOutcome> = body
                .into_iter()
                .filter(|o| !o.matches_label(label))
                .collect();
            if filtered.is_empty() {
                None
            } else {
                Some(filtered)
            }
        }
        _ => None,
    }
}

pub fn abrupt_completion_type_of(node: &Statement<'_>) -> Option<Vec<CompletionType>> {
    let outcomes = compute_outcomes(node)?;
    let mut out: Vec<CompletionType> = Vec::new();
    for o in outcomes {
        let t = o.to_completion_type();
        if !out.iter().any(|existing| same_completion(existing, &t)) {
            out.push(t);
        }
    }
    Some(out)
}

fn same_completion(a: &CompletionType, b: &CompletionType) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[cfg(test)]
#[path = "abrupt_completion_type_of_test.rs"]
mod abrupt_completion_type_of_test;
