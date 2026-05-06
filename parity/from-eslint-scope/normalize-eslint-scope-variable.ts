import type { Variable } from "eslint-scope";

import { compareRange } from "../normalized/compare-range.js";
import type { NormalizedVariable } from "../normalized/normalized-variable.js";
import { normalizeEslintScopeDefinition } from "./normalize-eslint-scope-definition.js";

export function normalizeEslintScopeVariable(v: Variable): NormalizedVariable {
  const defs = v.defs
    .map(normalizeEslintScopeDefinition)
    .sort((a, b) => compareRange(a.nodeRange, b.nodeRange));
  return {
    name: v.name,
    defs,
    referenceCount: v.references.length,
  };
}
