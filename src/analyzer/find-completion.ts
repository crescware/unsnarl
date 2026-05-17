import type { InferOutput } from "valibot";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import { normal$, return$, throw$ } from "../ir/completion/completion-type.js";
import type { ReferenceCompletion } from "../ir/reference/reference-completion.js";
import { AST_TYPE } from "../parser/ast-type.js";

/**
 * Walk a Reference's ancestor chain from leaf to root and resolve the
 * Completion category that carries its value, returning a Completion
 * for every Reference (the result is total over the input path).
 *
 * Function-shaped nodes (`FunctionDeclaration`, `FunctionExpression`,
 * `ArrowFunctionExpression`) and class-shaped nodes (`ClassDeclaration`,
 * `ClassExpression`) act as boundaries: anything reached through one of
 * them belongs to that inner construct, not to the enclosing function
 * whose `[[Value]]` we are classifying for. The class boundary covers
 * field initializers (synthetic per-instance constructor), static
 * blocks (class-definition-time enclosing context), computed property
 * keys, extends clauses, and member decorators -- none of which
 * contribute to the enclosing function's Completion Record.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 * @see https://github.com/crescware/unsnarl/issues/98 Issue #98
 */
export function findCompletion(
  path: readonly PathEntry[],
): ReferenceCompletion {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    if (type === AST_TYPE.ReturnStatement) {
      return completionFromNode(return$.literal, entry.node);
    }
    if (type === AST_TYPE.ThrowStatement) {
      return completionFromNode(throw$.literal, entry.node);
    }
    if (type === AST_TYPE.ArrowFunctionExpression) {
      // Block-body arrows defer to an inner ReturnStatement (already handled
      // by the deeper path entry). Expression-body arrows have no explicit
      // return: the body expression itself is the implicit return target.
      const body = (
        entry.node as unknown as { body?: { type?: string } | null }
      ).body;
      if (body && body.type !== AST_TYPE.BlockStatement) {
        return completionFromNode(
          return$.literal,
          body as { start?: number; end?: number },
        );
      }
      return NORMAL_COMPLETION;
    }
    if (
      type === AST_TYPE.FunctionExpression ||
      type === AST_TYPE.FunctionDeclaration
    ) {
      // A nested function boundary cuts off the search: a return/throw inside
      // an inner function belongs to that inner function, not to the outer one.
      return NORMAL_COMPLETION;
    }
    if (
      type === AST_TYPE.ClassExpression ||
      type === AST_TYPE.ClassDeclaration
    ) {
      // Class internals (field initializers, static blocks, computed keys,
      // extends clauses, member decorators) never flow into the enclosing
      // function's Completion `[[Value]]`. Member methods / getters / setters /
      // constructors are wrapped in their own FunctionExpression and are
      // already handled by the function boundary above; stopping at the class
      // here catches everything else inside the class body.
      return NORMAL_COMPLETION;
    }
  }
  return NORMAL_COMPLETION;
}

const NORMAL_COMPLETION: ReferenceCompletion = { type: normal$.literal };

function completionFromNode(
  type: InferOutput<typeof return$> | InferOutput<typeof throw$>,
  node: { start?: number; end?: number },
): ReferenceCompletion {
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    // Defensive: a real parser always produces start/end. Fall through to
    // normal so the analyzer never throws on malformed input.
    return NORMAL_COMPLETION;
  }
  return { type, startOffset: start, endOffset: end };
}
