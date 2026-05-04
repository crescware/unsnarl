export type ExpressionStatementContainer = Readonly<{
  startOffset: number;
  endOffset: number;
  // Span of the expression's "head" used as the display label: the callee
  // for a CallExpression (`console.log` from `console.log(a)`), the whole
  // expression otherwise (`x` from `x;`).
  headStartOffset: number;
  headEndOffset: number;
  // When true, the renderer appends `()` after the head text.
  isCall: boolean;
}>;
