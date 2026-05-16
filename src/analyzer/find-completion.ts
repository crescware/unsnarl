import type { InferOutput } from "valibot";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import { normal$, return$, throw$ } from "../ir/reference/completion-type.js";
import type { Completion } from "../ir/reference/completion.js";
import { AST_TYPE } from "../parser/ast-type.js";

/**
 * Walk a Reference's ancestor chain from leaf to root and resolve the
 * Completion category that carries its value. Total: every Reference
 * is classified.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export function findCompletion(path: readonly PathEntry[]): Completion {
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
  }
  return NORMAL_COMPLETION;
}

const NORMAL_COMPLETION: Completion = { type: normal$.literal };

function completionFromNode(
  type: InferOutput<typeof return$> | InferOutput<typeof throw$>,
  node: { start?: number; end?: number },
): Completion {
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    // Defensive: a real parser always produces start/end. Fall through to
    // normal so the analyzer never throws on malformed input.
    return NORMAL_COMPLETION;
  }
  return { type, startOffset: start, endOffset: end };
}
