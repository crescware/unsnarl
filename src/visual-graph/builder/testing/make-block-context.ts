import { parse } from "valibot";

import { caseClause$, other$ } from "../../../ir/scope/block-context-kind.js";
import {
  blockContext$,
  type BlockContext,
} from "../../../ir/scope/block-context.js";
import { AST_TYPE } from "../../../parser/ast-type.js";

export function baseBlockContext(): BlockContext {
  return parse(blockContext$, {
    kind: other$.literal,
    parentType: AST_TYPE.IfStatement,
    key: "consequent",
    parentSpanOffset: 0,
    ifChainRootOffset: null,
  });
}

export function baseCaseClauseBlockContext(): BlockContext {
  return parse(blockContext$, {
    kind: caseClause$.literal,
    parentType: AST_TYPE.SwitchStatement,
    key: "cases",
    parentSpanOffset: 0,
    caseTest: null,
  });
}
