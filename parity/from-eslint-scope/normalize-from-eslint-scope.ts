import type { Scope } from "eslint-scope";

import type { NormalizedScope } from "../normalized/normalized-scope.js";
import { buildEslintScopePathMap } from "./build-eslint-scope-path-map.js";
import { normalizeEslintScopeScope } from "./normalize-eslint-scope-scope.js";

export function normalizeFromEslintScope(root: Scope): NormalizedScope {
  const paths = buildEslintScopePathMap(root);
  return normalizeEslintScopeScope(root, [], paths);
}
