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

use unsnarl_ir::primitive::{SourceIndex, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::{
    SerializedCallbackHost, SerializedCallbackHostKind, SerializedDefinition,
    SerializedExpressionStatementContainer, SerializedHeadExpression, SerializedScope,
};

use crate::direction::Direction;
use crate::visual_node::{SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::branch_container_key::branch_container_key;
use super::build_scope::build_scope;
use super::context::BuilderContext;
use super::expression_statement_node_id::expression_statement_node_id;
use super::if_container_subgraph_id::if_container_subgraph_id;
use super::if_test_node_id::if_test_node_id;
use super::is_collapsed::is_collapsed;
use super::line_for_offset::line_for_offset;
use super::node_id::node_id;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;
use super::write_op_node_id::write_op_node_id;

fn make_if_test_anchor(
    id: String,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) -> VisualNode {
    SyntheticVisualNode::if_statement_test(id, line_for_offset(source_index, offset)).into()
}

/// Look up (or create on first sight) the `CallProxy` wrapper
/// subgraph for `statement`. Taking the
/// [`SerializedExpressionStatementContainer`] by reference -- the one
/// [`ExpressionStatementIndex::enclosing`] just returned -- means the
/// wrapper's id, `callName`, and span lines come straight off a known
/// statement; there is no offset to look back up and no miss to guard.
///
/// First-sight allocation appends the wrapper to `container` at
/// the call site, so the wrapper lands in the same source-order
/// position as the first callback child belonging to that
/// statement. Subsequent callbacks for the same statement reuse
/// the cached subgraph index and only emit themselves as wrapper
/// children.
fn ensure_call_proxy_wrapper(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_stmt_offset: &mut HashMap<u32, SubgraphIdx>,
    statement: &SerializedExpressionStatementContainer,
) -> SubgraphIdx {
    let stmt_offset = statement.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_stmt_offset.get(&stmt_offset) {
        return idx;
    }
    let id = expression_statement_node_id(stmt_offset);
    let name = render_head_expression(&statement.head, &ctx.source_index);
    let start_line = statement.start_span.line.0;
    let end_line = if statement.end_span.line.0 != start_line {
        Some(statement.end_span.line.0)
    } else {
        None
    };
    let mut sg =
        OwnedVisualSubgraph::call_proxy(id.clone(), start_line, name, Vec::new(), Direction::RL);
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    // Wire the offset cache so the downstream
    // `ensure_expression_statement_node` call (during reference
    // traversal) returns this wrapper subgraph's id instead of
    // emitting a separate leaf `expr_stmt_<offset>` node.
    state.expression_statement_by_offset.insert(stmt_offset, id);
    call_proxy_by_stmt_offset.insert(stmt_offset, idx);
    idx
}

/// The variable a `VariableDeclarator`-host's call result is bound to:
/// the variable in `scope` whose declarator init starts where the
/// host's bound expression starts. They are the same AST node, so the
/// start offsets match exactly.
fn result_var_for_host(
    host: &SerializedCallbackHost,
    scope: &SerializedScope,
    ctx: &BuilderContext<'_>,
) -> Option<String> {
    let init_start = host.start_span.offset.0;
    scope.variables.iter().find_map(|vid| {
        let v = ctx.variable_map.get(vid.value())?;
        match v.defs.first()? {
            SerializedDefinition::Variable(d) => {
                (d.init()?.span.offset.0 == init_start).then(|| v.id.value().to_string())
            }
            _ => None,
        }
    })
}

/// Look up (or create on first sight) the `CallProxy` wrapper for a
/// callback whose enclosing call's result is bound to `result_var`
/// (`const xs = arr.map(cb)`, including a call nested in arguments
/// `const x = foo(data.map(cb))`) -- the non-statement counterpart of
/// [`ensure_call_proxy_wrapper`].
///
/// The proxy spans the host's bound expression and is labelled with its
/// head, so it represents the whole bound call (`foo(...)`), not just
/// the inner callback's call. It carries the result variable's node id
/// as `owner_node_id`, so the emitter bundles that variable beside the
/// proxy in a `wrap_` box -- the call ↔ variable relationship shown by
/// containment, not an edge. Every callback within the same bound
/// expression shares one proxy, keyed by the host's start offset.
fn ensure_host_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
    result_var: &str,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    // Inputs of this call own the result variable; their init-time
    // owner edges retarget from the result-variable node to this proxy
    // (see `result_proxy_by_var`).
    state
        .result_proxy_by_var
        .insert(result_var.to_string(), id.clone());
    let name = render_head_expression(&host.head, &ctx.source_index);
    let start_line = host.start_span.line.0;
    let end_line = if host.end_span.line.0 != start_line {
        Some(host.end_span.line.0)
    } else {
        None
    };
    let mut sg = OwnedVisualSubgraph::call_proxy_owned(
        id,
        start_line,
        name,
        node_id(result_var),
        Vec::new(),
        Direction::RL,
    );
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    call_proxy_by_host.insert(key, idx);
    idx
}

/// Look up (or create on first sight) the `CallProxy` wrapper for a
/// callback returned from a function (`return arr.map(cb)`).
///
/// Unlike the variable-declarator case there is no result variable to
/// bundle with, so the proxy carries no owner. It spans the returned
/// expression and contains the callback; the return-completion inputs
/// (the call's receiver / callee / args) route to it via
/// `return_proxy_by_span` instead of a return-use node, so the returned
/// call's callback is contained rather than left as an island.
fn ensure_return_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    let container_key = format!("{}-{}", host.start_span.offset.0, host.end_span.offset.0);
    state.return_proxy_by_span.insert(container_key, id.clone());
    let name = render_head_expression(&host.head, &ctx.source_index);
    let start_line = host.start_span.line.0;
    let end_line = if host.end_span.line.0 != start_line {
        Some(host.end_span.line.0)
    } else {
        None
    };
    let mut sg = OwnedVisualSubgraph::call_proxy(id, start_line, name, Vec::new(), Direction::RL);
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    call_proxy_by_host.insert(key, idx);
    idx
}

/// What a reassignment `y = arr.map(cb)` binds its CallProxy to: the
/// write-op node the proxy is bundled with (owner) and, when the
/// reassignment is itself an ExpressionStatement, that statement's
/// offset (so the proxy claims it).
struct ReassignmentBinding {
    write_op_node: String,
    stmt_offset: Option<u32>,
}

/// The write-op node id of a reassignment host's target, when the
/// left-hand side is a plain identifier. The host carries the target
/// identifier's offset (`target_span`); the matching Write `WriteOp`
/// shares that offset (both are the write reference's identifier
/// offset), so its `ref_id` yields the `wr_<ref_id>` node the proxy is
/// bundled with. `None` for member / destructuring targets (no
/// `target_span`) or if no write op is found at that offset.
fn write_op_node_for_assignment(
    host: &SerializedCallbackHost,
    ctx: &BuilderContext<'_>,
) -> Option<String> {
    let target_offset = host.target_span.as_ref()?.offset.0;
    ctx.write_op_by_ref
        .values()
        .find(|op| op.offset == target_offset)
        .map(|op| write_op_node_id(&op.ref_id))
}

/// Look up (or create on first sight) the `CallProxy` wrapper for a
/// callback whose enclosing call's result is reassigned to an existing
/// variable (`y = arr.map(cb)`) -- the reassignment counterpart of
/// [`ensure_host_call_proxy`].
///
/// Unlike the declarator case the result variable's own node lives at
/// its declaration site, elsewhere in the graph; bundling it here would
/// pull it out of position. The reassignment's *write-op* node lives at
/// the assignment site instead, so the proxy carries that node as its
/// owner: the emitter bundles the write-op node beside the proxy in a
/// `wrap_` box, and the call's inputs retarget from the write-op node to
/// the proxy border (see `result_proxy_by_write_op`). The result is
/// `wrap[write | call_proxy "arr.map()"]`, parallel to the declarator's
/// `wrap[var | call_proxy]`.
fn ensure_assignment_call_proxy(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_host: &mut HashMap<u32, SubgraphIdx>,
    host: &SerializedCallbackHost,
    binding: &ReassignmentBinding,
) -> SubgraphIdx {
    let key = host.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_host.get(&key) {
        return idx;
    }
    let id = format!("call_proxy_{key}");
    state
        .result_proxy_by_write_op
        .insert(binding.write_op_node.clone(), id.clone());
    // When the reassignment is itself an ExpressionStatement, claim that
    // statement's offset so the callback's statement-contained reads
    // (its return-completed body, owner-less) route into this proxy
    // rather than minting a separate `expr_stmt_<offset>` node -- the
    // same claim `ensure_call_proxy_wrapper` makes for statement-hosted
    // callbacks. A reassignment nested in a declarator (`const r = (y =
    // ...)`) has no enclosing statement, so this stays `None` and the
    // body keeps its own return-use node.
    if let Some(off) = binding.stmt_offset {
        state
            .expression_statement_by_offset
            .entry(off)
            .or_insert_with(|| id.clone());
    }
    let name = render_head_expression(&host.head, &ctx.source_index);
    let start_line = host.start_span.line.0;
    let end_line = if host.end_span.line.0 != start_line {
        Some(host.end_span.line.0)
    } else {
        None
    };
    let mut sg = OwnedVisualSubgraph::call_proxy_owned(
        id,
        start_line,
        name,
        binding.write_op_node.clone(),
        Vec::new(),
        Direction::RL,
    );
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    call_proxy_by_host.insert(key, idx);
    idx
}

/// The `Call` / `New` nodes of a head expression's receiver chain,
/// outermost first. `arr.map(f).filter(g)` parses as
/// `(((arr.map)(f)).filter)(g)`, so the head is the outer `filter`
/// `Call` whose callee's object is the inner `map` `Call`; descending
/// through each `Call`'s `Member` callee object yields
/// `[filter-call, map-call]`. Only the *receiver* chain is followed
/// (a `Member` object that is itself a `Call`); arguments are not part
/// of the head, so a call nested as an argument (`foo(items.map(cb))`)
/// contributes no inner node here and is left to its single host proxy.
fn receiver_call_chain(head: &SerializedHeadExpression) -> Vec<&SerializedHeadExpression> {
    let mut out = Vec::new();
    let mut node = head;
    while let SerializedHeadExpression::Call { callee, .. }
    | SerializedHeadExpression::New { callee, .. } = node
    {
        out.push(node);
        match callee.as_ref() {
            SerializedHeadExpression::Member { object, .. } => node = object,
            _ => break,
        }
    }
    out
}

/// UTF-16 start/end offsets and start/end lines of a `Call` / `New`
/// head node. `None` for any other kind.
fn call_node_extent(node: &SerializedHeadExpression) -> Option<(u32, u32, u32, Option<u32>)> {
    let (start_span, end_span) = match node {
        SerializedHeadExpression::Call {
            start_span,
            end_span,
            ..
        }
        | SerializedHeadExpression::New {
            start_span,
            end_span,
            ..
        } => (start_span, end_span),
        _ => return None,
    };
    let start_line = start_span.line.0;
    let end_line = (end_span.line.0 != start_line).then_some(end_span.line.0);
    Some((start_span.offset.0, end_span.offset.0, start_line, end_line))
}

/// The proxy a callback should be built into, given its host proxy
/// (the outermost, host-bound call). For a method chain
/// `arr.map(f).filter(g)` the callbacks `f` and `g` belong to *different*
/// calls; placing both in the single host proxy would merge two distinct
/// block scopes. Walking the host head's receiver chain, each callback is
/// routed into the innermost call whose span contains its block, with a
/// nested `CallProxy` created per inner call -- so `filter`'s callback
/// stays in the host proxy while `map`'s callback descends into a nested
/// `arr.map()` proxy. A single (non-chained) call has only the outermost
/// node, so its callback stays in the host proxy unchanged.
fn callback_chain_target(
    arena: &mut BuildArena,
    ctx: &BuilderContext<'_>,
    host_proxy: SubgraphIdx,
    host_head: &SerializedHeadExpression,
    block_start: u32,
    block_end: u32,
    nested_call_proxy: &mut HashMap<(u32, u32), SubgraphIdx>,
) -> SubgraphIdx {
    let mut parent = host_proxy;
    let mut target = host_proxy;
    for (depth, &call_node) in receiver_call_chain(host_head).iter().enumerate() {
        let Some((start_offset, end_offset, start_line, end_line)) = call_node_extent(call_node)
        else {
            break;
        };
        // Calls are listed outermost first; once one no longer contains
        // the callback block, no deeper (narrower) call can either.
        if !(start_offset <= block_start && block_end <= end_offset) {
            break;
        }
        if depth == 0 {
            // The outermost call is the host's bound expression itself,
            // already materialised as the host proxy.
            nested_call_proxy
                .entry((start_offset, end_offset))
                .or_insert(host_proxy);
        } else if let Some(&idx) = nested_call_proxy.get(&(start_offset, end_offset)) {
            target = idx;
        } else {
            // One inner call of the chain gets its own nested CallProxy,
            // labelled with the call head itself (`arr.map()`), distinct
            // from the host proxy's whole-chain label
            // (`arr.map().filter()`). Keyed by call span so every callback
            // of that call shares it.
            let id = format!("call_proxy_{start_offset}_{end_offset}");
            let name = render_head_expression(call_node, &ctx.source_index);
            let mut sg =
                OwnedVisualSubgraph::call_proxy(id, start_line, name, Vec::new(), Direction::RL);
            sg.end_line = end_line;
            let idx = arena.push_subgraph(sg.into());
            arena.append_child(Container::Subgraph(parent), ElementHandle::Subgraph(idx));
            nested_call_proxy.insert((start_offset, end_offset), idx);
            target = idx;
        }
        parent = target;
    }
    target
}

/// Place the IfStatement's test anchor at the head of the consequent
/// subgraph it gates. `else` is the fallback path and carries no
/// test. A consequent that collapsed past the depth threshold builds
/// no subgraph (`should_subgraph` is always true for a non-collapsed
/// if-branch Block scope, so `subgraph_by_scope` misses only when the
/// scope collapsed); with nowhere to host the anchor it is dropped
/// rather than leaking into the surrounding container.
fn attach_test_anchor_to_consequent(
    arena: &mut BuildArena,
    state: &mut BuildState,
    consequent: &SerializedScope,
    offset: Utf16CodeUnitOffset,
    source_index: &SourceIndex<'_>,
) {
    if state.if_test_anchor_by_offset.contains_key(&offset.0) {
        return;
    }
    let Some(body_sg) = state.subgraph_by_scope.get(consequent.id.value()).copied() else {
        return;
    };
    let parent_id = consequent.upper.as_ref().map(|s| s.value()).unwrap_or("");
    let id = if_test_node_id(parent_id, offset.0);
    let node = make_if_test_anchor(id.clone(), offset, source_index);
    let idx = arena.push_node(node);
    arena.prepend_child(Container::Subgraph(body_sg), ElementHandle::Node(idx));
    state.if_test_anchor_by_offset.insert(offset.0, id);
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
        // Callback-arg children route into a `CallProxy` wrapper.
        // The wrapper is created on first sight (so it lands at the
        // first callback's source position) and reused for any
        // later siblings that share the same statement offset.
        //
        // A callback collapsed past the depth ceiling builds no
        // subgraph, so wrapping it would leave an empty CallProxy husk
        // with its inputs dangling on it. Skip the wrapper and let the
        // default path record the collapse (a BeyondDepth stub in this
        // container) instead.
        if child.callback_argument.is_some() && !is_collapsed(child, ctx.depths.as_ref()) {
            let host = child
                .callback_argument
                .as_ref()
                .and_then(|cb| cb.host.as_ref());
            // A reassignment host (`y = arr.map(cb)`) takes priority over
            // the enclosing-statement path below. The statement
            // `y = arr.map(cb);` is itself an ExpressionStatement, so the
            // statement path would label the proxy `y = arr.map()` and
            // strand the receiver on the write-op node. Wrapping by the
            // assignment instead -- a CallProxy owned by the write-op node
            // -- renders reassignment parallel to declaration
            // (`const xs = arr.map(cb)`): wrap[write | call_proxy]. Only a
            // plain-identifier target resolves to a single write-op node;
            // member / destructuring targets carry no `target_span` and
            // fall through to the statement / island paths.
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
                            ctx,
                            wrapper_idx,
                            &host.head,
                            child.block.span.offset.0,
                            child.block.end_span.offset.0,
                            &mut nested_call_proxy,
                        );
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
                build_scope(arena, state, ctx, child, Container::Subgraph(wrapper_idx));
                i += 1;
                continue;
            }
            // Not a statement-level call. If the call's result is bound
            // to a variable (`const xs = arr.map(cb)`, including a call
            // nested in arguments `const x = foo(data.map(cb))`), the
            // host annotation gives the binding's bound expression --
            // its span and label. Wrap the callback in a CallProxy
            // spanning that whole bound expression, bundled with the
            // result variable by containment, the same shape as the
            // statement path.
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
                            ctx,
                            wrapper_idx,
                            &host.head,
                            child.block.span.offset.0,
                            child.block.end_span.offset.0,
                            &mut nested_call_proxy,
                        );
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
                        ctx,
                        wrapper_idx,
                        &host.head,
                        child.block.span.offset.0,
                        child.block.end_span.offset.0,
                        &mut nested_call_proxy,
                    );
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
        if !key.starts_with("if:") {
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
