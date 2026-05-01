import { SCOPE_TYPE } from "../../../analyzer/scope-type.js";
import type { SerializedScope } from "../../../ir/model.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
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
  };
}
