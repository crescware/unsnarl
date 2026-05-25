//! Locate the nearest predicate-owning container along a reference's
//! ancestor chain.
//!
//! Walks the path leaf -> root tracking the slot key the previous
//! step occupied; reports the first ancestor whose `(type, key)`
//! pair identifies it as the predicate of an `if` / `switch` /
//! `while` / `do`-`while` / `for*` statement. The trailing `parent`
//! / `key` check covers the case where the reference itself is the
//! predicate slot of the immediate parent (the visitor has already
//! popped the predicate-owner off the path).

use unsnarl_ir::primitive::{SourceIndex, Utf8ByteOffset};
use unsnarl_ir::reference::PredicateContainer;
use unsnarl_oxc_parity::{AstType, PredicateContainerType};

use crate::path_entry::PathEntry;

const LOOP_HEADER_KEYS_FOR: &[&str] = &["init", "test", "update"];
const LOOP_HEADER_KEYS_FOR_OF_IN: &[&str] = &["left", "right"];

pub fn find_predicate_container(
    parent_type: Option<&AstType>,
    parent_offset: Option<Utf8ByteOffset>,
    key: Option<&str>,
    path: &[PathEntry],
    index: &SourceIndex<'_>,
) -> Option<PredicateContainer> {
    // `entry.node.span.start` and `parent_offset` arrive in UTF-8
    // byte units from oxc; the serialised `PredicateContainer.offset`
    // is in UTF-16 code units per the IR contract, so each candidate
    // offset is converted through `SourceIndex::span_at` before being
    // returned.
    let mut cur_key: Option<&str> = key;
    for entry in path.iter().rev() {
        let ty = &entry.node.r#type;
        if let Some(container_type) = predicate_container_for(ty, cur_key) {
            let offset = index.span_at(Utf8ByteOffset(entry.node.span.start)).offset;
            return Some(PredicateContainer::new(container_type, offset));
        }
        cur_key = entry.key;
    }
    let parent_type = parent_type?;
    let container_type = predicate_container_for(parent_type, key)?;
    let offset = index
        .span_at(parent_offset.unwrap_or(Utf8ByteOffset(0)))
        .offset;
    Some(PredicateContainer::new(container_type, offset))
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
