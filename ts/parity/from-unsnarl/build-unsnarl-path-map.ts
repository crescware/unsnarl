import type { Scope as UnsnarlScope } from "../../src/ir/scope/scope.js";
import { sortUnsnarlChildScopes } from "./sort-unsnarl-child-scopes.js";

export type UnsnarlPathMap = ReadonlyMap<UnsnarlScope, readonly number[]>;

export function buildUnsnarlPathMap(root: UnsnarlScope): UnsnarlPathMap {
  const map = new Map<UnsnarlScope, readonly number[]>();
  visit(root, []);
  return map;

  function visit(scope: UnsnarlScope, path: readonly number[]): void {
    map.set(scope, path);
    const sorted = sortUnsnarlChildScopes(scope.childScopes);
    sorted.forEach((child, i) => visit(child, [...path, i]));
  }
}
