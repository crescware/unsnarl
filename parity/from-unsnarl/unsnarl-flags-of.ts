import type { Reference as UnsnarlReference } from "../../src/ir/reference/reference.js";
import type { NormalizedReferenceFlags } from "../normalized/normalized-reference-flags.js";

export function unsnarlFlagsOf(
  ref: UnsnarlReference,
): NormalizedReferenceFlags {
  // call/receiver are unsnarl-only concepts that the eslint-scope side
  // stubs to false; the parity comparator only diffs read/write, so we
  // mirror the eslint-scope stub here to keep the normalized shape uniform.
  return {
    read: ref.isRead(),
    write: ref.isWrite(),
    call: false,
    receiver: false,
  };
}
