import type { ReferenceFlagBits } from "../../ir/reference/reference-flags.js";

export type ClassifyResult =
  | { kind: "binding" }
  | { kind: "skip" }
  | {
      kind: "reference";
      flags: ReferenceFlagBits;
      init: boolean;
    };
