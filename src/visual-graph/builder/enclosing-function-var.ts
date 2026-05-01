import type { SerializedScope } from "../../ir/model.js";
import { findEnclosingSubgraphScope } from "./find-enclosing-subgraph-scope.js";

export function enclosingFunctionVar(
  scopeId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
  subgraphOwnerVar: ReadonlyMap<string, string>,
): string | null {
  const fnScopeId = findEnclosingSubgraphScope(
    scopeId,
    scopeMap,
    subgraphOwnerVar,
  );
  if (fnScopeId === null) {
    return null;
  }
  return subgraphOwnerVar.get(fnScopeId) ?? null;
}
