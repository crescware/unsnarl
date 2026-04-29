import type {
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
} from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";

const MODULE_ROOT_ID = "module_root";

export class MermaidEmitter implements Emitter {
  readonly format = "mermaid";
  readonly contentType = "text/vnd.mermaid";

  emit(ir: SerializedIR, _opts: EmitOptions): string {
    const lines: string[] = ["flowchart RL"];

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

    function controlScopeLabel(scope: SerializedScope): string | null {
      const line = scope.block.span.line;
      if (scope.type === "catch") {
        return `catch L${line}`;
      }
      if (scope.type === "for") {
        return `for L${line}`;
      }
      if (scope.type === "switch") {
        return `switch L${line}`;
      }
      if (scope.type === "block") {
        const ctx = scope.blockContext;
        if (!ctx) {
          return null;
        }
        if (ctx.parentType === "TryStatement") {
          if (ctx.key === "block") {
            return `try L${line}`;
          }
          if (ctx.key === "finalizer") {
            return `finally L${line}`;
          }
        }
        if (ctx.parentType === "IfStatement") {
          if (ctx.key === "consequent") {
            return `if L${line}`;
          }
          if (ctx.key === "alternate") {
            return `else L${line}`;
          }
        }
        if (ctx.parentType === "SwitchStatement" && ctx.key === "cases") {
          return `case L${line}`;
        }
      }
      return null;
    }

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

    function isControlSubgraph(scope: SerializedScope): boolean {
      return controlScopeLabel(scope) !== null;
    }

    function shouldSubgraph(scope: SerializedScope): boolean {
      return isFunctionSubgraph(scope) || isControlSubgraph(scope);
    }

    const subgraphScopeId = (scope: SerializedScope): string => {
      const ownerVar = subgraphOwnerVar.get(scope.id);
      if (ownerVar) {
        return nodeId(ownerVar);
      }
      return `s_${sanitize(scope.id)}`;
    };

    const subgraphLabel = (scope: SerializedScope): string => {
      const ownerVar = subgraphOwnerVar.get(scope.id);
      if (ownerVar) {
        const v = variableMap.get(ownerVar);
        if (v) {
          return variableLabel(v);
        }
      }
      return controlScopeLabel(scope) ?? scope.type;
    };

    const subgraphOwnerVarSet = new Set(subgraphOwnerVar.values());

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

    const ifContainerLabel = (
      group: SerializedScope[],
      offset: number,
    ): string => {
      const hasElse = group.some((g) => g.blockContext?.key === "alternate");
      return `${hasElse ? "if-else" : "if"} L${lineForOffset(offset)}`;
    };

    const emitChildren = (
      parentScope: SerializedScope,
      indent: string,
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
          emitScope(child, indent);
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
            emitScope(g, indent);
          }
          i = j;
          continue;
        }
        const offset = child.blockContext?.parentSpanOffset ?? 0;
        const containerId = ifContainerSubgraphId(child.upper ?? "", offset);
        lines.push(
          `${indent}subgraph ${containerId}["${ifContainerLabel(group, offset)}"]`,
        );
        lines.push(`${indent}  direction RL`);
        for (const g of group) {
          emitScope(g, `${indent}  `);
        }
        lines.push(`${indent}end`);
        i = j;
      }
    };

    const emitScope = (scope: SerializedScope, indent: string): void => {
      const subgraph = shouldSubgraph(scope);
      const childIndent = subgraph ? `${indent}  ` : indent;
      if (subgraph) {
        lines.push(
          `${indent}subgraph ${subgraphScopeId(scope)}["${subgraphLabel(scope)}"]`,
        );
        lines.push(`${childIndent}direction RL`);
        const ownerVar = subgraphOwnerVar.get(scope.id);
        if (ownerVar && returnTargets.has(ownerVar)) {
          lines.push(`${childIndent}${returnNodeId(ownerVar)}((return))`);
        }
      }
      for (const vid of scope.variables) {
        if (hiddenVariables.has(vid)) {
          continue;
        }
        if (subgraphOwnerVarSet.has(vid)) {
          continue;
        }
        const v = variableMap.get(vid);
        if (!v) {
          continue;
        }
        lines.push(`${childIndent}${nodeId(vid)}["${variableLabel(v)}"]`);
      }
      const ops = writeOpsByScope.get(scope.id) ?? [];
      for (const op of ops) {
        const ownerVar = variableMap.get(op.varId);
        const isLet = ownerVar?.defs[0]?.declarationKind === "let";
        const head = isLet ? `let ${escape(op.varName)}` : escape(op.varName);
        lines.push(
          `${childIndent}${writeOpNodeId(op.refId)}(["${head}<br/>L${op.line}"])`,
        );
      }
      emitChildren(scope, childIndent);
      if (subgraph) {
        lines.push(`${indent}end`);
      }
    };

    const root = ir.scopes.find(
      (s) => s.type === "module" || s.type === "global",
    );
    if (root) {
      emitScope(root, "  ");
    }

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
        let lastOp: WriteOp | null = null;
        for (const op of prev) {
          if (
            op.scopeId === branchId ||
            isAncestorScope(branchId, op.scopeId)
          ) {
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
          isFirstInCase && previousFallthroughCase(opScope!)
            ? "fallthrough"
            : "set";
        lines.push(`  ${prevId} -->|${edgeKind}| ${writeOpNodeId(op.refId)}`);
      }
    }

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

    let needsModuleRoot = false;
    const emittedEdges = new Set<string>();
    const pushEdge = (from: string, label: string, to: string): void => {
      const key = `${from} -->|${label}| ${to}`;
      if (emittedEdges.has(key)) {
        return;
      }
      emittedEdges.add(key);
      lines.push(`  ${key}`);
    };

    for (const r of ir.references) {
      if (!r.resolved) {
        continue;
      }
      if (hiddenVariables.has(r.resolved)) {
        continue;
      }
      const predicateTarget = predicateTargetId(r);
      if (predicateTarget) {
        const fromIds = readOrigins(
          r.resolved,
          r.identifier.span.offset,
          r.from,
        );
        for (const fromId of fromIds) {
          pushEdge(fromId, edgeLabel(r), predicateTarget);
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
            pushEdge(fromId, edgeLabel(r), targetId);
          }
        }
        continue;
      }
      const label = edgeLabel(r);
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
          for (const fromId of fromIds) {
            pushEdge(fromId, label, returnNodeId(enclosingFn));
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
      lines.push(`  ${MODULE_ROOT_ID}((module))`);
    }

    type ModuleNode = { id: string; line: number };
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

    for (const [source, mod] of moduleNodes) {
      lines.push(`  ${mod.id}["module ${escape(source)}<br/>L${mod.line}"]`);
    }
    for (const inter of intermediates.values()) {
      lines.push(
        `  ${inter.id}["import ${escape(inter.name)}<br/>L${inter.line}"]`,
      );
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
          lines.push(`  ${mod.id} -->|read| ${inter.id}`);
          lines.push(`  ${inter.id} -->|read| ${localId}`);
          continue;
        }
      }
      lines.push(`  ${mod.id} -->|read| ${localId}`);
    }

    if (ir.unusedVariableIds.length > 0) {
      lines.push("  classDef unused stroke-dasharray: 5 5;");
      for (const id of ir.unusedVariableIds) {
        lines.push(`  class ${nodeId(id)} unused;`);
      }
    }

    return `${lines.join("\n")}\n`;
  }
}

function variableLabel(v: SerializedVariable): string {
  const def = v.defs[0];
  const kind = def?.type;
  const line = v.identifiers[0]?.line ?? def?.name.span.line ?? 0;
  const name = escape(v.name);
  const initType = def?.initType;
  const isFunctionInit =
    initType === "ArrowFunctionExpression" || initType === "FunctionExpression";
  const isLet = def?.declarationKind === "let";
  let head: string;
  switch (kind) {
    case "FunctionName":
      head = `${name}()`;
      break;
    case "ClassName":
      head = `class ${name}`;
      break;
    case "ImportBinding":
      head = def?.importKind === "namespace" ? `import ${name}` : name;
      break;
    case "CatchClause":
      head = `catch ${name}`;
      break;
    case "ImplicitGlobalVariable":
      head = `global ${name}`;
      break;
    default:
      if (isFunctionInit) {
        head = `${name}()`;
      } else if (isLet) {
        head = `let ${name}`;
      } else {
        head = name;
      }
  }
  return `${head}<br/>L${line}`;
}

function edgeLabel(r: SerializedReference): string {
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
  return `n_${sanitize(id)}`;
}

function returnNodeId(varId: string): string {
  return `return_${sanitize(varId)}`;
}

function writeOpNodeId(refId: string): string {
  return `wr_${sanitize(refId)}`;
}

function sanitize(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]/g, "_");
}

function escape(value: string): string {
  return value.replace(/"/g, '\\"');
}
