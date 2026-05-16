import { return$, throw$ } from "../ir/completion/completion-type.js";
import { abruptCompletionTypeOf } from "./abrupt-completion-type-of.js";
import { isAstNode } from "./is-ast-node.js";

export function caseExitsFunction(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return false;
  }
  const last = consequent[consequent.length - 1];
  if (!isAstNode(last)) {
    return false;
  }
  const abruptTypes = abruptCompletionTypeOf(last);
  if (abruptTypes === null) {
    return false;
  }
  for (const abruptType of abruptTypes) {
    if (abruptType !== return$.literal && abruptType !== throw$.literal) {
      return false;
    }
  }
  return true;
}
