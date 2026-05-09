import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { ExpressionStatementContainer } from "../ir/reference/expression-statement-container.js";
import { AST_TYPE } from "../parser/ast-type.js";

type Spanned = { start?: number; end?: number };
type ExpressionStatementNode = Spanned & {
  expression?: (Spanned & { type?: string; callee?: Spanned }) | null;
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
  const isCall = expression?.type === AST_TYPE.CallExpression;
  const headSource =
    isCall && expression?.callee ? expression.callee : (expression ?? node);
  const headStart = headSource.start;
  const headEnd = headSource.end;
  if (typeof headStart !== "number" || typeof headEnd !== "number") {
    return null;
  }
  return {
    startOffset: start,
    endOffset: end,
    headStartOffset: headStart,
    headEndOffset: headEnd,
    isCall,
  };
}
