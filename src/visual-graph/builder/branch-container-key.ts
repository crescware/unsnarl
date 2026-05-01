import type { SerializedScope } from "../../ir/model.js";

export function branchContainerKey(scope: SerializedScope): string | null {
  const ctx = scope.blockContext;
  if (!ctx) {
    return null;
  }
  if (ctx.parentType === "SwitchStatement" && ctx.key === "cases") {
    return `switch:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
  }
  if (
    ctx.parentType === "IfStatement" &&
    (ctx.key === "consequent" || ctx.key === "alternate")
  ) {
    return `if:${scope.upper ?? ""}:${ctx.parentSpanOffset}`;
  }
  return null;
}
