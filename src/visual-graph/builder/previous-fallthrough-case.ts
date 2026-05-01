import type { SerializedScope } from "../../ir/model.js";
import { branchContainerKey } from "./branch-container-key.js";

export function previousFallthroughCase(
  caseScope: SerializedScope,
  sortedCasesByContainer: ReadonlyMap<string, SerializedScope[]>,
): SerializedScope | null {
  const ckey = branchContainerKey(caseScope);
  if (!ckey) {
    return null;
  }
  const cases = sortedCasesByContainer.get(ckey);
  if (!cases) {
    return null;
  }
  const idx = cases.indexOf(caseScope);
  if (idx <= 0) {
    return null;
  }
  const prev = cases[idx - 1];
  if (!prev) {
    return null;
  }
  return prev.fallsThrough ? prev : null;
}
