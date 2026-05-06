import type { Reference as UnsnarlReference } from "../../src/ir/reference/reference.js";
import type { NormalizedReferenceFlags } from "../normalized/normalized-reference-flags.js";

export function unsnarlFlagsOf(
  ref: UnsnarlReference,
): NormalizedReferenceFlags {
  return {
    read: ref.isRead(),
    write: ref.isWrite(),
    call: ref.isCall(),
    receiver: ref.isReceiver(),
  };
}
