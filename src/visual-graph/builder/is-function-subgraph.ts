import type { SerializedScope } from "../../ir/model.js";

export function isFunctionSubgraph(
  scope: SerializedScope,
  subgraphOwnerVar: ReadonlyMap<string, string>,
): boolean {
  return subgraphOwnerVar.has(scope.id);
}
