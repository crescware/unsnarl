import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { isBranchScope } from "./is-branch-scope.js";

export function branchScopeOf(
  scopeId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): string | null {
  let cur: SerializedScope | null = scopeMap.get(scopeId) ?? null;
  while (cur) {
    if (isBranchScope(cur.id, scopeMap)) {
      return cur.id;
    }
    if (!cur.upper) {
      return null;
    }
    cur = scopeMap.get(cur.upper) ?? null;
  }
  return null;
}
