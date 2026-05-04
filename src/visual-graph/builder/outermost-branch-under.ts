import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { isBranchScope } from "./is-branch-scope.js";

// Walk upward from scopeId and return the outermost branch scope strictly
// nested under branchId. Returns null when scopeId is branchId itself,
// when scopeId is not actually under branchId, or when there is no
// intervening branch scope. Used to discover the merge container immediately
// underneath a given outer branch (e.g. an if/else nested inside a switch
// case) so callers can recurse one level at a time instead of jumping
// straight to the deepest branch container.
export function outermostBranchUnder(
  branchId: string,
  scopeId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): string | null {
  if (scopeId === branchId) {
    return null;
  }
  let result: string | null = null;
  let cur: SerializedScope | null = scopeMap.get(scopeId) ?? null;
  while (cur && cur.id !== branchId) {
    if (isBranchScope(cur.id, scopeMap)) {
      result = cur.id;
    }
    if (!cur.upper) {
      return null;
    }
    cur = scopeMap.get(cur.upper) ?? null;
  }
  if (cur?.id !== branchId) {
    return null;
  }
  return result;
}
