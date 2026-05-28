use unsnarl_annotations::{
    Annotations, ReferenceAnnotation, ReferenceAnnotationFlags, ScopeAnnotation, VariableAnnotation,
};
use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::reference::ReferenceCompletion;

use super::AnnotationsImpl;

fn rid(n: u32) -> ReferenceId {
    ReferenceId::from_usize(n as usize)
}

fn sid(n: u32) -> ScopeId {
    ScopeId::from_usize(n as usize)
}

fn vid(n: u32) -> VariableId {
    VariableId::from_usize(n as usize)
}

#[test]
fn missing_reference_returns_default_annotation() {
    let store = AnnotationsImpl::new();
    let r = store.of_reference(rid(0));
    assert!(!r.flags.call);
    assert!(!r.flags.receiver);
    assert!(r.owners.is_empty());
    assert!(r.predicate_container.is_none());
    assert!(matches!(r.completion, ReferenceCompletion::Normal));
    assert!(r.jsx_element.is_none());
    assert!(r.expression_statement_container.is_none());
}

#[test]
fn missing_scope_returns_default_annotation() {
    let store = AnnotationsImpl::new();
    let s = store.of_scope(sid(0));
    assert!(s.block_context.is_none());
    assert!(!s.falls_through);
    assert!(!s.exits_function);
    assert_eq!(s.nesting_depths.function, NestingDepth(0));
}

#[test]
fn missing_variable_returns_default_annotation() {
    let store = AnnotationsImpl::new();
    let v = store.of_variable(vid(0));
    assert!(!v.is_unused);
}

#[test]
fn set_reference_overrides_default() {
    let mut store = AnnotationsImpl::new();
    store.set_reference(
        rid(7),
        ReferenceAnnotation {
            owners: Vec::new(),
            flags: ReferenceAnnotationFlags {
                call: true,
                receiver: false,
            },
            predicate_container: None,
            completion: ReferenceCompletion::Normal,
            jsx_element: None,
            expression_statement_container: None,
        },
    );
    let r = store.of_reference(rid(7));
    assert!(r.flags.call);
    // Other IDs still see the default.
    let other = store.of_reference(rid(8));
    assert!(!other.flags.call);
}

#[test]
fn set_scope_overrides_default() {
    let mut store = AnnotationsImpl::new();
    store.set_scope(
        sid(3),
        ScopeAnnotation {
            block_context: None,
            falls_through: true,
            exits_function: false,
            nesting_depths: NestingDepths::uniform(NestingDepth(2)),
            abrupt_statements: Vec::new(),
        },
    );
    assert!(store.of_scope(sid(3)).falls_through);
    assert_eq!(
        store.of_scope(sid(3)).nesting_depths.function,
        NestingDepth(2)
    );
}

#[test]
fn set_variable_overrides_default() {
    let mut store = AnnotationsImpl::new();
    store.set_variable(vid(1), VariableAnnotation { is_unused: true });
    assert!(store.of_variable(vid(1)).is_unused);
    assert!(!store.of_variable(vid(2)).is_unused);
}
