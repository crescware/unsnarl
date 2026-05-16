import {
  array,
  boolean,
  nullable,
  object,
  pipe,
  readonly,
  type InferOutput,
} from "valibot";

import { filledString$ } from "../../util/filled-string.js";
import { span$ } from "../primitive/span.js";
import { predicateContainer$ } from "../reference/predicate-container.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import { serializedHeadExpression$ } from "./serialized-expression-statement-head.js";
import { variableId$ } from "./variable-id.js";

export const serializedReference$ = object({
  id: referenceId$,
  identifier: pipe(object({ name: filledString$, span: span$ }), readonly()),
  from: scopeId$,
  resolved: nullable(variableId$),
  owners: pipe(array(variableId$), readonly()),
  init: boolean(),
  flags: pipe(
    object({
      read: boolean(),
      write: boolean(),
      call: boolean(),
      receiver: boolean(),
    }),
    readonly(),
  ),
  predicateContainer: nullable(predicateContainer$),
  returnContainer: nullable(
    pipe(object({ startSpan: span$, endSpan: span$ }), readonly()),
  ),
  throwContainer: nullable(
    pipe(object({ startSpan: span$, endSpan: span$ }), readonly()),
  ),
  jsxElement: nullable(
    pipe(object({ startSpan: span$, endSpan: span$ }), readonly()),
  ),
  expressionStatementContainer: nullable(
    pipe(
      object({
        startSpan: span$,
        endSpan: span$,
        head: serializedHeadExpression$,
      }),
      readonly(),
    ),
  ),
});

export type SerializedReference = InferOutput<typeof serializedReference$>;
