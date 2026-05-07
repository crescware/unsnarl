import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";

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
    if (ctx) {
      if (ctx.parentType === AST_TYPE.TryStatement) {
        if (ctx.key === "block") {
          return "try";
        }
        if (ctx.key === "finalizer") {
          return "finally";
        }
      }
      if (ctx.parentType === AST_TYPE.IfStatement) {
        if (ctx.key === "consequent") {
          return "if";
        }
        if (ctx.key === "alternate") {
          return "else";
        }
      }
      if (ctx.parentType === AST_TYPE.SwitchStatement && ctx.key === "cases") {
        return "case";
      }
      if (ctx.parentType === AST_TYPE.WhileStatement && ctx.key === "body") {
        return "while";
      }
      if (ctx.parentType === AST_TYPE.DoWhileStatement && ctx.key === "body") {
        return "do-while";
      }
    }
    // A Block scope that is not the body of any of the recognised
    // control statements -- a bare `{ ... }`, the body of a for-loop
    // (whose ForStatement scope already renders as the "for" subgraph,
    // but its descendants land here when the body has its own lexical
    // declarations), or any other ECMA-262 14.2 Block. Render it as a
    // generic "block" subgraph so the rendering still mirrors the
    // source-level nesting.
    return "block";
  }
  return null;
}
