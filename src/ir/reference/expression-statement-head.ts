// Mini-AST that captures the shape of an ExpressionStatement's "head" for
// display purposes. The analyzer narrows the parser AST down to this small
// vocabulary; emitters walk the result to render a compact label.
//
// Every leaf-bearing piece of information (`name`, span offsets) comes from
// the parser AST. The shape itself is a derived view: it discards call
// arguments, computed property contents, and other detail that would bloat
// a one-line label. Synthesised display text (`()`, `await `, `.`) is the
// emitter's responsibility, not the IR's.
export type HeadExpression =
  | Readonly<{ kind: "identifier"; name: string }>
  | Readonly<{
      kind: "member";
      object: HeadExpression;
      property: string;
    }>
  | Readonly<{ kind: "call"; callee: HeadExpression }>
  | Readonly<{ kind: "new"; callee: HeadExpression }>
  | Readonly<{ kind: "await"; argument: HeadExpression }>
  | Readonly<{ kind: "raw"; startOffset: number; endOffset: number }>;
