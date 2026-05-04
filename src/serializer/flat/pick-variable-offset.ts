import type { Variable } from "../../ir/scope/variable.js";

export function pickVariableOffset(v: Variable): number {
  const head = v.identifiers[0] ?? null;
  if (head !== null) {
    return head.start ?? 0;
  }
  const def = v.defs[0] ?? null;
  if (def !== null) {
    return def.name.start ?? 0;
  }
  return 0;
}
