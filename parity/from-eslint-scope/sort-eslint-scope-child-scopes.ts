import type { Scope } from "eslint-scope";

import { compareRange } from "../normalized/compare-range.js";
import { rangeOf } from "../util/range-of.js";

export function sortEslintScopeChildScopes(scopes: readonly Scope[]): Scope[] {
  return [...scopes].sort((a, b) => {
    const r = compareRange(
      rangeOf(a.block as { start?: number; end?: number }),
      rangeOf(b.block as { start?: number; end?: number }),
    );
    if (r !== 0) {
      return r;
    }
    return a.type.localeCompare(b.type);
  });
}
