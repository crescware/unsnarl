import type { ReturnContainer } from "../ir/model.js";
import { AST_TYPE } from "../parser/ast-type.js";
import type { PathEntry } from "./walk/walk.js";

export function findReturnContainer(
  path: readonly PathEntry[],
): ReturnContainer | null {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    if (type === AST_TYPE.ReturnStatement) {
      return spanFromNode(entry.node);
    }
    if (type === AST_TYPE.ArrowFunctionExpression) {
      // Block-body arrows defer to an inner ReturnStatement (already handled
      // by the deeper path entry). Expression-body arrows have no explicit
      // return: the body expression itself is the implicit return target.
      const body = (
        entry.node as unknown as { body?: { type?: string } | null }
      ).body;
      if (body && body.type !== AST_TYPE.BlockStatement) {
        return spanFromNode(body as { start?: number; end?: number });
      }
      return null;
    }
    if (
      type === AST_TYPE.FunctionExpression ||
      type === AST_TYPE.FunctionDeclaration
    ) {
      // Block-body only; rely on a nested ReturnStatement.
      return null;
    }
  }
  return null;
}

function spanFromNode(node: {
  start?: number;
  end?: number;
}): ReturnContainer | null {
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    return null;
  }
  return { startOffset: start, endOffset: end };
}
