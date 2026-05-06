import type { NormalizedReferenceFlags } from "./normalized-reference-flags.js";
import type { NormalizedResolution } from "./normalized-resolution.js";
import type { Range } from "./range.js";

export type NormalizedReference = Readonly<{
  identifierRange: Range;
  init: boolean;
  flags: NormalizedReferenceFlags;
  resolved: NormalizedResolution | null;
}>;
