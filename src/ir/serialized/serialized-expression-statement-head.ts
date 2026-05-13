import {
  lazy,
  literal,
  object,
  pipe,
  readonly,
  string,
  variant,
  type GenericSchema,
  type InferOutput,
} from "valibot";

import { span$ } from "../primitive/span.js";

// Serialized counterpart of `src/ir/reference/expression-statement-head.ts`.
// Identical shape except the `raw` leaf carries `Span` (line/column/offset)
// instead of bare offsets, matching the convention used elsewhere in the
// serialized IR.
//
// The schema is recursive (member.object, call.callee, new.callee,
// await.argument all reference SerializedHeadExpression). valibot's lazy()
// breaks the cycle so InferOutput can resolve the type.
export const serializedHeadExpression$: GenericSchema<SerializedHeadExpression> =
  lazy(() =>
    variant("kind", [
      pipe(object({ kind: literal("identifier"), name: string() }), readonly()),
      pipe(
        object({
          kind: literal("member"),
          object: serializedHeadExpression$,
          property: string(),
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: literal("call"),
          callee: serializedHeadExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: literal("new"),
          callee: serializedHeadExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: literal("await"),
          argument: serializedHeadExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: literal("raw"),
          startSpan: span$,
          endSpan: span$,
        }),
        readonly(),
      ),
    ]),
  );

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
  | Readonly<{
      kind: "raw";
      startSpan: InferOutput<typeof span$>;
      endSpan: InferOutput<typeof span$>;
    }>;
