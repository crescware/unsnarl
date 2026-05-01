import type { SerializedScope } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";

export function branchContainerKey(scope: SerializedScope): string | null {
  const ctx = scope.blockContext;
  if (!ctx) {
    return null;
  }
  if (ctx.parentType === AST_TYPE.SwitchStatement && ctx.key === "cases") {
    return `switch:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
  }
  if (
    ctx.parentType === AST_TYPE.IfStatement &&
    (ctx.key === "consequent" || ctx.key === "alternate")
  ) {
    return `if:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
  }
  return null;
}
