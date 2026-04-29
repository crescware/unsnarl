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

    const subgraphScopes = new Map<string, string>();
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
        subgraphScopes.set(v.id, fnScope.id);
      }
    }
    const scopeToSubgraph = new Map<string, string>();
    for (const [vid, sid] of subgraphScopes) {
      scopeToSubgraph.set(sid, vid);
    }

    const findEnclosingSubgraphVar = (scopeId: string): string | null => {
      let cur: SerializedScope | undefined = scopeMap.get(scopeId);
      while (cur) {
        const owner = scopeToSubgraph.get(cur.id);
        if (owner) {
          return owner;
        }
        if (!cur.upper) {
          return null;
        }
        cur = scopeMap.get(cur.upper);
      }
      return null;
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

    const groups = new Map<string | null, string[]>();
    for (const v of ir.variables) {
      if (hiddenVariables.has(v.id)) {
        continue;
      }
      const enclosing = findEnclosingSubgraphVar(v.scope);
      const arr = groups.get(enclosing) ?? [];
      arr.push(v.id);
      groups.set(enclosing, arr);
    }

    const returnTargets = new Map<string, SerializedReference[]>();
    for (const r of ir.references) {
      if (r.owners.length > 0) {
        continue;
      }
      const enclosingFn = findEnclosingSubgraphVar(r.from);
      if (!enclosingFn) {
        continue;
      }
      const arr = returnTargets.get(enclosingFn) ?? [];
      arr.push(r);
      returnTargets.set(enclosingFn, arr);
    }

    const emitGroup = (group: string | null, indent: string): void => {
      const vars = groups.get(group) ?? [];
      for (const vid of vars) {
        const v = variableMap.get(vid);
        if (!v) {
          continue;
        }
        if (subgraphScopes.has(vid)) {
          lines.push(`${indent}subgraph ${nodeId(vid)}["${variableLabel(v)}"]`);
          lines.push(`${indent}  direction RL`);
          if (returnTargets.has(vid)) {
            lines.push(`${indent}  ${returnNodeId(vid)}((return))`);
          }
          emitGroup(vid, `${indent}  `);
          lines.push(`${indent}end`);
        } else {
          lines.push(`${indent}${nodeId(vid)}["${variableLabel(v)}"]`);
        }
      }
    };
    emitGroup(null, "  ");

    let needsModuleRoot = false;
    for (const r of ir.references) {
      if (!r.resolved) {
        continue;
      }
      if (hiddenVariables.has(r.resolved)) {
        continue;
      }
      const label = edgeLabel(r);
      const fromId = nodeId(r.resolved);
      if (r.owners.length > 0) {
        for (const ownerId of r.owners) {
          lines.push(`  ${fromId} -->|${label}| ${nodeId(ownerId)}`);
        }
      } else {
        const enclosingFn = findEnclosingSubgraphVar(r.from);
        if (enclosingFn) {
          lines.push(`  ${fromId} -->|${label}| ${returnNodeId(enclosingFn)}`);
        } else {
          needsModuleRoot = true;
          lines.push(`  ${fromId} -->|${label}| ${MODULE_ROOT_ID}`);
        }
      }
    }

    if (needsModuleRoot) {
      lines.push(`  ${MODULE_ROOT_ID}((module))`);
    }

    const moduleNodes = new Map<string, string>();
    type Intermediate = { id: string; name: string };
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
        moduleNodes.set(source, `mod_${sanitize(source)}`);
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
          });
        }
      }
    }

    for (const [source, id] of moduleNodes) {
      lines.push(`  ${id}["module ${escape(source)}"]`);
    }
    for (const inter of intermediates.values()) {
      lines.push(`  ${inter.id}["import ${escape(inter.name)}"]`);
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
      const modId = moduleNodes.get(source);
      if (!modId) {
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
          lines.push(`  ${modId} -->|read| ${inter.id}`);
          lines.push(`  ${inter.id} -->|read| ${localId}`);
          continue;
        }
      }
      lines.push(`  ${modId} -->|read| ${localId}`);
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
      head = isFunctionInit ? `${name}()` : name;
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

function sanitize(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]/g, "_");
}

function escape(value: string): string {
  return value.replace(/"/g, '\\"');
}
