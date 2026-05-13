import {
  array,
  boolean,
  custom,
  nullable,
  object,
  string,
  type InferOutput,
} from "valibot";

import type { Span } from "../primitive/span.js";
import type { PredicateContainer } from "../reference/predicate-container.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import type { SerializedHeadExpression } from "./serialized-expression-statement-head.js";
import { variableId$ } from "./variable-id.js";

const span$ = custom<Span>(() => true);
const predicateContainer$ = custom<PredicateContainer>(() => true);
const serializedHeadExpression$ = custom<SerializedHeadExpression>(() => true);

export const serializedReference$ = object({
  id: referenceId$,
  identifier: object({ name: string(), span: span$ }),
  from: scopeId$,
  resolved: nullable(variableId$),
  owners: array(variableId$),
  init: boolean(),
  flags: object({
    read: boolean(),
    write: boolean(),
    call: boolean(),
    receiver: boolean(),
  }),
  predicateContainer: nullable(predicateContainer$),
  returnContainer: nullable(object({ startSpan: span$, endSpan: span$ })),
  jsxElement: nullable(object({ startSpan: span$, endSpan: span$ })),
  expressionStatementContainer: nullable(
    object({
      startSpan: span$,
      endSpan: span$,
      head: serializedHeadExpression$,
    }),
  ),
});

export type SerializedReference = InferOutput<typeof serializedReference$>;
