//! Resolves a reference's predicate-container target id by routing
//! `PredicateContainerType` to the matching anchor map. The maps
//! themselves are populated by `attach_loop_test_anchor` /
//! `attach_switch_discriminant_anchor` / `build_children`
//! (if-test pushes).

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedReference;
use unsnarl_oxc_parity::PredicateContainerType;

use super::timing::TimingScope;

/// Anchor-map handles the `predicate_target_id` lookup needs. Each
/// field is a `(offset → anchor-node-id)` map carried by the
/// builder state. Kept as a single bundle so callers do not have
/// to pass five maps individually.
pub struct PredicateAnchorMaps<'a> {
    pub if_test: &'a HashMap<u32, String>,
    pub switch_discriminant: &'a HashMap<u32, String>,
    pub while_test: &'a HashMap<u32, String>,
    pub do_while_test: &'a HashMap<u32, String>,
    pub for_test: &'a HashMap<u32, String>,
}

pub fn predicate_target_id(
    r: &SerializedReference,
    anchors: &PredicateAnchorMaps<'_>,
) -> Option<String> {
    let _t = TimingScope::start("predicate_target_id");
    let pc = r.predicate_container.as_ref()?;
    let offset = pc.offset.0;
    let map = match pc.r#type {
        PredicateContainerType::SwitchStatement => anchors.switch_discriminant,
        PredicateContainerType::WhileStatement => anchors.while_test,
        PredicateContainerType::DoWhileStatement => anchors.do_while_test,
        PredicateContainerType::ForStatement
        | PredicateContainerType::ForOfStatement
        | PredicateContainerType::ForInStatement => anchors.for_test,
        PredicateContainerType::IfStatement => anchors.if_test,
    };
    map.get(&offset).cloned()
}

#[cfg(test)]
#[path = "predicate_target_id_test.rs"]
mod predicate_target_id_test;
