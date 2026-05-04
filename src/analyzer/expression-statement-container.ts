import type { ExpressionStatementContainer } from "../ir/reference/expression-statement-container.js";
import { AST_TYPE } from "../parser/ast-type.js";
import type { PathEntry } from "./walk/walk.js";

type Spanned = { start?: number; end?: number };
type ExpressionStatementNode = Spanned & {
  expression?: (Spanned & { type?: string; callee?: Spanned }) | null;
};

export function findExpressionStatementContainer(
  path: readonly PathEntry[],
): ExpressionStatementContainer | null {
  let candidate: ExpressionStatementNode | null = null;
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    // A function ancestor at any depth means the ref will be routed to
    // ReturnUse (or further inward), not to module_root. Discard any
    // closer ExpressionStatement we already captured — it would never be
    // consulted by the builder.
    if (
      type === AST_TYPE.FunctionDeclaration ||
      type === AST_TYPE.FunctionExpression ||
      type === AST_TYPE.ArrowFunctionExpression
    ) {
      return null;
    }
    if (candidate === null && type === AST_TYPE.ExpressionStatement) {
      candidate = entry.node as unknown as ExpressionStatementNode;
    }
  }
  return candidate ? containerFromExpressionStatement(candidate) : null;
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
