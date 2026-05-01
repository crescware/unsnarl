import type { AstExpression, ReferenceFlagBits } from "../../ir/model.js";
import type { ClassifyResult } from "./classify-result.js";

export function reference(
  flags: ReferenceFlagBits,
  init: boolean,
  writeExpr: AstExpression | null,
): ClassifyResult {
  return { kind: "reference", flags, init, writeExpr };
}
