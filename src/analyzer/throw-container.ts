import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { ThrowContainer } from "../ir/reference/throw-container.js";
import { AST_TYPE } from "../parser/ast-type.js";

export function findThrowContainer(
  path: readonly PathEntry[],
): ThrowContainer | null {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    if (type === AST_TYPE.ThrowStatement) {
      return spanFromNode(entry.node);
    }
    if (
      type === AST_TYPE.ArrowFunctionExpression ||
      type === AST_TYPE.FunctionExpression ||
      type === AST_TYPE.FunctionDeclaration
    ) {
      // A nested function boundary cuts off the search: a throw inside an
      // inner function belongs to that inner function, not to the outer one.
      return null;
    }
  }
  return null;
}

function spanFromNode(node: {
  start?: number;
  end?: number;
}): ThrowContainer | null {
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    return null;
  }
  return { startOffset: start, endOffset: end };
}
