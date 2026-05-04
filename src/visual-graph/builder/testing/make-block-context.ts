import type { BlockContext } from "../../../ir/scope/block-context.js";
import { AST_TYPE } from "../../../parser/ast-type.js";

export function baseBlockContext(): BlockContext {
  return {
    kind: "other",
    parentType: AST_TYPE.IfStatement,
    key: "consequent",
    parentSpanOffset: 0,
  };
}

export function baseCaseClauseBlockContext(): BlockContext {
  return {
    kind: "case-clause",
    parentType: AST_TYPE.SwitchStatement,
    key: "cases",
    parentSpanOffset: 0,
    caseTest: null,
  };
}
