import type { SerializedScope } from "../../ir/model.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";

export function isControlSubgraph(scope: SerializedScope): boolean {
  return controlSubgraphKindOf(scope) !== null;
}
