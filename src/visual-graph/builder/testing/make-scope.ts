import { SCOPE_TYPE } from "../../../analyzer/scope-type.js";
import type { SerializedScope } from "../../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { CATEGORY } from "../../../serializer/category.js";
import { span } from "./span.js";

export function baseScope(): SerializedScope {
  return {
    id: "s",
    type: SCOPE_TYPE.Block,
    isStrict: false,
    upper: null,
    childScopes: [],
    variableScope: "s",
    block: {
      type: AST_TYPE.BlockStatement,
      span: span(0),
      endSpan: span(10, 1, 10),
    },
    variables: [],
    references: [],
    through: [],
    functionExpressionScope: false,
    blockContext: null,
    fallsThrough: false,
    exitsFunction: false,
    categoryDepths: {
      [CATEGORY.Function]: 0,
      [CATEGORY.If]: 0,
      [CATEGORY.For]: 0,
      [CATEGORY.While]: 0,
      [CATEGORY.Switch]: 0,
      [CATEGORY.TryCatchFinally]: 0,
      [CATEGORY.Block]: 0,
    },
  };
}
