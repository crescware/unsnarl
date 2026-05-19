//! Locate the nearest predicate-owning container along a reference's
//! ancestor chain.
//!
//! Mirrors `ts/src/analyzer/predicate.ts`. Walks the path leaf -> root
//! tracking the slot key the previous step occupied; reports the
//! first ancestor whose `(type, key)` pair identifies it as the
//! predicate of an `if` / `switch` / `while` / `do`-`while` / `for*`
//! statement. The trailing `parent` / `key` check covers the case
//! where the reference itself is the predicate slot of the immediate
//! parent (the visitor has already popped the predicate-owner off
//! the path).

use unsnarl_ir::reference::PredicateContainer;
use unsnarl_ir::SourceOffset;
use unsnarl_oxc_parity::{AstType, PredicateContainerType};

use crate::path_entry::PathEntry;

const LOOP_HEADER_KEYS_FOR: &[&str] = &["init", "test", "update"];
const LOOP_HEADER_KEYS_FOR_OF_IN: &[&str] = &["left", "right"];

pub fn find_predicate_container(
    parent_type: Option<&AstType>,
    parent_offset: Option<u32>,
    key: Option<&str>,
    path: &[PathEntry],
) -> Option<PredicateContainer> {
    let mut cur_key: Option<&str> = key;
    for entry in path.iter().rev() {
        let ty = &entry.node.r#type;
        let offset = SourceOffset(entry.node.span.start);
        if let Some(container_type) = predicate_container_for(ty, cur_key) {
            return Some(PredicateContainer {
                r#type: container_type,
                offset,
            });
        }
        cur_key = entry.key;
    }
    let parent_type = parent_type?;
    let parent_offset = SourceOffset(parent_offset.unwrap_or(0));
    let container_type = predicate_container_for(parent_type, key)?;
    Some(PredicateContainer {
        r#type: container_type,
        offset: parent_offset,
    })
}

fn predicate_container_for(ty: &AstType, key: Option<&str>) -> Option<PredicateContainerType> {
    let key = key?;
    match (ty, key) {
        (AstType::IfStatement, "test") => Some(PredicateContainerType::IfStatement),
        (AstType::SwitchStatement, "discriminant") => Some(PredicateContainerType::SwitchStatement),
        (AstType::WhileStatement, "test") => Some(PredicateContainerType::WhileStatement),
        (AstType::DoWhileStatement, "test") => Some(PredicateContainerType::DoWhileStatement),
        (AstType::ForStatement, k) if LOOP_HEADER_KEYS_FOR.contains(&k) => {
            Some(PredicateContainerType::ForStatement)
        }
        (AstType::ForOfStatement, k) if LOOP_HEADER_KEYS_FOR_OF_IN.contains(&k) => {
            Some(PredicateContainerType::ForOfStatement)
        }
        (AstType::ForInStatement, k) if LOOP_HEADER_KEYS_FOR_OF_IN.contains(&k) => {
            Some(PredicateContainerType::ForInStatement)
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "find_predicate_container_test.rs"]
mod find_predicate_container_test;
