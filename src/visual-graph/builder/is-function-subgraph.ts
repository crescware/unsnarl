import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";

export function isFunctionSubgraph(
  scope: SerializedScope,
  subgraphOwnerVar: ReadonlyMap<string, string>,
): boolean {
  return subgraphOwnerVar.has(scope.id);
}
