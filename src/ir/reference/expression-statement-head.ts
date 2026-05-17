import {
  boolean,
  lazy,
  number,
  object,
  pipe,
  readonly,
  string,
  variant,
  type GenericSchema,
  type InferOutput,
} from "valibot";

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
} from "./expression-statement-head-kind.js";

// Mini-AST that captures the shape of an ExpressionStatement's "head" for
// display purposes. The analyzer narrows the parser AST down to this small
// vocabulary; emitters walk the result to render a compact label.
//
// Recursive (member.object, call.callee, new.callee, await.argument all
// reference HeadExpression). valibot's official recursive-schema pattern
// requires the type to be hand-written and the schema to be annotated with
// `GenericSchema<unknown, T>`: TS cannot infer the output type from a
// self-referencing schema, so InferOutput is not available here. The
// GenericSchema annotation lets the compiler still verify the schema's
// output shape matches HeadExpression.
// Each operand of an assign / update head carries its own source span
// alongside the structural head. This keeps the per-side position
// information that the older `raw` head used to provide for the whole
// AssignmentExpression / UpdateExpression -- without that span,
// downstream consumers would lose the ability to locate the operand in
// source, especially for the `elided` operand where the structural
// head carries no position data of its own.
export type HeadOperand = Readonly<{
  head: HeadExpression;
  startOffset: number;
  endOffset: number;
}>;

export type HeadExpression =
  | Readonly<{ kind: InferOutput<typeof identifier$>; name: string }>
  | Readonly<{
      kind: InferOutput<typeof member$>;
      object: HeadExpression;
      property: string;
    }>
  | Readonly<{
      kind: InferOutput<typeof call$>;
      callee: HeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof new$>;
      callee: HeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof await$>;
      argument: HeadExpression;
    }>
  | Readonly<{
      kind: InferOutput<typeof assign$>;
      operator: string;
      left: HeadOperand;
      right: HeadOperand;
    }>
  | Readonly<{
      kind: InferOutput<typeof update$>;
      operator: string;
      prefix: boolean;
      argument: HeadOperand;
    }>
  | Readonly<{ kind: InferOutput<typeof elided$> }>
  | Readonly<{
      kind: InferOutput<typeof raw$>;
      startOffset: number;
      endOffset: number;
    }>;

const headOperand$: GenericSchema<unknown, HeadOperand> = lazy(() =>
  pipe(
    object({
      head: headExpression$,
      startOffset: number(),
      endOffset: number(),
    }),
    readonly(),
  ),
);

export const headExpression$: GenericSchema<unknown, HeadExpression> = lazy(
  () =>
    variant("kind", [
      pipe(
        object({
          kind: identifier$,
          name: string(),
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: member$,
          object: headExpression$,
          property: string(),
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: call$,
          callee: headExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: new$,
          callee: headExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: await$,
          argument: headExpression$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: assign$,
          operator: string(),
          left: headOperand$,
          right: headOperand$,
        }),
        readonly(),
      ),
      pipe(
        object({
          kind: update$,
          operator: string(),
          prefix: boolean(),
          argument: headOperand$,
        }),
        readonly(),
      ),
      pipe(object({ kind: elided$ }), readonly()),
      pipe(
        object({
          kind: raw$,
          startOffset: number(),
          endOffset: number(),
        }),
        readonly(),
      ),
    ]),
);
