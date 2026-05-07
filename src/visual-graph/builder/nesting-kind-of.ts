import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import {
  NESTING_KIND,
  type NestingKind,
} from "../../serializer/nesting-kind.js";

export function nestingKindOf(scope: SerializedScope): NestingKind | null {
  if (scope.type === SCOPE_TYPE.Function) {
    return scope.functionExpressionScope ? null : NESTING_KIND.Function;
  }
  if (scope.type === SCOPE_TYPE.For) {
    return NESTING_KIND.For;
  }
  if (scope.type === SCOPE_TYPE.Switch) {
    return NESTING_KIND.Switch;
  }
  if (scope.type === SCOPE_TYPE.Catch) {
    return NESTING_KIND.TryCatchFinally;
  }
  if (scope.type === SCOPE_TYPE.Block) {
    const ctx = scope.blockContext;
    if (!ctx) {
      return NESTING_KIND.Block;
    }
    if (ctx.parentType === AST_TYPE.IfStatement) {
      return NESTING_KIND.If;
    }
    if (
      ctx.parentType === AST_TYPE.ForStatement ||
      ctx.parentType === AST_TYPE.ForInStatement ||
      ctx.parentType === AST_TYPE.ForOfStatement
    ) {
      return NESTING_KIND.For;
    }
    if (
      ctx.parentType === AST_TYPE.WhileStatement ||
      ctx.parentType === AST_TYPE.DoWhileStatement
    ) {
      return NESTING_KIND.While;
    }
    if (
      ctx.parentType === AST_TYPE.TryStatement ||
      ctx.parentType === AST_TYPE.CatchClause
    ) {
      return NESTING_KIND.TryCatchFinally;
    }
    if (ctx.parentType === AST_TYPE.SwitchStatement) {
      return NESTING_KIND.Switch;
    }
    return NESTING_KIND.Block;
  }
  return null;
}
