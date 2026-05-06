import type { Reference as UnsnarlReference } from "../../src/ir/reference/reference.js";
import type { NormalizedReference } from "../normalized/normalized-reference.js";
import { rangeOf } from "../util/range-of.js";
import type { UnsnarlPathMap } from "./build-unsnarl-path-map.js";
import { unsnarlFlagsOf } from "./unsnarl-flags-of.js";
import { unsnarlResolutionOf } from "./unsnarl-resolution-of.js";

export function normalizeUnsnarlReference(
  ref: UnsnarlReference,
  paths: UnsnarlPathMap,
): NormalizedReference {
  return {
    identifierRange: rangeOf(ref.identifier),
    init: ref.init,
    flags: unsnarlFlagsOf(ref),
    resolved: unsnarlResolutionOf(ref, paths),
  };
}
