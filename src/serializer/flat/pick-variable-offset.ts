import type { Variable } from "../../ir/model.js";

export function pickVariableOffset(v: Variable): number {
  const head = v.identifiers[0];
  if (head !== undefined) {
    return head.start ?? 0;
  }
  const def = v.defs[0];
  if (def !== undefined) {
    return def.name.start ?? 0;
  }
  return 0;
}
