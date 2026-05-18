import {
  boolean,
  lazy,
  object,
  pipe,
  readonly,
  string,
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
  assign$,
  update$,
  elided$,
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
// Serialized counterpart of HeadOperand: the per-side span info that
// keeps source-position fidelity for assign / update operands (matches
// the convention used by raw heads: Span carries line+column+offset).
const serializedHeadOperand$: GenericSchema<unknown, SerializedHeadOperand> =
  lazy(() =>
    pipe(
      object({
        head: serializedHeadExpression$,
        startSpan: span$,
        endSpan: span$,
      }),
      readonly(),
    ),
  );

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
        kind: assign$,
        operator: string(),
        left: serializedHeadOperand$,
        right: serializedHeadOperand$,
      }),
      readonly(),
    ),
    pipe(
      object({
        kind: update$,
        operator: string(),
        prefix: boolean(),
        argument: serializedHeadOperand$,
      }),
      readonly(),
    ),
    pipe(object({ kind: elided$ }), readonly()),
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

export type SerializedHeadOperand = Readonly<{
  head: SerializedHeadExpression;
  startSpan: Span;
  endSpan: Span;
}>;

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
      kind: InferOutput<typeof assign$>;
      operator: string;
      left: SerializedHeadOperand;
      right: SerializedHeadOperand;
    }>
  | Readonly<{
      kind: InferOutput<typeof update$>;
      operator: string;
      prefix: boolean;
      argument: SerializedHeadOperand;
    }>
  | Readonly<{ kind: InferOutput<typeof elided$> }>
  | Readonly<{
      kind: InferOutput<typeof raw$>;
      startSpan: Span;
      endSpan: Span;
    }>;
