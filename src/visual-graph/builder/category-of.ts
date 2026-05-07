import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { CATEGORY, type Category } from "../../serializer/category.js";

export function categoryOf(scope: SerializedScope): Category | null {
  if (scope.type === SCOPE_TYPE.Function) {
    return scope.functionExpressionScope ? null : CATEGORY.Function;
  }
  if (scope.type === SCOPE_TYPE.For) {
    return CATEGORY.For;
  }
  if (scope.type === SCOPE_TYPE.Switch) {
    return CATEGORY.Switch;
  }
  if (scope.type === SCOPE_TYPE.Catch) {
    return CATEGORY.TryCatchFinally;
  }
  if (scope.type === SCOPE_TYPE.Block) {
    const ctx = scope.blockContext;
    if (!ctx) {
      return CATEGORY.Block;
    }
    if (ctx.parentType === AST_TYPE.IfStatement) {
      return CATEGORY.If;
    }
    if (
      ctx.parentType === AST_TYPE.ForStatement ||
      ctx.parentType === AST_TYPE.ForInStatement ||
      ctx.parentType === AST_TYPE.ForOfStatement
    ) {
      return CATEGORY.For;
    }
    if (
      ctx.parentType === AST_TYPE.WhileStatement ||
      ctx.parentType === AST_TYPE.DoWhileStatement
    ) {
      return CATEGORY.While;
    }
    if (
      ctx.parentType === AST_TYPE.TryStatement ||
      ctx.parentType === AST_TYPE.CatchClause
    ) {
      return CATEGORY.TryCatchFinally;
    }
    if (ctx.parentType === AST_TYPE.SwitchStatement) {
      return CATEGORY.Switch;
    }
    return CATEGORY.Block;
  }
  return null;
}
