import type { Reference } from "eslint-scope";

import type { NormalizedResolution } from "../normalized/normalized-resolution.js";
import type { EslintScopePathMap } from "./build-eslint-scope-path-map.js";

export function eslintScopeResolutionOf(
  ref: Reference,
  paths: EslintScopePathMap,
): NormalizedResolution | null {
  if (ref.resolved === null) {
    return null;
  }
  const scopePath = paths.get(ref.resolved.scope);
  if (scopePath === undefined) {
    return null;
  }
  return { scopePath, varName: ref.resolved.name };
}
