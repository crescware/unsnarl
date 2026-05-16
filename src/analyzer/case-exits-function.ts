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
  // Contract: a non-null return from abruptCompletionTypeOf is a non-empty
  // type set. An empty set would let the trailing loop silently classify
  // the case as "exits", which is the wrong default for a contract
  // violation. Surface analyzer regressions loudly instead.
  if (abruptTypes.size === 0) {
    throw new Error(
      "caseExitsFunction: abruptCompletionTypeOf returned an empty type set",
    );
  }
  for (const abruptType of abruptTypes) {
    if (abruptType !== return$.literal && abruptType !== throw$.literal) {
      return false;
    }
  }
  return true;
}
