import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";

export function isAncestorScope(
  ancestorId: string,
  descendantId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): boolean {
  let cur: SerializedScope | null = scopeMap.get(descendantId) ?? null;
  while (cur) {
    if (cur.id === ancestorId) {
      return true;
    }
    if (!cur.upper) {
      return false;
    }
    cur = scopeMap.get(cur.upper) ?? null;
  }
  return false;
}
