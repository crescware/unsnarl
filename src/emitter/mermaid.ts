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
    const lines: string[] = ["flowchart LR"];
    const variableMap = new Map<string, SerializedVariable>();
    for (const v of ir.variables) {
      variableMap.set(v.id, v);
    }
    const scopeMap = new Map<string, SerializedScope>();
    for (const s of ir.scopes) {
      scopeMap.set(s.id, s);
    }

    let needsModuleRoot = false;
    const declared = ir.variables.filter(
      (v) => v.defs[0]?.type !== "ImplicitGlobalVariable",
    );
    const implicit = ir.variables.filter(
      (v) => v.defs[0]?.type === "ImplicitGlobalVariable",
    );

    for (const v of declared) {
      lines.push(`  ${nodeId(v.id)}["${variableLabel(v)}"]`);
    }
    for (const v of implicit) {
      lines.push(`  ${nodeId(v.id)}["${unresolvedLabel(v)}"]`);
    }

    for (const r of ir.references) {
      const fromVarId =
        r.owner ?? findEnclosingVariableId(r, scopeMap, variableMap);
      const fromId = fromVarId ? nodeId(fromVarId) : MODULE_ROOT_ID;
      if (!fromVarId) {
        needsModuleRoot = true;
      }
      const toId = r.resolved
        ? nodeId(r.resolved)
        : `${MODULE_ROOT_ID}__${sanitize(r.identifier.name)}`;
      const label = edgeLabel(r);
      lines.push(`  ${fromId} -->|${label}| ${toId}`);
    }

    if (needsModuleRoot) {
      lines.push(`  ${MODULE_ROOT_ID}["(module)"]`);
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
  return `${escape(v.name)} : ${kind}\\nL${line}`;
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

function findEnclosingVariableId(
  r: SerializedReference,
  scopeMap: Map<string, SerializedScope>,
  variableMap: Map<string, SerializedVariable>,
): string | null {
  let cur = scopeMap.get(r.from);
  while (cur && cur.upper) {
    const upper = scopeMap.get(cur.upper);
    if (!upper) {
      return null;
    }
    for (const vid of upper.variables) {
      const v = variableMap.get(vid);
      if (!v) {
        continue;
      }
      if (v.defs.some((d) => d.node.span.offset === cur!.block.span.offset)) {
        return vid;
      }
    }
    cur = upper;
  }
  return null;
}

function nodeId(id: string): string {
  return `n_${sanitize(id)}`;
}

function sanitize(value: string): string {
  return value.replace(/[^a-zA-Z0-9_]/g, "_");
}

function escape(value: string): string {
  return value.replace(/"/g, '\\"');
}
