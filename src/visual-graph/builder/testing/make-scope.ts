import { parse } from "valibot";

import { SCOPE_TYPE } from "../../../analyzer/scope-type.js";
import {
  serializedScope$,
  type SerializedScope,
} from "../../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { NESTING_KIND } from "../../../serializer/nesting-kind.js";
import { span } from "./span.js";

export function baseScope(): SerializedScope {
  return parse(serializedScope$, {
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
    nestingDepths: {
      [NESTING_KIND.Function]: 0,
      [NESTING_KIND.If]: 0,
      [NESTING_KIND.For]: 0,
      [NESTING_KIND.While]: 0,
      [NESTING_KIND.Switch]: 0,
      [NESTING_KIND.TryCatchFinally]: 0,
      [NESTING_KIND.Block]: 0,
    },
  });
}
