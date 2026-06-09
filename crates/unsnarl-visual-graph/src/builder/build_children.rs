//! Walks `parent_scope.child_scopes` and dispatches each child to
//! [`build_scope`]. Branches of a single `if`/`else if`/`else`
//! chain — siblings whose `branch_container_key` starts with
//! `"if:"` and matches — are grouped under a single
//! `IfElseContainer` wrapping subgraph so the rendered diagram
//! shows one merged container per chain.
//!
//! For each `if` / `else if` arm the matching test anchor is
//! placed at the head of the consequent subgraph it gates. When the
//! consequent collapsed past the depth threshold it has no subgraph
//! to host the anchor, so the anchor is dropped. The `else`
//! (alternate) branch carries no test of its own. An `if`-only
//! statement (no `else`) is treated as a lone branch and rendered
//! without the `IfElseContainer` wrapping.

use std::collections::HashMap;

use unsnarl_ir::primitive::Utf16CodeUnitOffset;
use unsnarl_ir::serialized::{SerializedCallbackHostKind, SerializedScope};
use unsnarl_oxc_parity::AstType;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::attach_test_anchor_to_consequent::attach_test_anchor_to_consequent;
use super::branch_container_key::branch_container_key;
use super::build_conditional_group::build_conditional_group;
use super::build_scope::build_scope;
use super::callback_chain_target::{callback_chain_target, ChainHost};
use super::context::BuilderContext;
use super::ensure_assignment_call_proxy::{ensure_assignment_call_proxy, ReassignmentBinding};
use super::ensure_call_proxy_wrapper::ensure_call_proxy_wrapper;
use super::ensure_host_call_proxy::ensure_host_call_proxy;
use super::ensure_return_call_proxy::ensure_return_call_proxy;
use super::if_container_subgraph_id::if_container_subgraph_id;
use super::innermost_chain_proxy_id::innermost_chain_proxy_id;
use super::is_collapsed::is_collapsed;
use super::line_for_offset::line_for_offset;
use super::result_var_for_host::result_var_for_host;
use super::route_collapsed_callback_to_stub::route_collapsed_callback_to_stub;
use super::state::BuildState;
use super::write_op_node_for_assignment::write_op_node_for_assignment;

/// The source span of `scope` when it is a synthesised ternary arm
/// (`? then` / `: else`), else `None`. Lets callers gate a ternary arm's
/// proxy redirect to reads inside that arm (see
/// `BuildState::result_proxy_arm_span` / `ternary_callback_arm_spans`).
fn conditional_arm_span(scope: &SerializedScope) -> Option<(u32, u32)> {
    let c = scope.block_context.as_ref()?;
    if matches!(c.parent_type(), AstType::ConditionalExpression)
        && (c.key() == "consequent" || c.key() == "alternate")
    {
        Some((scope.block.span.offset.0, scope.block.end_span.offset.0))
    } else {
        None
    }
}

pub fn build_children(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    parent_scope: &SerializedScope,
    container: Container,
) {
    // Collect children in source order. We hold owned references
    // (`&SerializedScope`) borrowed from the IR via `scope_map`.
    let children: Vec<&SerializedScope> = parent_scope
        .child_scopes
        .iter()
        .filter_map(|id| ctx.scope_map.get(id.value()).copied())
        .collect();

    // `ExpressionStatement start offset → CallProxy wrapper`.
    // Populated lazily inside the main loop: the wrapper is
    // appended to `container` at the position of its *first*
    // callback-arg child, and every subsequent callback child
    // belonging to the same ExpressionStatement is routed into the
    // existing wrapper. Allocating at first-sight (rather than in a
    // separate pre-pass over `children`) preserves the source order
    // of sibling scopes that interleave with the callback statement
    // -- `[block, callback, block]` now renders as
    // `[block, wrapper, block]` instead of being reordered to
    // `[wrapper, block, block]` by a pre-pass that always appends
    // wrappers first.
    let mut call_proxy_by_stmt_offset: HashMap<u32, SubgraphIdx> = HashMap::new();
    // `host start offset → result-bound CallProxy wrapper`, the
    // non-statement counterpart of `call_proxy_by_stmt_offset`. Keyed
    // by the host's bound-expression start so every callback within one
    // binding (including nested argument calls) shares a single proxy.
    let mut call_proxy_by_host: HashMap<u32, SubgraphIdx> = HashMap::new();
    // `(call start, call end) → nested CallProxy` for the inner calls of
    // a method chain (`arr.map(f).filter(g)`). Keyed by call span so the
    // map / filter calls each get their own proxy nested inside the host
    // proxy, keeping `f` and `g` in distinct block scopes.
    let mut nested_call_proxy: HashMap<(u32, u32), SubgraphIdx> = HashMap::new();

    let mut i = 0;
    while i < children.len() {
        let child = children[i];
        // A callback collapsed past the depth ceiling has its interior
        // omitted. Building a CallProxy here would either leave an empty
        // husk or add a box -- the opposite of what pruning is for. Let
        // `build_scope` record the collapse and emit the BeyondDepth stub,
        // then point the call's host edges at that stub so the receiver /
        // binding relationship survives as `<input> -> ((...))` instead of
        // being re-minted onto a return-use node.
        if let Some(cb) = child.callback_argument.as_ref() {
            if is_collapsed(child, ctx.depths.as_ref()) {
                build_scope(arena, state, ctx, child, container);
                route_collapsed_callback_to_stub(state, ctx, child, cb.host.as_ref(), parent_scope);
                i += 1;
                continue;
            }
        }
        // Callback-arg children route into a `CallProxy` wrapper.
        // The wrapper is created on first sight (so it lands at the
        // first callback's source position) and reused for any
        // later siblings that share the same statement offset.
        if child.callback_argument.is_some() {
            let host = child
                .callback_argument
                .as_ref()
                .and_then(|cb| cb.host.as_ref());
            // A reassignment host (`y = arr.map(cb)`) takes priority over
            // the enclosing-statement path below: `y = arr.map(cb);` is
            // itself an ExpressionStatement, so the statement path would
            // label the proxy `y = arr.map()` and strand the receiver on
            // the write-op node. Handling it as an assignment instead
            // keeps reassignment parallel to declaration
            // (`const xs = arr.map(cb)`). Only a plain-identifier target
            // resolves to a single write-op node; member / destructuring
            // targets carry no `target_span` and fall through to the
            // statement / island paths.
            if let Some(host) = host {
                if matches!(host.kind, SerializedCallbackHostKind::Assignment) {
                    if let Some(write_op_node) = write_op_node_for_assignment(host, ctx) {
                        let stmt_offset = ctx
                            .expression_statement_index
                            .enclosing(child.block.span.offset.0, child.block.end_span.offset.0)
                            .map(|s| s.start_span.offset.0);
                        let binding = ReassignmentBinding {
                            write_op_node,
                            stmt_offset,
                        };
                        let wrapper_idx = ensure_assignment_call_proxy(
                            arena,
                            state,
                            ctx,
                            container,
                            &mut call_proxy_by_host,
                            host,
                            &binding,
                        );
                        let target = callback_chain_target(
                            arena,
                            state,
                            ctx,
                            container,
                            ChainHost {
                                proxy: wrapper_idx,
                                head: &host.head,
                            },
                            (child.block.span.offset.0, child.block.end_span.offset.0),
                            &mut nested_call_proxy,
                        );
                        // Route the chain's input (the innermost receiver)
                        // to the innermost proxy so the backbone runs
                        // `arr -> arr.map() -> ... -> [write]`.
                        if let Some(inner) =
                            innermost_chain_proxy_id(arena, &host.head, &nested_call_proxy)
                        {
                            // Gate a ternary-arm reassignment proxy to its
                            // hosting arm so the sibling arm's value keeps
                            // its edge to the write-op node.
                            if let Some(span) = conditional_arm_span(parent_scope) {
                                state
                                    .result_proxy_write_op_arm_span
                                    .insert(binding.write_op_node.clone(), span);
                            }
                            state
                                .result_proxy_by_write_op
                                .insert(binding.write_op_node.clone(), inner);
                        }
                        build_scope(arena, state, ctx, child, Container::Subgraph(target));
                        i += 1;
                        continue;
                    }
                }
            }
            // The CallProxy wrapper is a rendering construct that
            // reuses the `expr_stmt_<offset>` leaf, so it fires when
            // the callback's block span is contained by some registered
            // (non-synthetic) `ExpressionStatement` -- that is what
            // `enclosing` actually tests, not "is this callback's own
            // call statement-level?". The two coincide when the
            // callback's own call is the nearest enclosing statement,
            // but a variable-bound / returned callback nested inside an
            // outer statement-level call's body is also contained and so
            // also routes into that outer statement's wrapper. That
            // correlation is a visual-layer concern: resolve it here from
            // the `ExpressionStatement` spans the builder already owns,
            // rather than reading it off the IR annotation.
            if let Some(statement) = ctx
                .expression_statement_index
                .enclosing(child.block.span.offset.0, child.block.end_span.offset.0)
            {
                let wrapper_idx = ensure_call_proxy_wrapper(
                    arena,
                    state,
                    ctx,
                    container,
                    &mut call_proxy_by_stmt_offset,
                    statement,
                );
                // A statement-level method chain (`arr.map(f).filter(g);`)
                // splits its callbacks into a sibling backbone the same
                // way the bound case does, using the statement head's
                // receiver chain.
                let target = callback_chain_target(
                    arena,
                    state,
                    ctx,
                    container,
                    ChainHost {
                        proxy: wrapper_idx,
                        head: &statement.head,
                    },
                    (child.block.span.offset.0, child.block.end_span.offset.0),
                    &mut nested_call_proxy,
                );
                // Route the chain's receiver read (e.g. `arr`) to the
                // innermost proxy so the backbone runs
                // `arr -> arr.map() -> ...`. The callbacks' returns no
                // longer ride this claim -- they resolve to per-call
                // return-use nodes (see `resolve_read_target_id`) -- so
                // re-pointing it only moves the receiver, not the bodies.
                if let Some(inner) =
                    innermost_chain_proxy_id(arena, &statement.head, &nested_call_proxy)
                {
                    state
                        .expression_statement_by_offset
                        .insert(statement.start_span.offset.0, inner);
                }
                // When this statement-hosted callback lives in a ternary
                // arm (`enabled ? items.map(cb) : other;`), record the arm
                // span so the sibling arm's plain value reads are not
                // pulled onto this statement's container (they flow to the
                // ternary's consumer instead — see
                // `BuildState::ternary_callback_arm_spans`).
                if let Some(span) = conditional_arm_span(parent_scope) {
                    state.ternary_callback_arm_spans.push(span);
                }
                build_scope(arena, state, ctx, child, Container::Subgraph(target));
                i += 1;
                continue;
            }
            // Not a statement-level call. If the call's result is bound
            // to a variable (`const xs = arr.map(cb)`, including a call
            // nested in arguments `const x = foo(data.map(cb))`), the
            // host annotation names the bound expression to wrap and
            // label.
            if let Some(host) = host {
                if matches!(host.kind, SerializedCallbackHostKind::VariableDeclarator) {
                    if let Some(result_var) = result_var_for_host(host, parent_scope, ctx) {
                        let wrapper_idx = ensure_host_call_proxy(
                            arena,
                            state,
                            ctx,
                            container,
                            &mut call_proxy_by_host,
                            host,
                            &result_var,
                        );
                        let target = callback_chain_target(
                            arena,
                            state,
                            ctx,
                            container,
                            ChainHost {
                                proxy: wrapper_idx,
                                head: &host.head,
                            },
                            (child.block.span.offset.0, child.block.end_span.offset.0),
                            &mut nested_call_proxy,
                        );
                        // Route the chain's input (the innermost receiver)
                        // to the innermost proxy so the backbone runs
                        // `arr -> arr.map() -> ... -> xs`.
                        if let Some(inner) =
                            innermost_chain_proxy_id(arena, &host.head, &nested_call_proxy)
                        {
                            state.result_proxy_by_var.insert(result_var.clone(), inner);
                            // A ternary binds `xs` from two arms; gate the
                            // proxy redirect to the arm that hosts the call
                            // so the sibling arm's value reaches `xs`
                            // directly instead of through the call.
                            if let Some(span) = conditional_arm_span(parent_scope) {
                                state.result_proxy_arm_span.insert(result_var.clone(), span);
                            }
                        }
                        build_scope(arena, state, ctx, child, Container::Subgraph(target));
                        i += 1;
                        continue;
                    }
                }
                if matches!(host.kind, SerializedCallbackHostKind::Return) {
                    let wrapper_idx = ensure_return_call_proxy(
                        arena,
                        state,
                        ctx,
                        container,
                        &mut call_proxy_by_host,
                        host,
                    );
                    let target = callback_chain_target(
                        arena,
                        state,
                        ctx,
                        container,
                        ChainHost {
                            proxy: wrapper_idx,
                            head: &host.head,
                        },
                        (child.block.span.offset.0, child.block.end_span.offset.0),
                        &mut nested_call_proxy,
                    );
                    // Route the chain's input (the innermost receiver) to
                    // the innermost proxy so the returned call's backbone
                    // runs `arr -> arr.map() -> ...` into the return proxy.
                    if let Some(inner) =
                        innermost_chain_proxy_id(arena, &host.head, &nested_call_proxy)
                    {
                        let container_key =
                            format!("{}-{}", host.start_span.offset.0, host.end_span.offset.0);
                        // A returned ternary shares one completion span
                        // across both arms; gate the proxy to the arm
                        // hosting the call so the sibling arm's value gets
                        // its own return-use node.
                        if let Some(span) = conditional_arm_span(parent_scope) {
                            state
                                .return_proxy_arm_span
                                .insert(container_key.clone(), span);
                        }
                        state.return_proxy_by_span.insert(container_key, inner);
                    }
                    build_scope(arena, state, ctx, child, Container::Subgraph(target));
                    i += 1;
                    continue;
                }
            }
            // No statement and no result-bound declarator host (bare
            // `return`, assignment, computed receiver, ...): the
            // callback gets no CallProxy wrapper -- only the
            // `<callee>(args[N])` label from `describe_subgraph`. Fall
            // through to default handling.
        }
        let ckey = branch_container_key(child);
        let Some(key) = ckey.as_deref() else {
            build_scope(arena, state, ctx, child, container);
            i += 1;
            continue;
        };
        let is_ternary = key.starts_with("ternary:");
        if !key.starts_with("if:") && !is_ternary {
            build_scope(arena, state, ctx, child, container);
            i += 1;
            continue;
        }
        let mut group: Vec<&SerializedScope> = vec![child];
        let mut j = i + 1;
        while j < children.len() {
            let next = children[j];
            if branch_container_key(next).as_deref() != Some(key) {
                break;
            }
            group.push(next);
            j += 1;
        }
        if is_ternary {
            build_conditional_group(arena, state, ctx, container, &group);
            i = j;
            continue;
        }
        if group.len() < 2 {
            let lone = group[0];
            let lone_offset = lone.block_context.as_ref().map(|c| c.parent_span_offset());
            build_scope(arena, state, ctx, lone, container);
            if let Some(offset) = lone_offset {
                attach_test_anchor_to_consequent(arena, state, lone, offset, &ctx.source_index);
            }
            i = j;
            continue;
        }

        let offset = child
            .block_context
            .as_ref()
            .map(|c| c.parent_span_offset())
            .unwrap_or(Utf16CodeUnitOffset(0));
        let parent_id = child.upper.as_ref().map(|s| s.value()).unwrap_or("");
        let container_id = if_container_subgraph_id(parent_id, offset.0);
        let has_else = group
            .iter()
            .any(|v| v.block_context.as_ref().map(|c| c.key()) == Some("alternate"));
        let descriptor = OwnedVisualSubgraph::if_else_container(
            container_id,
            line_for_offset(&ctx.source_index, offset),
            has_else,
            Vec::new(),
            Direction::RL,
        )
        .into();
        let container_idx: SubgraphIdx = arena.push_subgraph(descriptor);
        arena.append_child(container, ElementHandle::Subgraph(container_idx));

        // Build each branch first so the consequent subgraphs exist
        // before we attach test anchors to them.
        for g in &group {
            build_scope(arena, state, ctx, g, Container::Subgraph(container_idx));
        }

        // Attach each IfStatement's test anchor to the consequent it
        // gates. Distinct `parentSpanOffset` values within the group
        // correspond to distinct IfStatement nodes.
        let mut seen_offsets: std::collections::HashSet<Utf16CodeUnitOffset> =
            std::collections::HashSet::new();
        for g in &group {
            let off = g.block_context.as_ref().map(|c| c.parent_span_offset());
            let key = g.block_context.as_ref().map(|c| c.key());
            let Some(off) = off else { continue };
            if seen_offsets.contains(&off) {
                continue;
            }
            if key != Some("consequent") {
                continue;
            }
            seen_offsets.insert(off);
            attach_test_anchor_to_consequent(arena, state, g, off, &ctx.source_index);
        }

        let container_line = arena.subgraph(container_idx).descriptor.line();
        let child_handles: Vec<ElementHandle> = arena.subgraph(container_idx).children.clone();
        let mut container_end_line = container_line;
        for handle in child_handles {
            if let ElementHandle::Subgraph(child_sg) = handle {
                if let Some(end) = arena.subgraph(child_sg).descriptor.end_line() {
                    container_end_line = container_end_line.max(end);
                }
            }
        }
        if container_end_line != container_line {
            arena
                .subgraph_mut(container_idx)
                .descriptor
                .set_end_line(Some(container_end_line));
        }
        i = j;
    }
}

#[cfg(test)]
#[path = "build_children_test.rs"]
mod build_children_test;
