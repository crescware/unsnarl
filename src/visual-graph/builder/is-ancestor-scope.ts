import type { SerializedScope } from "../../ir/model.js";

export function isAncestorScope(
  ancestorId: string,
  descendantId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): boolean {
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
}
