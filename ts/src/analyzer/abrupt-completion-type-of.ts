import {
  break$,
  continue$,
  return$,
  throw$,
} from "../ir/completion/completion-type.js";
import type { AbruptCompletion } from "../ir/completion/completion.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

/**
 * For a Statement node, return the set of ECMA §6.2.4 abrupt
 * completion `[[Type]]` values reachable at its termination, or
 * `null` if any path through the statement can fall through to a
 * normal completion.
 *
 * Covers all four abrupt types: return, throw, break, continue. The
 * caller decides which subset matters for its question (e.g.
 * `caseExitsFunction` filters to return / throw only).
 *
 * `LabeledStatement` is not handled here: a labelled wrapper around
 * a return / throw / break / continue routes through the switch's
 * `default` branch and returns `null`. ECMA §14.13.4 LabelledStatement
 * Runtime Semantics says the wrapper inherits its body's completion
 * (with label-matching break / continue collapsed to normal), but
 * that requires tracking enclosing labels — a separate design
 * decision left to a sister issue.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://tc39.es/ecma262/#sec-labelled-statements ECMA §14.13 Labelled Statements
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 * @see https://github.com/crescware/unsnarl/issues/97 Issue #97 (LabeledStatement transparent handling)
 */
export function abruptCompletionTypeOf(
  node: AstNode,
): ReadonlySet<AbruptCompletion["type"]> | null {
  switch (node.type) {
    case AST_TYPE.ReturnStatement:
      return new Set<AbruptCompletion["type"]>([return$.literal]);
    case AST_TYPE.ThrowStatement:
      return new Set<AbruptCompletion["type"]>([throw$.literal]);
    case AST_TYPE.BreakStatement:
      return new Set<AbruptCompletion["type"]>([break$.literal]);
    case AST_TYPE.ContinueStatement:
      return new Set<AbruptCompletion["type"]>([continue$.literal]);
    case AST_TYPE.BlockStatement: {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isAstNode(last)) {
          return abruptCompletionTypeOf(last);
        }
      }
      return null;
    }
    case AST_TYPE.IfStatement: {
      const consequent = node["consequent"];
      const alternate = node["alternate"];
      if (isAstNode(consequent) && isAstNode(alternate)) {
        const consequentTypes = abruptCompletionTypeOf(consequent);
        const alternateTypes = abruptCompletionTypeOf(alternate);
        if (consequentTypes === null || alternateTypes === null) {
          return null;
        }
        return new Set<AbruptCompletion["type"]>([
          ...consequentTypes,
          ...alternateTypes,
        ]);
      }
      return null;
    }
    default:
      return null;
  }
}
