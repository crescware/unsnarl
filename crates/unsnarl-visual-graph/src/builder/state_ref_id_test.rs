//! Sibling tests for [`state_ref_id`]. Cases mirror
//! `ts/src/visual-graph/builder/state-ref-id.test.ts`.

use super::state_ref_id;
use crate::builder::testing::{
    base_builder_context, base_serialized_reference, base_write_op, empty_serialized_ir,
    reference_id, span_offset_line,
};
use crate::builder::write_op::WriteOp;
use unsnarl_ir::serialized::serialized_reference::SerializedReferenceIdentifier;
use unsnarl_ir::serialized::{SerializedIR, SerializedReference};

fn write_op_at(ref_id: &str, var_id: &str, offset: u32) -> WriteOp {
    WriteOp {
        ref_id: ref_id.to_string(),
        var_id: var_id.to_string(),
        offset,
        ..base_write_op()
    }
}

#[test]
fn ref_id_that_names_a_writeop_returns_the_writeops_node_id() {
    let ir = empty_serialized_ir();
    let mut ctx = base_builder_context(&ir);
    let op = write_op_at("wRef", "v", 10);
    ctx.write_op_by_ref.insert("wRef".to_string(), op);
    assert_eq!(state_ref_id("wRef", "v", &ctx), "wr_wRef");
}

fn ir_with_refs(refs: Vec<SerializedReference>) -> SerializedIR {
    let mut ir = empty_serialized_ir();
    ir.references = refs;
    ir
}

#[test]
fn reference_not_found_in_ir_references_returns_node_id_of_var() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    assert_eq!(state_ref_id("missing", "v", &ctx), "n_v");
}

#[test]
fn reference_exists_but_no_prior_writes_returns_node_id_of_var() {
    let mut r = base_serialized_reference();
    r.id = reference_id("readRef");
    r.identifier = SerializedReferenceIdentifier::new("x".to_string(), span_offset_line(20, 1));
    let ir = ir_with_refs(vec![r]);
    let mut ctx = base_builder_context(&ir);
    ctx.write_ops_by_variable
        .insert("v".to_string(), Vec::new());
    assert_eq!(state_ref_id("readRef", "v", &ctx), "n_v");
}

#[test]
fn reference_exists_with_prior_write_returns_write_op_node_id() {
    let mut r = base_serialized_reference();
    r.id = reference_id("readRef");
    r.identifier = SerializedReferenceIdentifier::new("x".to_string(), span_offset_line(20, 1));
    let ir = ir_with_refs(vec![r]);
    let mut ctx = base_builder_context(&ir);
    let earlier = write_op_at("wEarlier", "v", 5);
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![earlier]);
    assert_eq!(state_ref_id("readRef", "v", &ctx), "wr_wEarlier");
}
