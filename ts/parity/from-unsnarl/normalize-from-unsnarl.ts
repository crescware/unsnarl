import type { Scope as UnsnarlScope } from "../../src/ir/scope/scope.js";
import type { NormalizedScope } from "../normalized/normalized-scope.js";
import { buildUnsnarlPathMap } from "./build-unsnarl-path-map.js";
import { normalizeUnsnarlScope } from "./normalize-unsnarl-scope.js";

export function normalizeFromUnsnarl(root: UnsnarlScope): NormalizedScope {
  const paths = buildUnsnarlPathMap(root);
  return normalizeUnsnarlScope(root, [], paths);
}
