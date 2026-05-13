import {
  lazy,
  object,
  pipe,
  readonly,
  variant,
  type GenericSchema,
  type InferOutput,
} from "valibot";

import { filledString$, type FilledString } from "../../util/filled-string.js";
import { span$, type Span } from "../primitive/span.js";
import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  raw$,
} from "../reference/expression-statement-head-kind.js";

// Serialized counterpart of `src/ir/reference/expression-statement-head.ts`.
// Identical shape except the `raw` leaf carries `Span` (line/column/offset)
// instead of bare offsets, matching the convention used elsewhere in the
// serialized IR.
//
// The schema is recursive (member.object, call.callee, new.callee,
// await.argument all reference SerializedHeadExpression). valibot's lazy()
// breaks the cycle so InferOutput can resolve the type.
export const serializedHeadExpression$: GenericSchema<
  unknown,
  SerializedHeadExpression
> = lazy(() =>
  variant("kind", [
    pipe(
      object({
        kind: identifier$,
        name: filledString$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: member$,
        object: serializedHeadExpression$,
        property: filledString$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: call$,
        callee: serializedHeadExpression$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: new$,
        callee: serializedHeadExpression$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: await$,
        argument: serializedHeadExpression$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: raw$,
        startSpan: span$,
        endSpan: span$,
      }),
      readonly(),
    ),
  ]),
);

export type SerializedHeadExpression =
  | Readonly<{
      kind: InferOutput<typeof identifier$>;
      name: FilledString;
    }>
  | Readonly<{
      kind: InferOutput<typeof member$>;
      object: SerializedHeadExpression;
      property: FilledString;
    }>
  | Readonly<{
      kind: InferOutput<typeof call$>;
      callee: SerializedHeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof new$>;
      callee: SerializedHeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof await$>;
      argument: SerializedHeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof raw$>;
      startSpan: Span;
      endSpan: Span;
    }>;
