import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";

export function isFunctionSubgraph(scope: SerializedScope): boolean {
  return scope.type === SCOPE_TYPE.Function && !scope.functionExpressionScope;
}
