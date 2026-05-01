import type { AstExpression } from "../../ir/model.js";

export function isAstExpression(value: unknown): value is AstExpression {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
