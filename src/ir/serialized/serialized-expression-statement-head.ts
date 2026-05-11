import type { Span } from "../primitive/span.js";

// Serialized counterpart of `src/ir/reference/expression-statement-head.ts`.
// Identical shape except the `raw` leaf carries `Span` (line/column/offset)
// instead of bare offsets, matching the convention used elsewhere in the
// serialized IR.
export type SerializedHeadExpression =
  | Readonly<{ kind: "identifier"; name: string }>
  | Readonly<{
      kind: "member";
      object: SerializedHeadExpression;
      property: string;
    }>
  | Readonly<{ kind: "call"; callee: SerializedHeadExpression }>
  | Readonly<{ kind: "new"; callee: SerializedHeadExpression }>
  | Readonly<{ kind: "await"; argument: SerializedHeadExpression }>
  | Readonly<{ kind: "raw"; startSpan: Span; endSpan: Span }>;
