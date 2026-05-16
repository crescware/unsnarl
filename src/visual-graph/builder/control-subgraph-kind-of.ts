import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import type { VisualSubgraph } from "../visual-subgraph.js";

export function controlSubgraphKindOf(
  scope: SerializedScope,
): VisualSubgraph["kind"] | null {
  if (scope.type === SCOPE_TYPE.Catch) {
    return SUBGRAPH_KIND.Catch;
  }
  if (scope.type === SCOPE_TYPE.For) {
    return SUBGRAPH_KIND.For;
  }
  if (scope.type === SCOPE_TYPE.Switch) {
    return SUBGRAPH_KIND.Switch;
  }
  if (scope.type === SCOPE_TYPE.Block) {
    const ctx = scope.blockContext;
    if (ctx) {
      if (ctx.parentType === AST_TYPE.TryStatement) {
        if (ctx.key === "block") {
          return SUBGRAPH_KIND.Try;
        }
        if (ctx.key === "finalizer") {
          return SUBGRAPH_KIND.Finally;
        }
      }
      if (ctx.parentType === AST_TYPE.IfStatement) {
        if (ctx.key === "consequent") {
          return SUBGRAPH_KIND.If;
        }
        if (ctx.key === "alternate") {
          return SUBGRAPH_KIND.Else;
        }
      }
      if (ctx.parentType === AST_TYPE.SwitchStatement && ctx.key === "cases") {
        return SUBGRAPH_KIND.Case;
      }
      if (ctx.parentType === AST_TYPE.WhileStatement && ctx.key === "body") {
        return SUBGRAPH_KIND.While;
      }
      if (ctx.parentType === AST_TYPE.DoWhileStatement && ctx.key === "body") {
        return SUBGRAPH_KIND.DoWhile;
      }
    }
    // A Block scope that is not the body of any of the recognised
    // control statements -- a bare `{ ... }`, the body of a for-loop
    // (whose ForStatement scope already renders as the "for" subgraph,
    // but its descendants land here when the body has its own lexical
    // declarations), or any other ECMA-262 14.2 Block. Render it as a
    // generic block subgraph so the rendering still mirrors the
    // source-level nesting.
    return SUBGRAPH_KIND.Block;
  }
  return null;
}
