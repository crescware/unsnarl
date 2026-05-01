import { SCOPE_TYPE } from "../../constants.js";
import type { SerializedScope } from "../../ir/model.js";
import type { VisualSubgraph } from "../model.js";

export function controlSubgraphKindOf(
  scope: SerializedScope,
): VisualSubgraph["kind"] | null {
  if (scope.type === SCOPE_TYPE.Catch) {
    return "catch";
  }
  if (scope.type === SCOPE_TYPE.For) {
    return "for";
  }
  if (scope.type === SCOPE_TYPE.Switch) {
    return "switch";
  }
  if (scope.type === SCOPE_TYPE.Block) {
    const ctx = scope.blockContext;
    if (!ctx) {
      return null;
    }
    if (ctx.parentType === "TryStatement") {
      if (ctx.key === "block") {
        return "try";
      }
      if (ctx.key === "finalizer") {
        return "finally";
      }
    }
    if (ctx.parentType === "IfStatement") {
      if (ctx.key === "consequent") {
        return "if";
      }
      if (ctx.key === "alternate") {
        return "else";
      }
    }
    if (ctx.parentType === "SwitchStatement" && ctx.key === "cases") {
      return "case";
    }
  }
  return null;
}
