import { caseClause$, other$ } from "../../../ir/scope/block-context-kind.js";
import type { BlockContext } from "../../../ir/scope/block-context.js";
import { AST_TYPE } from "../../../parser/ast-type.js";
import { asFilledString } from "../../../util/filled-string.js";

export function baseBlockContext(): BlockContext {
  return {
    kind: other$.literal,
    parentType: AST_TYPE.IfStatement,
    key: asFilledString("consequent"),
    parentSpanOffset: 0,
    ifChainRootOffset: null,
  };
}

export function baseCaseClauseBlockContext(): BlockContext {
  return {
    kind: caseClause$.literal,
    parentType: AST_TYPE.SwitchStatement,
    key: asFilledString("cases"),
    parentSpanOffset: 0,
    caseTest: null,
  };
}
