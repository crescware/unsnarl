import type {
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
} from "../ir/model.js";
import type {
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";

const MODULE_ROOT_ID = "module_root";

export function buildVisualGraph(ir: SerializedIR): VisualGraph {
  const graph: VisualGraph = {
    version: 1,
    source: { path: ir.source.path, language: ir.source.language },
    direction: "RL",
    elements: [],
    edges: [],
  };
  const edges = graph.edges;

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

  const findEnclosingSubgraphScope = (scopeId: string): string | null => {
    let cur: SerializedScope | undefined = scopeMap.get(scopeId);
    while (cur) {
      if (subgraphOwnerVar.has(cur.id)) {
        return cur.id;
      }
      if (!cur.upper) {
        return null;
      }
      cur = scopeMap.get(cur.upper);
    }
    return null;
  };

  const enclosingFunctionVar = (scopeId: string): string | null => {
    const fnScopeId = findEnclosingSubgraphScope(scopeId);
    if (fnScopeId === null) {
      return null;
    }
    return subgraphOwnerVar.get(fnScopeId) ?? null;
  };

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

  const returnTargets = new Map<string, SerializedReference[]>();
  for (const r of ir.references) {
    if (r.owners.length > 0) {
      continue;
    }
    const enclosingFn = enclosingFunctionVar(r.from);
    if (!enclosingFn) {
      continue;
    }
    const arr = returnTargets.get(enclosingFn) ?? [];
    arr.push(r);
    returnTargets.set(enclosingFn, arr);
  }

  type WriteOp = {
    refId: string;
    varId: string;
    varName: string;
    line: number;
    offset: number;
    scopeId: string;
  };
  const refsByVariable = new Map<string, SerializedReference[]>();
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
  const writeOpsByVariable = new Map<string, WriteOp[]>();
  const writeOpsByScope = new Map<string, WriteOp[]>();
  const writeOpByRef = new Map<string, WriteOp>();
  for (const v of ir.variables) {
    if (hiddenVariables.has(v.id)) {
      continue;
    }
    const refs = refsByVariable.get(v.id) ?? [];
    const ops: WriteOp[] = [];
    for (const r of refs) {
      if (!r.flags.write) {
        continue;
      }
      const op: WriteOp = {
        refId: r.id,
        varId: v.id,
        varName: v.name,
        line: r.identifier.span.line,
        offset: r.identifier.span.offset,
        scopeId: r.from,
      };
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

  const stateAt = (varId: string, offset: number): string => {
    const ops = writeOpsByVariable.get(varId) ?? [];
    let last: WriteOp | null = null;
    for (const op of ops) {
      if (op.offset >= offset) {
        break;
      }
      last = op;
    }
    return last ? last.refId : varId;
  };

  const isAncestorScope = (
    ancestorId: string,
    descendantId: string,
  ): boolean => {
    let cur: SerializedScope | undefined = scopeMap.get(descendantId);
    while (cur) {
      if (cur.id === ancestorId) {
        return true;
      }
      if (!cur.upper) {
        return false;
      }
      cur = scopeMap.get(cur.upper);
    }
    return false;
  };

  const branchContainerKey = (scope: SerializedScope): string | null => {
    const ctx = scope.blockContext;
    if (!ctx) {
      return null;
    }
    if (ctx.parentType === "SwitchStatement" && ctx.key === "cases") {
      return `switch:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
    }
    if (
      ctx.parentType === "IfStatement" &&
      (ctx.key === "consequent" || ctx.key === "alternate")
    ) {
      return `if:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
    }
    return null;
  };

  const isBranchScope = (scopeId: string): boolean => {
    const scope = scopeMap.get(scopeId);
    return scope ? branchContainerKey(scope) !== null : false;
  };

  const branchScopeOf = (scopeId: string): string | null => {
    let cur: SerializedScope | undefined = scopeMap.get(scopeId);
    while (cur) {
      if (isBranchScope(cur.id)) {
        return cur.id;
      }
      if (!cur.upper) {
        return null;
      }
      cur = scopeMap.get(cur.upper);
    }
    return null;
  };

  function isFunctionSubgraph(scope: SerializedScope): boolean {
    return subgraphOwnerVar.has(scope.id);
  }

  function controlSubgraphKindOf(
    scope: SerializedScope,
  ): VisualSubgraph["kind"] | null {
    if (scope.type === "catch") {
      return "catch";
    }
    if (scope.type === "for") {
      return "for";
    }
    if (scope.type === "switch") {
      return "switch";
    }
    if (scope.type === "block") {
      const ctx = scope.blockContext;
      if (!ctx) {
        return null;
      }
      if (ctx.parentType === "TryStatement") {
        if (ctx.key === "block") {
          return "try";
        }
        if (ctx.key === "finalizer") {
          return "finally";
        }
      }
      if (ctx.parentType === "IfStatement") {
        if (ctx.key === "consequent") {
          return "if";
        }
        if (ctx.key === "alternate") {
          return "else";
        }
      }
      if (ctx.parentType === "SwitchStatement" && ctx.key === "cases") {
        return "case";
      }
    }
    return null;
  }

  function isControlSubgraph(scope: SerializedScope): boolean {
    return controlSubgraphKindOf(scope) !== null;
  }

  function shouldSubgraph(scope: SerializedScope): boolean {
    return isFunctionSubgraph(scope) || isControlSubgraph(scope);
  }

  const subgraphScopeId = (scope: SerializedScope): string =>
    `s_${sanitize(scope.id)}`;

  const lineForOffset = (offset: number): number => {
    let line = 1;
    const limit = Math.min(offset, ir.raw.length);
    for (let i = 0; i < limit; i++) {
      if (ir.raw.charCodeAt(i) === 10) {
        line += 1;
      }
    }
    return line;
  };

  const ifContainerSubgraphId = (
    parentScopeId: string,
    offset: number,
  ): string => `cont_if_${sanitize(parentScopeId)}_${offset}`;

  const sortedCasesByContainer = new Map<string, SerializedScope[]>();
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

  const previousFallthroughCase = (
    caseScope: SerializedScope,
  ): SerializedScope | null => {
    const ckey = branchContainerKey(caseScope);
    if (!ckey) {
      return null;
    }
    const cases = sortedCasesByContainer.get(ckey);
    if (!cases) {
      return null;
    }
    const idx = cases.indexOf(caseScope);
    if (idx <= 0) {
      return null;
    }
    const prev = cases[idx - 1];
    if (!prev) {
      return null;
    }
    return prev.fallsThrough ? prev : null;
  };

  const lastWriteOpInScopeBefore = (
    varId: string,
    scopeId: string,
    offset: number,
  ): WriteOp | null => {
    const ops = writeOpsByVariable.get(varId) ?? [];
    let last: WriteOp | null = null;
    for (const op of ops) {
      if (op.offset >= offset) {
        break;
      }
      if (op.scopeId === scopeId || isAncestorScope(scopeId, op.scopeId)) {
        last = op;
      }
    }
    return last;
  };

  const stateRefId = (refId: string, varId: string): string => {
    const op = writeOpByRef.get(refId);
    if (op) {
      return writeOpNodeId(op.refId);
    }
    const ref = ir.references.find((r) => r.id === refId);
    if (!ref) {
      return nodeId(varId);
    }
    const stateRef = stateAt(varId, ref.identifier.span.offset);
    return stateRef === varId ? nodeId(varId) : writeOpNodeId(stateRef);
  };

  const readOrigins = (
    varId: string,
    refOffset: number,
    refScopeId: string,
  ): string[] => {
    const ops = writeOpsByVariable.get(varId) ?? [];
    const prev = ops.filter((op) => op.offset < refOffset);
    const last = prev[prev.length - 1];
    if (!last) {
      return [nodeId(varId)];
    }
    if (isAncestorScope(last.scopeId, refScopeId)) {
      return [writeOpNodeId(last.refId)];
    }
    const lastBranchId = branchScopeOf(last.scopeId);
    if (!lastBranchId) {
      return [writeOpNodeId(last.refId)];
    }
    const lastBranchScope = scopeMap.get(lastBranchId);
    if (!lastBranchScope) {
      return [writeOpNodeId(last.refId)];
    }
    const containerKey = branchContainerKey(lastBranchScope);
    if (containerKey === null) {
      return [writeOpNodeId(last.refId)];
    }

    const branchScopeIds: string[] = [];
    for (const s of ir.scopes) {
      if (branchContainerKey(s) === containerKey) {
        branchScopeIds.push(s.id);
      }
    }

    const origins: string[] = [];
    const isSwitch = containerKey.startsWith("switch:");
    for (const branchId of branchScopeIds) {
      const branchScope = scopeMap.get(branchId);
      if (isSwitch && branchScope !== undefined && branchScope.fallsThrough) {
        const cases = sortedCasesByContainer.get(containerKey);
        if (cases) {
          const idx = cases.indexOf(branchScope);
          if (idx >= 0 && idx < cases.length - 1) {
            continue;
          }
        }
      }
      // Cases ending in return/throw exit the function, so their writes
      // never reach a read after the switch.
      if (isSwitch && branchScope !== undefined && branchScope.exitsFunction) {
        continue;
      }
      let lastOp: WriteOp | null = null;
      for (const op of prev) {
        if (op.scopeId === branchId || isAncestorScope(branchId, op.scopeId)) {
          lastOp = op;
        }
      }
      if (lastOp) {
        origins.push(writeOpNodeId(lastOp.refId));
      }
    }

    if (containerKey.startsWith("if:")) {
      const hasAlternate = branchScopeIds.some((id) => {
        const s = scopeMap.get(id);
        return s?.blockContext?.key === "alternate";
      });
      if (!hasAlternate) {
        const ifOffset = lastBranchScope.blockContext?.parentSpanOffset ?? 0;
        const before = ops.filter((op) => op.offset < ifOffset);
        const lastBefore = before[before.length - 1];
        if (lastBefore) {
          origins.push(writeOpNodeId(lastBefore.refId));
        } else {
          origins.push(nodeId(varId));
        }
      }
    }

    if (origins.length === 0) {
      return [writeOpNodeId(last.refId)];
    }
    return Array.from(new Set(origins));
  };

  const ownerTargetId = (ownerVarId: string, offset: number): string => {
    const ops = writeOpsByVariable.get(ownerVarId) ?? [];
    let last: WriteOp | null = null;
    for (const op of ops) {
      if (op.offset > offset) {
        break;
      }
      last = op;
    }
    return last ? writeOpNodeId(last.refId) : nodeId(ownerVarId);
  };

  const predicateTargetId = (r: SerializedReference): string | null => {
    const pc = r.predicateContainer;
    if (!pc) {
      return null;
    }
    if (pc.type === "SwitchStatement") {
      let cur = scopeMap.get(r.from);
      while (cur) {
        if (cur.type === "switch" && cur.block.span.offset === pc.offset) {
          return `s_${sanitize(cur.id)}`;
        }
        if (!cur.upper) {
          break;
        }
        cur = scopeMap.get(cur.upper);
      }
      return null;
    }
    const containerKey = `if:${r.from}:${pc.offset}`;
    const branches = ir.scopes.filter(
      (s) => branchContainerKey(s) === containerKey,
    );
    if (branches.length >= 2) {
      return ifContainerSubgraphId(r.from, pc.offset);
    }
    const single = branches[0];
    if (single) {
      return `s_${sanitize(single.id)}`;
    }
    return null;
  };

  type Container = { elements: VisualElement[] };
  const functionSubgraphByFn = new Map<string, VisualSubgraph>();
  const subgraphByScope = new Map<string, VisualSubgraph>();

  const buildScope = (scope: SerializedScope, container: Container): void => {
    const subgraphHere = shouldSubgraph(scope);
    let bodyContainer: Container = container;
    if (subgraphHere) {
      const sg = describeSubgraph(scope);
      container.elements.push(sg);
      bodyContainer = sg;
      subgraphByScope.set(scope.id, sg);
      const ownerVar = subgraphOwnerVar.get(scope.id);
      if (ownerVar) {
        functionSubgraphByFn.set(ownerVar, sg);
      }
    }
    for (const vid of scope.variables) {
      if (hiddenVariables.has(vid)) {
        continue;
      }
      const v = variableMap.get(vid);
      if (!v) {
        continue;
      }
      bodyContainer.elements.push(makeVariableNode(v));
    }
    const ops = writeOpsByScope.get(scope.id) ?? [];
    for (const op of ops) {
      const ownerVar = variableMap.get(op.varId);
      const declarationKind = ownerVar?.defs[0]?.declarationKind;
      const node: VisualNode = {
        type: "node",
        id: writeOpNodeId(op.refId),
        kind: "WriteOp",
        name: op.varName,
        line: op.line,
      };
      if (declarationKind) {
        node.declarationKind = declarationKind;
      }
      bodyContainer.elements.push(node);
    }
    buildChildren(scope, bodyContainer);
  };

  const buildChildren = (
    parentScope: SerializedScope,
    container: Container,
  ): void => {
    const children: SerializedScope[] = [];
    for (const id of parentScope.childScopes) {
      const c = scopeMap.get(id);
      if (c) {
        children.push(c);
      }
    }
    let i = 0;
    while (i < children.length) {
      const child = children[i];
      if (!child) {
        i++;
        continue;
      }
      const ckey = branchContainerKey(child);
      if (ckey === null || !ckey.startsWith("if:")) {
        buildScope(child, container);
        i++;
        continue;
      }
      const group: SerializedScope[] = [child];
      let j = i + 1;
      while (j < children.length) {
        const next = children[j];
        if (!next || branchContainerKey(next) !== ckey) {
          break;
        }
        group.push(next);
        j++;
      }
      if (group.length < 2) {
        for (const g of group) {
          buildScope(g, container);
        }
        i = j;
        continue;
      }
      const offset = child.blockContext?.parentSpanOffset ?? 0;
      const containerId = ifContainerSubgraphId(child.upper ?? "", offset);
      const hasElse = group.some((g) => g.blockContext?.key === "alternate");
      const containerSubgraph: VisualSubgraph = {
        type: "subgraph",
        id: containerId,
        kind: "if-else-container",
        line: lineForOffset(offset),
        direction: "RL",
        hasElse,
        elements: [],
      };
      container.elements.push(containerSubgraph);
      for (const g of group) {
        buildScope(g, containerSubgraph);
      }
      let containerEndLine = containerSubgraph.line;
      for (const elem of containerSubgraph.elements) {
        if (elem.type === "subgraph" && elem.endLine !== undefined) {
          containerEndLine = Math.max(containerEndLine, elem.endLine);
        }
      }
      if (containerEndLine !== containerSubgraph.line) {
        containerSubgraph.endLine = containerEndLine;
      }
      i = j;
    }
  };

  const describeSubgraph = (scope: SerializedScope): VisualSubgraph => {
    const id = subgraphScopeId(scope);
    const endLine = scope.block.endSpan.line;
    if (isFunctionSubgraph(scope)) {
      const ownerVarId = subgraphOwnerVar.get(scope.id);
      if (!ownerVarId) {
        throw new Error(
          `expected owner variable for function subgraph ${scope.id}`,
        );
      }
      const ownerVar = variableMap.get(ownerVarId);
      const startLine = ownerVar?.identifiers[0]?.line ?? scope.block.span.line;
      return {
        type: "subgraph",
        id,
        kind: "function",
        line: startLine,
        endLine,
        direction: "RL",
        ownerNodeId: nodeId(ownerVarId),
        ownerName: ownerVar?.name ?? "",
        elements: [],
      };
    }
    const kind = controlSubgraphKindOf(scope);
    if (kind === null) {
      throw new Error(
        `expected control subgraph kind for scope ${scope.id} (type=${scope.type})`,
      );
    }
    const sg: VisualSubgraph = {
      type: "subgraph",
      id,
      kind,
      line: scope.block.span.line,
      endLine,
      direction: "RL",
      elements: [],
    };
    if (kind === "case") {
      sg.caseTest = scope.blockContext?.caseTest ?? null;
    }
    return sg;
  };

  const root = ir.scopes.find(
    (s) => s.type === "module" || s.type === "global",
  );
  if (root) {
    buildScope(root, graph);
  }

  let needsModuleRoot = false;
  const emittedEdges = new Set<string>();
  function pushEdge(from: string, label: string, to: string): void {
    const key = `${from} -->|${label}| ${to}`;
    if (emittedEdges.has(key)) {
      return;
    }
    emittedEdges.add(key);
    edges.push({ from, to, label });
  }

  const returnUseAdded = new Set<string>();
  const returnSubgraphsByFn = new Map<string, Map<string, VisualSubgraph>>();
  const findHostSubgraph = (
    ref: SerializedReference,
    enclosingFnVarId: string,
  ): VisualSubgraph | null => {
    let cur: SerializedScope | undefined = scopeMap.get(ref.from);
    while (cur) {
      const sg = subgraphByScope.get(cur.id);
      if (sg) {
        return sg;
      }
      if (!cur.upper) {
        break;
      }
      cur = scopeMap.get(cur.upper);
    }
    return functionSubgraphByFn.get(enclosingFnVarId) ?? null;
  };
  function ensureReturnUseNode(
    enclosingFnVarId: string,
    ref: SerializedReference,
  ): string | null {
    const host = findHostSubgraph(ref, enclosingFnVarId);
    if (!host) {
      return null;
    }
    const containerKey = ref.returnContainer
      ? `${ref.returnContainer.startSpan.offset}-${ref.returnContainer.endSpan.offset}`
      : "implicit";
    let perFn = returnSubgraphsByFn.get(enclosingFnVarId);
    if (!perFn) {
      perFn = new Map();
      returnSubgraphsByFn.set(enclosingFnVarId, perFn);
    }
    let sg = perFn.get(containerKey);
    if (!sg) {
      const startLine = ref.returnContainer?.startSpan.line ?? host.line;
      const endLine = ref.returnContainer?.endSpan.line;
      sg = {
        type: "subgraph",
        id: returnSubgraphId(enclosingFnVarId, containerKey),
        kind: "return",
        line: startLine,
        direction: "RL",
        elements: [],
      };
      if (endLine !== undefined && endLine !== startLine) {
        sg.endLine = endLine;
      }
      host.elements.push(sg);
      perFn.set(containerKey, sg);
    }
    const id = retUseNodeId(ref.id);
    if (!returnUseAdded.has(ref.id)) {
      returnUseAdded.add(ref.id);
      const v = ref.resolved ? variableMap.get(ref.resolved) : undefined;
      const name = v?.name ?? ref.identifier.name ?? "";
      sg.elements.push({
        type: "node",
        id,
        kind: "ReturnUse",
        name,
        line: ref.identifier.span.line,
      });
    }
    return id;
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
        const prevCase = previousFallthroughCase(opScope);
        if (prevCase) {
          const prevCaseLast = lastWriteOpInScopeBefore(
            op.varId,
            prevCase.id,
            op.offset,
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
          if (isAncestorScope(candidate.scopeId, op.scopeId)) {
            prevId = writeOpNodeId(candidate.refId);
            break;
          }
        }
      }
      const edgeKind =
        opScope && isFirstInCase && previousFallthroughCase(opScope)
          ? "fallthrough"
          : "set";
      pushEdge(prevId, edgeKind, writeOpNodeId(op.refId));
    }
  }

  for (const r of ir.references) {
    if (!r.resolved) {
      continue;
    }
    if (hiddenVariables.has(r.resolved)) {
      continue;
    }
    const predicateTarget = predicateTargetId(r);
    if (predicateTarget) {
      const fromIds = readOrigins(r.resolved, r.identifier.span.offset, r.from);
      for (const fromId of fromIds) {
        pushEdge(fromId, edgeLabelOfRef(r), predicateTarget);
      }
      continue;
    }
    if (r.flags.write) {
      if (r.flags.call || (r.flags.read && r.owners.length > 0)) {
        const fromId = stateRefId(r.id, r.resolved);
        for (const ownerId of r.owners) {
          if (ownerId === r.resolved) {
            continue;
          }
          const targetId = ownerTargetId(ownerId, r.identifier.span.offset);
          pushEdge(fromId, edgeLabelOfRef(r), targetId);
        }
      }
      continue;
    }
    const label = edgeLabelOfRef(r);
    const fromIds = readOrigins(r.resolved, r.identifier.span.offset, r.from);
    if (r.owners.length > 0) {
      for (const ownerId of r.owners) {
        if (ownerId === r.resolved) {
          continue;
        }
        const targetId = ownerTargetId(ownerId, r.identifier.span.offset);
        for (const fromId of fromIds) {
          pushEdge(fromId, label, targetId);
        }
      }
    } else {
      const enclosingFn = enclosingFunctionVar(r.from);
      if (enclosingFn) {
        const useTargetId = ensureReturnUseNode(enclosingFn, r);
        if (useTargetId) {
          for (const fromId of fromIds) {
            pushEdge(fromId, label, useTargetId);
          }
        }
      } else {
        needsModuleRoot = true;
        for (const fromId of fromIds) {
          pushEdge(fromId, label, MODULE_ROOT_ID);
        }
      }
    }
  }

  if (needsModuleRoot) {
    graph.elements.push({
      type: "node",
      id: MODULE_ROOT_ID,
      kind: "ModuleSink",
      name: "module",
      line: 0,
    });
  }

  // module sources, intermediates, and import edges
  type ModuleNode = { id: string; line: number; source: string };
  type Intermediate = { id: string; name: string; line: number };
  const moduleNodes = new Map<string, ModuleNode>();
  const intermediates = new Map<string, Intermediate>();
  const intermediateKey = (source: string, originalName: string): string =>
    `${source}::${originalName}`;

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
      type: "node",
      id: mod.id,
      kind: "ModuleSource",
      name: mod.source,
      line: mod.line,
    });
  }
  for (const inter of intermediates.values()) {
    graph.elements.push({
      type: "node",
      id: inter.id,
      kind: "ImportIntermediate",
      name: inter.name,
      line: inter.line,
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
        pushEdge(mod.id, "read", inter.id);
        pushEdge(inter.id, "read", localId);
        continue;
      }
    }
    pushEdge(mod.id, "read", localId);
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

function findNodeById(
  elements: VisualElement[],
  id: string,
): VisualNode | null {
  for (const e of elements) {
    if (e.type === "node") {
      if (e.id === id) {
        return e;
      }
    } else {
      const found = findNodeById(e.elements, id);
      if (found) {
        return found;
      }
    }
  }
  return null;
}

function makeVariableNode(v: SerializedVariable): VisualNode {
  const def = v.defs[0];
  const initType = def?.initType ?? null;
  const isFunctionInit =
    initType === "ArrowFunctionExpression" || initType === "FunctionExpression";
  const declarationKind = def?.declarationKind ?? null;
  const importKind = def?.importKind ?? null;
  const importedName = def?.importedName ?? null;
  const importSource = def?.importSource ?? null;
  const node: VisualNode = {
    type: "node",
    id: nodeId(v.id),
    kind: (def?.type ?? "Variable") as VisualNode["kind"],
    name: v.name,
    line: v.identifiers[0]?.line ?? def?.name.span.line ?? 0,
  };
  if (declarationKind) {
    node.declarationKind = declarationKind;
  }
  if (isFunctionInit) {
    node.initIsFunction = true;
  }
  if (importKind) {
    node.importKind = importKind;
  }
  if (def?.type === "ImportBinding") {
    node.importedName = importedName;
    if (importSource) {
      node.importSource = importSource;
    }
  }
  return node;
}

function edgeLabelOfRef(r: SerializedReference): string {
  const parts: string[] = [];
  if (r.flags.read) {
    parts.push("read");
  }
  if (r.flags.write) {
    parts.push("write");
  }
  if (r.flags.call) {
    parts.push("call");
  }
  return parts.length > 0 ? parts.join(",") : "ref";
}

function nodeId(id: string): string {
  return visualNodeIdFromVariableId(id);
}

/**
 * Stable mapping from an IR VariableId to the `n_*` id used by the
 * visual graph. Exposed so other layers (e.g. pruning) can resolve
 * references against the same canonical form without depending on a
 * private `sanitize` helper.
 */
export function visualNodeIdFromVariableId(varId: string): string {
  return `n_${sanitize(varId)}`;
}

function returnSubgraphId(varId: string, containerKey: string): string {
  return `s_return_${sanitize(varId)}_${sanitize(containerKey)}`;
}

function retUseNodeId(refId: string): string {
  return `ret_use_${sanitize(refId)}`;
}

function writeOpNodeId(refId: string): string {
  return `wr_${sanitize(refId)}`;
}

function sanitize(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]/g, "_");
}
