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
    const root =
      ctx.kind === "other" && ctx.ifChainRootOffset !== undefined
        ? ctx.ifChainRootOffset
        : ctx.parentSpanOffset;
    return `if:${scope.upper ?? ""}:${root}`;
  }
  if (
    ctx.parentType === AST_TYPE.TryStatement &&
    (ctx.key === "block" || ctx.key === "handler")
  ) {
    return `try:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
  }
  return null;
}
