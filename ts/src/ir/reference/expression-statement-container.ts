import type { HeadExpression } from "./expression-statement-head.js";

export type ExpressionStatementContainer = Readonly<{
  startOffset: number;
  endOffset: number;
  // Structural view of the statement's head expression. See
  // `expression-statement-head.ts` for the type's design intent.
  head: HeadExpression;
}>;
