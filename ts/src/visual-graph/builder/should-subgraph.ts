import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { isClassSubgraph } from "./is-class-subgraph.js";
import { isControlSubgraph } from "./is-control-subgraph.js";
import { isFunctionSubgraph } from "./is-function-subgraph.js";

export function shouldSubgraph(scope: SerializedScope): boolean {
  return (
    isFunctionSubgraph(scope) ||
    isClassSubgraph(scope) ||
    isControlSubgraph(scope)
  );
}
