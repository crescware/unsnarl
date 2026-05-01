import type { BlockContext } from "../../../ir/model.js";
import { AST_TYPE } from "../../../parser/ast-type.js";

export function baseBlockContext(): BlockContext {
  return {
    parentType: AST_TYPE.IfStatement,
    key: "consequent",
    parentSpanOffset: 0,
    caseTest: null,
  };
}
