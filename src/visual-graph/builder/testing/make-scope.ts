import { AST_TYPE, SCOPE_TYPE } from "../../../constants.js";
import type { SerializedScope } from "../../../ir/model.js";
import { span } from "./span.js";

export function makeScope(
  overrides: Partial<SerializedScope> = {},
): SerializedScope {
  const offset = overrides.block?.span?.offset ?? 0;
  return {
    id: "s",
    type: SCOPE_TYPE.Block,
    isStrict: false,
    upper: null,
    childScopes: [],
    variableScope: "s",
    block: {
      type: AST_TYPE.BlockStatement,
      span: span(offset),
      endSpan: span(offset + 10, 1, offset + 10),
    },
    variables: [],
    references: [],
    through: [],
    functionExpressionScope: false,
    blockContext: null,
    fallsThrough: false,
    exitsFunction: false,
    ...overrides,
  };
}
