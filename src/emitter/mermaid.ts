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
      if (!def || def.type !== "FunctionName") {
        continue;
      }
      const fnScope = ir.scopes.find(
        (s) =>
          s.upper === v.scope && s.block.span.offset === def.node.span.offset,
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

    const groups = new Map<string | null, string[]>();
    for (const v of ir.variables) {
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
        } else if (v.defs[0]?.type === "ImplicitGlobalVariable") {
          lines.push(`${indent}${nodeId(vid)}["${unresolvedLabel(v)}"]`);
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
  const kind = v.defs[0]?.type ?? "Variable";
  const line = v.identifiers[0]?.line ?? v.defs[0]?.name.span.line ?? 0;
  return `${escape(v.name)} : ${kind}<br/>L${line}`;
}

function unresolvedLabel(v: SerializedVariable): string {
  return `(unresolved:${escape(v.name)})`;
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
