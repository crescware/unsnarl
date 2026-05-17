import { type InferOutput, parse } from "valibot";

import { caseClause$, other$ } from "../../../ir/scope/block-context-kind.js";
import {
  caseClauseBlockContext$,
  otherBlockContext$,
} from "../../../ir/scope/block-context.js";
import { AST_TYPE } from "../../../parser/ast-type.js";

export function baseBlockContext(): InferOutput<typeof otherBlockContext$> {
  return parse(otherBlockContext$, {
    kind: other$.literal,
    parentType: AST_TYPE.IfStatement,
    key: "consequent",
    parentSpanOffset: 0,
    ifChainRootOffset: null,
  });
}

export function baseCaseClauseBlockContext(): InferOutput<
  typeof caseClauseBlockContext$
> {
  return parse(caseClauseBlockContext$, {
    kind: caseClause$.literal,
    parentType: AST_TYPE.SwitchStatement,
    key: "cases",
    parentSpanOffset: 0,
    caseTest: null,
  });
}
