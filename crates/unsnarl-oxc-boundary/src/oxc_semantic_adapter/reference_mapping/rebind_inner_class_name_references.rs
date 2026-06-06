//! Redirect references inside a class declaration scope from the outer
//! `ClassName` binding to the inner one synthesised by
//! `variable_mapping::push_inner_class_name`.

use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{ScopeData, VariableData};

/// Redirect references whose identifier sits inside a class declaration
/// scope from the outer `ClassName` binding to the inner one
/// synthesised by
/// [`super::super::variable_mapping::push_inner_class_name`].
///
/// `oxc_semantic` only binds a class declaration's name in the
/// enclosing scope, so any reference to that name from inside the
/// class body resolves to the outer binding. The parity baseline,
/// mirrored by `push_inner_class_name`, additionally exposes the name
/// on the class scope so references from method bodies (e.g.
/// `new C()` inside `class C { m() { ... } }`) resolve to the inner
/// row. Walk every reference; if its identifier span lies inside a
/// class scope that owns a synthesised inner binding sharing the
/// identifier's name, move the cross-link from the outer to the inner
/// variable and update `ReferenceData::resolved`.
///
/// When multiple inner-class scopes share the identifier's name and
/// each contains the reference span (e.g. nested `class C { method()
/// { class C { foo() { return C; } } } }`), the *innermost* enclosing
/// scope wins, mirroring lexical resolution. `inner_class_names` is
/// built in `scope_descendants_from_root` DFS order, so a naive walk
/// would otherwise pick the outermost match and bind references to
/// the wrong row.
pub(super) fn rebind_inner_class_name_references(
    scopes: &IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    references: &mut IndexVec<ReferenceId, ReferenceData>,
    inner_class_names: &[super::super::variable_mapping::InnerClassName],
) {
    if inner_class_names.is_empty() {
        return;
    }
    let snapshots: Vec<(ReferenceId, VariableId, String, Span)> = references
        .iter_enumerated()
        .filter_map(|(ref_id, r)| {
            r.resolved.map(|outer| {
                (
                    ref_id,
                    outer,
                    r.identifier.name().to_string(),
                    r.identifier.span,
                )
            })
        })
        .collect();
    for (ref_id, outer, name, span) in snapshots {
        let mut best: Option<(u32, &super::super::variable_mapping::InnerClassName)> = None;
        for icn in inner_class_names {
            if icn.inner == outer {
                continue;
            }
            if variables[icn.inner].name() != name {
                continue;
            }
            let class_span = scopes[icn.class_scope].block.span;
            if span.start < class_span.start || span.end > class_span.end {
                continue;
            }
            let width = class_span.end - class_span.start;
            if best.is_none_or(|(w, _)| width < w) {
                best = Some((width, icn));
            }
        }
        if let Some((_, icn)) = best {
            references[ref_id].resolved = Some(icn.inner);
            variables[outer].references.retain(|&id| id != ref_id);
            variables[icn.inner].references.push(ref_id);
        }
    }
}
