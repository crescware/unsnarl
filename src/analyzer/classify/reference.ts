import type { ReferenceFlagBits } from "../../ir/model.js";
import type { ClassifyResult } from "./classify-result.js";

export function reference(
  flags: ReferenceFlagBits,
  init: boolean,
): ClassifyResult {
  return { kind: "reference", flags, init };
}
