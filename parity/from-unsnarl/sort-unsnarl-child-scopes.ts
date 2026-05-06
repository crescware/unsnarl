import type { Scope as UnsnarlScope } from "../../src/ir/scope/scope.js";
import { compareRange } from "../normalized/compare-range.js";
import { rangeOf } from "../util/range-of.js";

export function sortUnsnarlChildScopes(
  scopes: readonly UnsnarlScope[],
): UnsnarlScope[] {
  return [...scopes].sort((a, b) => {
    const r = compareRange(rangeOf(a.block), rangeOf(b.block));
    if (r !== 0) {
      return r;
    }
    return a.type.localeCompare(b.type);
  });
}
