import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import type { ExpressionStatementContainer } from "../ir/reference/expression-statement-container.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { buildHeadExpression } from "./expression-statement-head.js";

type ExpressionStatementNode = {
  start?: number;
  end?: number;
  expression?: AstNode | null;
};

export function findExpressionStatementContainer(
  path: readonly PathEntry[],
): ExpressionStatementContainer | null {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    if (entry.node.type === AST_TYPE.ExpressionStatement) {
      return containerFromExpressionStatement(
        entry.node as unknown as ExpressionStatementNode,
      );
    }
  }
  return null;
}

function containerFromExpressionStatement(
  node: ExpressionStatementNode,
): ExpressionStatementContainer | null {
  const start = node.start;
  const end = node.end;
  if (typeof start !== "number" || typeof end !== "number") {
    return null;
  }
  const expression = node.expression ?? null;
  return {
    startOffset: start,
    endOffset: end,
    head: buildHeadExpression(expression, {
      startOffset: start,
      endOffset: end,
    }),
  };
}
