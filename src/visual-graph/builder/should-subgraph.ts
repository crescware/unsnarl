import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { isControlSubgraph } from "./is-control-subgraph.js";
import { isFunctionSubgraph } from "./is-function-subgraph.js";

export function shouldSubgraph(
  scope: SerializedScope,
  subgraphOwnerVar: ReadonlyMap<string, string>,
): boolean {
  return (
    isFunctionSubgraph(scope, subgraphOwnerVar) || isControlSubgraph(scope)
  );
}
