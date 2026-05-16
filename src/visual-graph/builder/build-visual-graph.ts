import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import type { VisualEdge } from "../visual-edge.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualGraph } from "../visual-graph.js";
import { branchContainerKey } from "./branch-container-key.js";
import { buildScope } from "./build-scope.js";
import type { BuildState, PendingLoopTestAnchor } from "./build-state.js";
import type { BuilderContext, BuildVisualGraphOptions } from "./context.js";
import { edgeLabelOfRef } from "./edge-label-of-ref.js";
import { enclosingFunctionVar } from "./enclosing-function-var.js";
import { ensureExpressionStatementNode } from "./ensure-expression-statement-node.js";
import { expressionStatementNodeId } from "./expression-statement-node-id.js";
import { findHostSubgraph } from "./find-host-subgraph.js";
import { findNodeById } from "./find-node-by-id.js";
import { intermediateKey } from "./intermediate-key.js";
import { isAncestorScope } from "./is-ancestor-scope.js";
import { lastWriteOpInScopeBefore } from "./last-write-op-in-scope-before.js";
import { MODULE_ROOT_ID } from "./module-root-id.js";
import { nodeId } from "./node-id.js";
import { ownerTargetId } from "./owner-target-id.js";
import { predicateTargetId } from "./predicate-target-id.js";
import { previousFallthroughCase } from "./previous-fallthrough-case.js";
import { pushEdge } from "./push-edge.js";
import { readOrigins } from "./read-origins.js";
import { resolveReadTargetId } from "./resolve-read-target-id.js";
import { retUseNodeId } from "./ret-use-node-id.js";
import { sanitize } from "./sanitize.js";
import { setPredecessorOf } from "./set-predecessor-of.js";
import { stateRefId } from "./state-ref-id.js";
import { throwUseNodeId } from "./throw-use-node-id.js";
import { writeOpNodeId } from "./write-op-node-id.js";
import type { WriteOp } from "./write-op.js";

export function buildVisualGraph(
  ir: SerializedIR,
  opts?: BuildVisualGraphOptions,
): VisualGraph {
  const graph = {
    version: SERIALIZED_IR_VERSION,
    source: { path: ir.source.path, language: ir.source.language },
    direction: DIRECTION.RL,
    elements: [] as VisualElement[],
    edges: [] as VisualEdge[],
    boundaryEdges: [],
    pruning: null,
  } satisfies VisualGraph;

  const variableMap = new Map<string, SerializedVariable>();
  for (const v of ir.variables) {
    variableMap.set(v.id, v);
  }
  const scopeMap = new Map<string, SerializedScope>();
  for (const s of ir.scopes) {
    scopeMap.set(s.id, s);
  }

  // var declarations remain visible as nodes (via scope.variables) but their
  // references are excluded from edge / WriteOp emission below.
  const varVarIds = new Set<string>();
  for (const v of ir.variables) {
    const def = v.defs[0];
    if (
      def?.type === DEFINITION_TYPE.Variable &&
      def.declarationKind === VARIABLE_DECLARATION_KIND.Var
    ) {
      varVarIds.add(v.id);
    }
  }

  const subgraphOwnerVar = new Map<string, string>();
  for (const variable of ir.variables) {
    const def = variable.defs[0];
    if (!def) {
      continue;
    }
    let blockOffset: number | null = null;
    if (def.type === DEFINITION_TYPE.FunctionName) {
      blockOffset = def.node.span.offset;
    } else if (
      def.type === DEFINITION_TYPE.Variable &&
      def.init !== null &&
      (def.init.type === AST_TYPE.FunctionExpression ||
        def.init.type === AST_TYPE.ArrowFunctionExpression)
    ) {
      blockOffset = def.init.span.offset;
    }
    if (blockOffset === null) {
      continue;
    }
    const fnScope = ir.scopes.find(
      (v) => v.upper === variable.scope && v.block.span.offset === blockOffset,
    );
    if (fnScope) {
      subgraphOwnerVar.set(fnScope.id, variable.id);
    }
  }

  const refsByVariable = new Map<string, /* mutable */ SerializedReference[]>();
  for (const r of ir.references) {
    if (!r.resolved) {
      continue;
    }
    if (varVarIds.has(r.resolved)) {
      continue;
    }
    const arr = refsByVariable.get(r.resolved) ?? [];
    arr.push(r);
    refsByVariable.set(r.resolved, arr);
  }
  for (const [, refs] of refsByVariable) {
    refs.sort((a, b) => a.identifier.span.offset - b.identifier.span.offset);
  }
  const writeOpsByVariable = new Map<string, /* mutable */ WriteOp[]>();
  const writeOpsByScope = new Map<string, /* mutable */ WriteOp[]>();
  const writeOpByRef = new Map<string, WriteOp>();
  for (const v of ir.variables) {
    const refs = refsByVariable.get(v.id) ?? [];
    const ops: /* mutable */ WriteOp[] = [];
    for (const r of refs) {
      if (!r.flags.write) {
        continue;
      }
      if (r.init) {
        // init Write reference is the binding's initial PutValue; the Variable
        // node already represents the declaration, so emitting a WriteOp here
        // would double-count the initialization as an explicit assignment.
        continue;
      }
      const op = {
        refId: r.id,
        varId: v.id,
        varName: v.name,
        line: r.identifier.span.line,
        offset: r.identifier.span.offset,
        scopeId: r.from,
      } as const satisfies WriteOp;
      ops.push(op);
      writeOpByRef.set(r.id, op);
      const sopArr = writeOpsByScope.get(op.scopeId) ?? [];
      sopArr.push(op);
      writeOpsByScope.set(op.scopeId, sopArr);
    }
    if (ops.length > 0) {
      writeOpsByVariable.set(v.id, ops);
    }
  }

  const sortedCasesByContainer = new Map<
    string,
    /* mutable */ SerializedScope[]
  >();
  for (const s of ir.scopes) {
    const ckey = branchContainerKey(s);
    if (ckey?.startsWith("switch:")) {
      const arr = sortedCasesByContainer.get(ckey) ?? [];
      arr.push(s);
      sortedCasesByContainer.set(ckey, arr);
    }
  }
  for (const [, arr] of sortedCasesByContainer) {
    arr.sort((a, b) => a.block.span.offset - b.block.span.offset);
  }

  const ctx = {
    ir,
    variableMap,
    scopeMap,
    subgraphOwnerVar,
    writeOpsByVariable,
    writeOpsByScope,
    writeOpByRef,
    sortedCasesByContainer,
    ...(opts?.depths ? { depths: opts.depths } : {}),
  } as const satisfies BuilderContext;
  const state = {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    throwSubgraphsByFn: new Map(),
    throwUseAdded: new Set(),
    ifTestAnchorByOffset: new Map(),
    switchDiscriminantAnchorByOffset: new Map(),
    whileTestAnchorByOffset: new Map(),
    doWhileTestAnchorByOffset: new Map(),
    forTestAnchorByOffset: new Map(),
    pendingLoopTestAnchors: [] as PendingLoopTestAnchor[],
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: graph.edges,
    collapsedRootByScope: new Map(),
    collapsedAnchorByRoot: new Map(),
    suppressedPredicateRedirect: new Map(),
    beyondDepthStubByParent: new Map(),
    nodeIdOriginScope: new Map(),
  } as const satisfies BuildState;

  const root = ir.scopes.find(
    (v) => v.type === SCOPE_TYPE.Module || v.type === SCOPE_TYPE.Global,
  );
  if (root) {
    buildScope(root, graph, ctx, state);
  }

  // let-chain edges (set / fallthrough)
  for (const ops of writeOpsByVariable.values()) {
    const head = ops[0];
    if (!head) {
      continue;
    }
    for (let i = 0; i < ops.length; i++) {
      const op = ops[i];
      if (!op) {
        continue;
      }
      let prevId = nodeId(op.varId);
      const opScope = scopeMap.get(op.scopeId) ?? null;
      const opBranchKey = opScope ? branchContainerKey(opScope) : null;
      const isFirstInCase =
        opScope !== null &&
        opBranchKey !== null &&
        opBranchKey.startsWith("switch:") &&
        !ops.slice(0, i).some((prevOp) => prevOp.scopeId === op.scopeId);
      if (isFirstInCase && opScope) {
        const prevCase = previousFallthroughCase(
          opScope,
          sortedCasesByContainer,
        );
        if (prevCase) {
          const prevCaseLast = lastWriteOpInScopeBefore(
            op.varId,
            prevCase.id,
            op.offset,
            writeOpsByVariable,
            scopeMap,
          );
          if (prevCaseLast) {
            prevId = writeOpNodeId(prevCaseLast.refId);
          }
        }
      } else {
        for (let j = i - 1; j >= 0; j--) {
          const candidate = ops[j];
          if (!candidate) {
            continue;
          }
          if (isAncestorScope(candidate.scopeId, op.scopeId, scopeMap)) {
            prevId = writeOpNodeId(candidate.refId);
            break;
          }
        }
      }
      const edgeKind =
        opScope &&
        isFirstInCase &&
        previousFallthroughCase(opScope, sortedCasesByContainer)
          ? "fallthrough"
          : "set";
      pushEdge(state, prevId, edgeKind, writeOpNodeId(op.refId));
    }
  }

  for (const r of ir.references) {
    if (!r.resolved) {
      continue;
    }
    if (varVarIds.has(r.resolved)) {
      continue;
    }
    // When the ref's containing scope (or any ancestor) was collapsed,
    // the inner ret-use / expr-stmt / write-op nodes were never built
    // -- they would have lived inside the now-hidden subtree. Instead
    // of creating fragments of the hidden body in unrelated outer
    // subgraphs, route the ref's read directly to the collapsed
    // anchor (variable owner if any, otherwise the closest visible
    // ancestor subgraph). The user still sees "this outer variable is
    // consumed somewhere inside the surviving outer container", just
    // without a separate node per inner reference.
    const collapsedRoot = state.collapsedRootByScope?.get(r.from);
    if (collapsedRoot !== undefined) {
      if (r.flags.write) {
        continue;
      }
      const target = collapsedTargetFor(collapsedRoot, state);
      if (target === null) {
        continue;
      }
      const fromIds = readOrigins(
        r.resolved,
        r.identifier.span.offset,
        r.from,
        ctx,
      );
      for (const fromId of fromIds) {
        pushEdge(state, fromId, edgeLabelOfRef(r), target);
      }
      continue;
    }
    const predicateTarget = predicateTargetId(r, scopeMap, state);
    if (predicateTarget && !r.flags.write) {
      const fromIds = readOrigins(
        r.resolved,
        r.identifier.span.offset,
        r.from,
        ctx,
      );
      for (const fromId of fromIds) {
        pushEdge(state, fromId, edgeLabelOfRef(r), predicateTarget);
      }
      continue;
    }
    // The ref reads a predicate (if/while/for/switch test) whose anchor
    // was suppressed because the gated scope collapsed. The redirect
    // map points to the closest visible ancestor subgraph of that
    // collapsed body, so the read still lands somewhere meaningful
    // instead of dangling off into module_root.
    if (predicateTarget === null && r.predicateContainer && !r.flags.write) {
      const redirect = state.suppressedPredicateRedirect?.get(
        r.predicateContainer.offset,
      );
      if (redirect !== undefined) {
        const fromIds = readOrigins(
          r.resolved,
          r.identifier.span.offset,
          r.from,
          ctx,
        );
        for (const fromId of fromIds) {
          pushEdge(state, fromId, edgeLabelOfRef(r), redirect);
        }
      }
      continue;
    }
    if (r.flags.write) {
      if (r.flags.call || (r.flags.read && r.owners.length > 0)) {
        const fromId = stateRefId(r.id, r.resolved, ctx);
        for (const ownerId of r.owners) {
          if (ownerId === r.resolved) {
            continue;
          }
          const targetId = ownerTargetId(
            ownerId,
            r.identifier.span.offset,
            writeOpsByVariable,
          );
          pushEdge(state, fromId, edgeLabelOfRef(r), targetId);
        }
      }
      if (r.flags.read) {
        const op = writeOpByRef.get(r.id);
        if (op) {
          const wrTargetId = writeOpNodeId(r.id);
          const setPredId = setPredecessorOf(
            op,
            writeOpsByVariable.get(r.resolved) ?? [],
            scopeMap,
          );
          const fromIds = readOrigins(
            r.resolved,
            r.identifier.span.offset,
            r.from,
            ctx,
          );
          for (const fromId of fromIds) {
            if (fromId === setPredId || fromId === wrTargetId) {
              continue;
            }
            pushEdge(state, fromId, "read", wrTargetId);
          }
        }
      }
      continue;
    }
    const label = edgeLabelOfRef(r);
    const fromIds = readOrigins(
      r.resolved,
      r.identifier.span.offset,
      r.from,
      ctx,
    );
    if (r.owners.length > 0) {
      for (const ownerId of r.owners) {
        if (ownerId === r.resolved) {
          continue;
        }
        const targetId = ownerTargetId(
          ownerId,
          r.identifier.span.offset,
          writeOpsByVariable,
        );
        for (const fromId of fromIds) {
          pushEdge(state, fromId, label, targetId);
        }
      }
    } else {
      const enclosingFn = enclosingFunctionVar(
        r.from,
        scopeMap,
        subgraphOwnerVar,
      );
      const host = findHostSubgraph(r, enclosingFn, scopeMap, state);
      const targetElements = host?.elements ?? graph.elements;
      const exprStmtId = ensureExpressionStatementNode(
        r,
        ir.raw,
        targetElements,
        state,
      );
      const targetId = resolveReadTargetId(
        exprStmtId,
        enclosingFn,
        r,
        ctx,
        state,
      );
      for (const fromId of fromIds) {
        pushEdge(state, fromId, label, targetId);
      }
    }
  }

  const needsModuleRoot = state.edges.some(
    (edge) => edge.to === MODULE_ROOT_ID,
  );
  if (needsModuleRoot) {
    graph.elements.push({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: MODULE_ROOT_ID,
      kind: NODE_KIND.SyntheticModuleSink,
      name: "module",
      line: 0,
      endLine: null,
      isJsxElement: false,
      unused: false,
    });
  }

  // module sources, intermediates, and import edges
  interface ModuleNode {
    id: string;
    line: number;
    source: string;
  }
  interface Intermediate {
    id: string;
    name: string;
    line: number;
  }
  const moduleNodes = new Map<string, ModuleNode>();
  const intermediates = new Map<string, Intermediate>();

  for (const v of ir.variables) {
    const def = v.defs[0];
    if (def?.type !== DEFINITION_TYPE.ImportBinding) {
      continue;
    }
    const source = def.importSource;
    if (!source) {
      continue;
    }
    if (!moduleNodes.has(source)) {
      moduleNodes.set(source, {
        id: `mod_${sanitize(source)}`,
        line: def.parent?.span.line ?? 0,
        source,
      });
    }
    if (
      def.importKind === IMPORT_KIND.Named &&
      def.importedName !== null &&
      def.importedName !== v.name
    ) {
      const key = intermediateKey(source, def.importedName);
      if (!intermediates.has(key)) {
        intermediates.set(key, {
          id: `import_${sanitize(key)}`,
          name: def.importedName,
          line: def.node.span.line,
        });
      }
    }
  }

  for (const mod of moduleNodes.values()) {
    graph.elements.push({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: mod.id,
      kind: NODE_KIND.SyntheticModuleSource,
      name: mod.source,
      line: mod.line,
      endLine: null,
      isJsxElement: false,
      unused: false,
    });
  }
  for (const inter of intermediates.values()) {
    graph.elements.push({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: inter.id,
      kind: NODE_KIND.SyntheticImportIntermediate,
      name: inter.name,
      line: inter.line,
      endLine: null,
      isJsxElement: false,
      unused: false,
    });
  }
  for (const v of ir.variables) {
    const def = v.defs[0];
    if (def?.type !== DEFINITION_TYPE.ImportBinding) {
      continue;
    }
    const source = def.importSource;
    if (!source) {
      continue;
    }
    const mod = moduleNodes.get(source);
    if (!mod) {
      continue;
    }
    const localId = nodeId(v.id);
    const isRenamed =
      def.importKind === IMPORT_KIND.Named &&
      def.importedName !== null &&
      def.importedName !== v.name;
    if (isRenamed && def.importedName !== null) {
      const inter = intermediates.get(
        intermediateKey(source, def.importedName),
      );
      if (inter) {
        pushEdge(state, mod.id, "read", inter.id);
        pushEdge(state, inter.id, "read", localId);
        continue;
      }
    }
    pushEdge(state, mod.id, "read", localId);
  }

  for (const { subgraph, node, position } of state.pendingLoopTestAnchors) {
    if (position === "first") {
      subgraph.elements.unshift(node);
    } else {
      subgraph.elements.push(node);
    }
  }

  for (const id of ir.unusedVariableIds) {
    // var bindings have no edges in the visual graph, so surfacing their
    // unused state would imply a usage signal the rendering can otherwise
    // not show. Keep the IR-level fact (ir.unusedVariableIds) intact and
    // skip the visual mark.
    if (varVarIds.has(id)) {
      continue;
    }
    const target = nodeId(id);
    const node = findNodeById(graph.elements, target);
    if (node) {
      node.unused = true;
    }
  }

  // Edge redirection for collapsed scopes: an endpoint that originated
  // inside a collapsed subtree is rewritten to the variable anchor for
  // that subtree, falling back to the closest visible ancestor subgraph.
  // Edges between two endpoints that resolve to the same target collapse
  // into nothing (self-loops are dropped).
  if ((state.collapsedRootByScope?.size ?? 0) > 0) {
    redirectEdgesIntoCollapsed(graph.edges, ir, state);
  }

  return graph;
}

// Pick the visible target a collapsed subtree should be represented
// by: either the parent-scope variable that owns it (e.g. `fnB`) or a
// shared BeyondDepth stub placed inside the closest visible ancestor
// subgraph. The choice is made once during buildScope; this lookup is
// just the cache hit. Returns null when nothing upward was visible at
// build time (forces the caller to drop the edge).
function collapsedTargetFor(
  rootScopeId: string,
  state: BuildState,
): string | null {
  return state.collapsedAnchorByRoot?.get(rootScopeId) ?? null;
}

function redirectEdgesIntoCollapsed(
  edges: /* mutable */ VisualEdge[],
  ir: SerializedIR,
  state: BuildState,
): void {
  const collapsedRootByScope =
    state.collapsedRootByScope ?? new Map<string, string>();
  const originScopeByNodeId = new Map<string, string>(
    state.nodeIdOriginScope ?? new Map(),
  );
  // Variables: include every variable (even those whose nodes were never
  // emitted because they live inside a collapsed scope).
  for (const v of ir.variables) {
    const id = nodeId(v.id);
    if (!originScopeByNodeId.has(id)) {
      originScopeByNodeId.set(id, v.scope);
    }
  }
  // References whose nodes (write op / return-use / throw-use / expression
  // statement) were never created because their containing scope collapsed.
  for (const r of ir.references) {
    const wid = writeOpNodeId(r.id);
    if (!originScopeByNodeId.has(wid)) {
      originScopeByNodeId.set(wid, r.from);
    }
    const ruid = retUseNodeId(r.id);
    if (!originScopeByNodeId.has(ruid)) {
      originScopeByNodeId.set(ruid, r.from);
    }
    const tuid = throwUseNodeId(r.id);
    if (!originScopeByNodeId.has(tuid)) {
      originScopeByNodeId.set(tuid, r.from);
    }
    const c = r.expressionStatementContainer;
    if (c) {
      const sid = expressionStatementNodeId(c.startSpan.offset);
      if (!originScopeByNodeId.has(sid)) {
        originScopeByNodeId.set(sid, r.from);
      }
    }
  }

  // Returns a redirected node id, or null when the endpoint lives inside
  // a collapsed subtree with no surviving ancestor at all (drop signal).
  function redirect(id: string): string | null {
    const scope = originScopeByNodeId.get(id);
    if (scope === undefined) {
      return id;
    }
    const root = collapsedRootByScope.get(scope);
    if (root === undefined) {
      return id;
    }
    return collapsedTargetFor(root, state);
  }

  const redirected: VisualEdge[] = [];
  const seen = new Set<string>();
  for (const e of edges) {
    const from = redirect(e.from);
    const to = redirect(e.to);
    if (from === null || to === null) {
      continue;
    }
    if (from === to) {
      continue;
    }
    const key = `${from}\t${to}\t${e.label}`;
    if (seen.has(key)) {
      continue;
    }
    seen.add(key);
    redirected.push({ ...e, from, to });
  }
  edges.length = 0;
  edges.push(...redirected);
}
