import type { Reference } from "eslint-scope";

import type { NormalizedReference } from "../normalized/normalized-reference.js";
import { rangeOf } from "../util/range-of.js";
import type { EslintScopePathMap } from "./build-eslint-scope-path-map.js";
import { eslintScopeFlagsOf } from "./eslint-scope-flags-of.js";
import { eslintScopeResolutionOf } from "./eslint-scope-resolution-of.js";

export function normalizeEslintScopeReference(
  ref: Reference,
  paths: EslintScopePathMap,
): NormalizedReference {
  const initRaw = (ref as Reference & { init?: boolean }).init;
  return {
    identifierRange: rangeOf(
      ref.identifier as { start?: number; end?: number },
    ),
    init: initRaw === true,
    flags: eslintScopeFlagsOf(ref),
    resolved: eslintScopeResolutionOf(ref, paths),
  };
}
