import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";

export function isClassSubgraph(scope: SerializedScope): boolean {
  return scope.type === SCOPE_TYPE.Class;
}
