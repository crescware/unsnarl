import type { Reference } from "eslint-scope";

import type { NormalizedReferenceFlags } from "../normalized/normalized-reference-flags.js";

export function eslintScopeFlagsOf(ref: Reference): NormalizedReferenceFlags {
  return {
    read: ref.isRead(),
    write: ref.isWrite(),
    call: false,
    receiver: false,
  };
}
