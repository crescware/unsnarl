import type { Scope } from "eslint-scope";

import { compareRange } from "../normalized/compare-range.js";
import type { NormalizedScope } from "../normalized/normalized-scope.js";
import { nodeTypeOf } from "../util/node-type-of.js";
import { rangeOf } from "../util/range-of.js";
import type { EslintScopePathMap } from "./build-eslint-scope-path-map.js";
import { normalizeEslintScopeReference } from "./normalize-eslint-scope-reference.js";
import { normalizeEslintScopeVariable } from "./normalize-eslint-scope-variable.js";
import { sortEslintScopeChildScopes } from "./sort-eslint-scope-child-scopes.js";

export function normalizeEslintScopeScope(
  scope: Scope,
  path: readonly number[],
  paths: EslintScopePathMap,
): NormalizedScope {
  const variables = scope.variables
    .map(normalizeEslintScopeVariable)
    .sort((a, b) => a.name.localeCompare(b.name));
  const references = scope.references
    .map((v) => normalizeEslintScopeReference(v, paths))
    .sort(
      (a, b) =>
        compareRange(a.identifierRange, b.identifierRange) ||
        Number(a.init) - Number(b.init),
    );
  const sortedChildren = sortEslintScopeChildScopes(scope.childScopes);
  const childScopes = sortedChildren.map((child, i) =>
    normalizeEslintScopeScope(child, [...path, i], paths),
  );
  return {
    type: scope.type,
    isStrict: scope.isStrict,
    path,
    blockType: nodeTypeOf(scope.block as { type?: string }),
    blockRange: rangeOf(scope.block as { start?: number; end?: number }),
    variables,
    references,
    childScopes,
  };
}
