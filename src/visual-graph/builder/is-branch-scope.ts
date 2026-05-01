import type { SerializedScope } from "../../ir/model.js";
import { branchContainerKey } from "./branch-container-key.js";

export function isBranchScope(
  scopeId: string,
  scopeMap: ReadonlyMap<string, SerializedScope>,
): boolean {
  const scope = scopeMap.get(scopeId);
  return scope ? branchContainerKey(scope) !== null : false;
}
