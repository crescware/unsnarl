import type { Scope } from "eslint-scope";

import { sortEslintScopeChildScopes } from "./sort-eslint-scope-child-scopes.js";

export type EslintScopePathMap = ReadonlyMap<Scope, readonly number[]>;

export function buildEslintScopePathMap(root: Scope): EslintScopePathMap {
  const map = new Map<Scope, readonly number[]>();
  visit(root, []);
  return map;

  function visit(scope: Scope, path: readonly number[]): void {
    map.set(scope, path);
    const sorted = sortEslintScopeChildScopes(scope.childScopes);
    sorted.forEach((child, i) => visit(child, [...path, i]));
  }
}
