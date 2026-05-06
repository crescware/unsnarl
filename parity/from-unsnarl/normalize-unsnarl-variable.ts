import type { Variable as UnsnarlVariable } from "../../src/ir/scope/variable.js";
import { compareRange } from "../normalized/compare-range.js";
import type { NormalizedVariable } from "../normalized/normalized-variable.js";
import { normalizeUnsnarlDefinition } from "./normalize-unsnarl-definition.js";

export function normalizeUnsnarlVariable(
  v: UnsnarlVariable,
): NormalizedVariable {
  const defs = v.defs
    .map(normalizeUnsnarlDefinition)
    .sort((a, b) => compareRange(a.nodeRange, b.nodeRange));
  return {
    name: v.name,
    defs,
    referenceCount: v.references.length,
  };
}
