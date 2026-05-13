import type { BlockContext } from "../../../ir/scope/block-context.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { asFilledString } from "../../../util/filled-string.js";

export function baseBlockContext(): BlockContext {
  return {
    kind: "other",
    parentType: AST_TYPE.IfStatement,
    key: asFilledString("consequent"),
    parentSpanOffset: 0,
    ifChainRootOffset: null,
  };
}

export function baseCaseClauseBlockContext(): BlockContext {
  return {
    kind: "case-clause",
    parentType: AST_TYPE.SwitchStatement,
    key: asFilledString("cases"),
    parentSpanOffset: 0,
    caseTest: null,
  };
}
