import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";

export function findEnclosingSubgraphScope(
  scopeId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
  subgraphOwnerVar: ReadonlyMap<string, string>,
): string | null {
  let cur: SerializedScope | null = scopeMap.get(scopeId) ?? null;
  while (cur) {
    if (subgraphOwnerVar.has(cur.id)) {
      return cur.id;
    }
    if (!cur.upper) {
      return null;
    }
    cur = scopeMap.get(cur.upper) ?? null;
  }
  return null;
}
