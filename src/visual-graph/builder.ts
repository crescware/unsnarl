import { DIRECTION, NODE_KIND, VISUAL_ELEMENT_TYPE } from "../constants.js";
import type {
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
} from "../ir/model.js";
import { branchContainerKey } from "./builder/branch-container-key.js";
import { buildScope } from "./builder/build-scope.js";
import type { BuildState } from "./builder/build-state.js";
import type { BuilderContext } from "./builder/context.js";
import { edgeLabelOfRef } from "./builder/edge-label-of-ref.js";
import { enclosingFunctionVar } from "./builder/enclosing-function-var.js";
import { ensureReturnUseNode } from "./builder/ensure-return-use-node.js";
import { findNodeById } from "./builder/find-node-by-id.js";
import { intermediateKey } from "./builder/intermediate-key.js";
import { isAncestorScope } from "./builder/is-ancestor-scope.js";
import { lastWriteOpInScopeBefore } from "./builder/last-write-op-in-scope-before.js";
import { nodeId } from "./builder/node-id.js";
import { ownerTargetId } from "./builder/owner-target-id.js";
import { predicateTargetId } from "./builder/predicate-target-id.js";
import { previousFallthroughCase } from "./builder/previous-fallthrough-case.js";
import { pushEdge } from "./builder/push-edge.js";
import { readOrigins } from "./builder/read-origins.js";
import { sanitize } from "./builder/sanitize.js";
import { stateRefId } from "./builder/state-ref-id.js";
import { writeOpNodeId } from "./builder/write-op-node-id.js";
import type { WriteOp } from "./builder/write-op.js";
import type { VisualEdge, VisualElement, VisualGraph } from "./model.js";

const MODULE_ROOT_ID = "module_root";

export function buildVisualGraph(ir: SerializedIR): VisualGraph {
  const graph = {
    version: 1,
    source: { path: ir.source.path, language: ir.source.language },
    direction: DIRECTION.RL,
    elements: [] as VisualElement[],
    edges: [] as VisualEdge[],
  } satisfies VisualGraph;

  const variableMap = new Map<string, SerializedVariable>();
  for (const v of ir.variables) {
    variableMap.set(v.id, v);
  }
  const scopeMap = new Map<string, SerializedScope>();
  for (const s of ir.scopes) {
    scopeMap.set(s.id, s);
  }

  const subgraphOwnerVar = new Map<string, string>();
  for (const v of ir.variables) {
    const def = v.defs[0];
    if (!def) {
      continue;
    }
    let blockOffset: number | null = null;
    if (def.type === "FunctionName") {
      blockOffset = def.node.span.offset;
    } else if (
      def.type === "Variable" &&
      def.initSpan !== null &&
      (def.initType === "FunctionExpression" ||
        def.initType === "ArrowFunctionExpression")
    ) {
      blockOffset = def.initSpan.offset;
    }
    if (blockOffset === null) {
      continue;
    }
    const fnScope = ir.scopes.find(
      (s) => s.upper === v.scope && s.block.span.offset === blockOffset,
    );
    if (fnScope) {
      subgraphOwnerVar.set(fnScope.id, v.id);
    }
  }

  const hiddenVariables = new Set<string>();
  for (const v of ir.variables) {
    if (v.defs[0]?.type !== "ImplicitGlobalVariable") {
      continue;
    }
    const refs = ir.references.filter((r) => r.resolved === v.id);
    if (refs.length > 0 && refs.every((r) => r.flags.receiver)) {
      hiddenVariables.add(v.id);
    }
  }

  const refsByVariable = new Map<string, /* mutable */ SerializedReference[]>();
  for (const r of ir.references) {
    if (!r.resolved) {
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
    if (hiddenVariables.has(v.id)) {
      continue;
    }
    const refs = refsByVariable.get(v.id) ?? [];
    const ops: /* mutable */ WriteOp[] = [];
    for (const r of refs) {
      if (!r.flags.write) {
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
    hiddenVariables,
    writeOpsByVariable,
    writeOpsByScope,
    writeOpByRef,
    sortedCasesByContainer,
  } as const satisfies BuilderContext;
  const state = {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    emittedEdges: new Set(),
    edges: graph.edges,
  } as const satisfies BuildState;

  const root = ir.scopes.find(
    (s) => s.type === "module" || s.type === "global",
  );
  if (root) {
    buildScope(root, graph, ctx, state);
  }

  let needsModuleRoot = false;

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
      const opScope = scopeMap.get(op.scopeId);
      const opBranchKey = opScope ? branchContainerKey(opScope) : null;
      const isFirstInCase =
        opScope !== undefined &&
        opBranchKey !== null &&
        opBranchKey.startsWith("switch:") &&
        !ops
          .slice(0, i)
          .some(
            (prevOp) => prevOp !== undefined && prevOp.scopeId === op.scopeId,
          );
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
    if (hiddenVariables.has(r.resolved)) {
      continue;
    }
    const predicateTarget = predicateTargetId(r, ir.scopes, scopeMap);
    if (predicateTarget) {
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
      if (enclosingFn) {
        const useTargetId = ensureReturnUseNode(enclosingFn, r, ctx, state);
        if (useTargetId) {
          for (const fromId of fromIds) {
            pushEdge(state, fromId, label, useTargetId);
          }
        }
      } else {
        needsModuleRoot = true;
        for (const fromId of fromIds) {
          pushEdge(state, fromId, label, MODULE_ROOT_ID);
        }
      }
    }
  }

  if (needsModuleRoot) {
    graph.elements.push({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: MODULE_ROOT_ID,
      kind: NODE_KIND.ModuleSink,
      name: "module",
      line: 0,
      isJsxElement: false,
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
    if (hiddenVariables.has(v.id)) {
      continue;
    }
    const def = v.defs[0];
    if (def?.type !== "ImportBinding") {
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
      def.importKind === "named" &&
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
      kind: NODE_KIND.ModuleSource,
      name: mod.source,
      line: mod.line,
      isJsxElement: false,
    });
  }
  for (const inter of intermediates.values()) {
    graph.elements.push({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: inter.id,
      kind: NODE_KIND.ImportIntermediate,
      name: inter.name,
      line: inter.line,
      isJsxElement: false,
    });
  }
  for (const v of ir.variables) {
    if (hiddenVariables.has(v.id)) {
      continue;
    }
    const def = v.defs[0];
    if (def?.type !== "ImportBinding") {
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
      def.importKind === "named" &&
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

  for (const id of ir.unusedVariableIds) {
    const target = nodeId(id);
    const node = findNodeById(graph.elements, target);
    if (node) {
      node.unused = true;
    }
  }

  return graph;
}
