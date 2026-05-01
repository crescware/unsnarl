import type { AstExpression, ReferenceFlagBits } from "../../ir/model.js";

export type ClassifyResult =
  | { kind: "binding" }
  | { kind: "skip" }
  | {
      kind: "reference";
      flags: ReferenceFlagBits;
      init: boolean;
      writeExpr: AstExpression | null;
    };
