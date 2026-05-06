import type { Scope as UnsnarlScope } from "../../src/ir/scope/scope.js";
import { compareRange } from "../normalized/compare-range.js";
import type { NormalizedScope } from "../normalized/normalized-scope.js";
import { nodeTypeOf } from "../util/node-type-of.js";
import { rangeOf } from "../util/range-of.js";
import type { UnsnarlPathMap } from "./build-unsnarl-path-map.js";
import { normalizeUnsnarlReference } from "./normalize-unsnarl-reference.js";
import { normalizeUnsnarlVariable } from "./normalize-unsnarl-variable.js";
import { sortUnsnarlChildScopes } from "./sort-unsnarl-child-scopes.js";

export function normalizeUnsnarlScope(
  scope: UnsnarlScope,
  path: readonly number[],
  paths: UnsnarlPathMap,
): NormalizedScope {
  const variables = scope.variables
    .map(normalizeUnsnarlVariable)
    .sort((a, b) => a.name.localeCompare(b.name));
  const references = scope.references
    .map((r) => normalizeUnsnarlReference(r, paths))
    .sort(
      (a, b) =>
        compareRange(a.identifierRange, b.identifierRange) ||
        Number(a.init) - Number(b.init),
    );
  const sortedChildren = sortUnsnarlChildScopes(scope.childScopes);
  const childScopes = sortedChildren.map((child, i) =>
    normalizeUnsnarlScope(child, [...path, i], paths),
  );
  return {
    type: scope.type,
    isStrict: scope.isStrict,
    path,
    blockType: nodeTypeOf(scope.block),
    blockRange: rangeOf(scope.block),
    variables,
    references,
    childScopes,
  };
}
