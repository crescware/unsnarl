import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";

export function isControlSubgraph(scope: SerializedScope): boolean {
  return controlSubgraphKindOf(scope) !== null;
}
