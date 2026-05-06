import type { Reference as UnsnarlReference } from "../../src/ir/reference/reference.js";
import type { NormalizedResolution } from "../normalized/normalized-resolution.js";
import type { UnsnarlPathMap } from "./build-unsnarl-path-map.js";

export function unsnarlResolutionOf(
  ref: UnsnarlReference,
  paths: UnsnarlPathMap,
): NormalizedResolution | null {
  if (ref.resolved === null) {
    return null;
  }
  const scopePath = paths.get(ref.resolved.scope);
  if (scopePath === undefined) {
    throw new Error(
      `Unsnarl normalizer: resolved variable's scope is unknown for ref to ${ref.resolved.name}`,
    );
  }
  return { scopePath, varName: ref.resolved.name };
}
