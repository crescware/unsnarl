import {
  array,
  boolean,
  nullable,
  object,
  pipe,
  readonly,
  variant,
  type InferOutput,
} from "valibot";

import { filledString$ } from "../../util/filled-string.js";
import { normal$, return$, throw$ } from "../completion/completion-type.js";
import { span$ } from "../primitive/span.js";
import { predicateContainer$ } from "../reference/predicate-container.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import { serializedHeadExpression$ } from "./serialized-expression-statement-head.js";
import { variableId$ } from "./variable-id.js";

/**
 * Span-based Completion shape.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
const serializedCompletion$ = variant("type", [
  pipe(object({ type: normal$ }), readonly()),
  pipe(object({ type: return$, startSpan: span$, endSpan: span$ }), readonly()),
  pipe(object({ type: throw$, startSpan: span$, endSpan: span$ }), readonly()),
]);

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
  completion: serializedCompletion$,
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
