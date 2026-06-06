//! Reparent class-decorator references from the class's enclosing scope
//! to the class scope itself, matching the parity baseline.

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;

use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{ScopeData, VariableData};

/// Reparent class-decorator references from the class's enclosing scope
/// — where `oxc_semantic` records them, because decorators evaluate
/// before the class body is opened — to the class scope itself, matching
/// the parity baseline. A reference is moved (along with its entry on the
/// owning scope's `references` list) when its identifier span lies inside
/// a decorator's span.
pub(super) fn reparent_decorator_references(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
) {
    let scoping = semantic.scoping();
    let nodes = semantic.nodes();
    // Build node_id → IR scope id for nodes that anchor a scope.
    let mut node_to_ir_scope: std::collections::HashMap<oxc_semantic::NodeId, ScopeId> =
        std::collections::HashMap::new();
    for oxc_scope_id in scoping.scope_descendants_from_root() {
        let Some(ir) = translation[oxc_scope_id] else {
            continue;
        };
        let anchor = scoping.get_node_id(oxc_scope_id);
        node_to_ir_scope.entry(anchor).or_insert(ir);
    }
    // Collect (class_ir_scope, decorator_span) pairs.
    let mut decorator_spans: Vec<(ScopeId, Span)> = Vec::new();
    for node in nodes.iter() {
        let AstKind::Class(class) = node.kind() else {
            continue;
        };
        let Some(&class_ir) = node_to_ir_scope.get(&node.id()) else {
            continue;
        };
        for decorator in &class.decorators {
            decorator_spans.push((class_ir, decorator.span));
        }
    }
    if decorator_spans.is_empty() {
        return;
    }
    // Snapshot to avoid holding immutable borrow during the mutation
    // loop.
    let snapshots: Vec<(ReferenceId, ScopeId, Span)> = references
        .iter_enumerated()
        .map(|(id, r)| (id, r.from, r.identifier.span))
        .collect();
    for (ref_id, old_from, span) in snapshots {
        for (class_ir, dec_span) in &decorator_spans {
            if dec_span.start <= span.start && span.end <= dec_span.end {
                if old_from == *class_ir {
                    break;
                }
                scopes[old_from].references.retain(|&r| r != ref_id);
                scopes[*class_ir].references.push(ref_id);
                references[ref_id].from = *class_ir;
                // Note: `through` is only populated for unresolved /
                // implicit-global refs, which already include the
                // class scope on their chain; no fixup needed.
                let _ = variables;
                break;
            }
        }
    }
}
