import {
  array,
  custom,
  object,
  pipe,
  readonly,
  string,
  tupleWithRest,
  type InferOutput,
} from "valibot";

import type { Span } from "../primitive/span.js";
import { referenceId$ } from "./reference-id.js";
import { scopeId$ } from "./scope-id.js";
import type { SerializedDefinition } from "./serialized-definition.js";
import { variableId$ } from "./variable-id.js";

const span$ = custom<Span>(() => true);
const serializedDefinition$ = custom<SerializedDefinition>(() => true);

// SerializedVariable carries at least one def by construction: the serializer
// filters implicit-arguments bindings (the only producer of empty defs) at
// boundary entry. tupleWithRest expresses that invariant at both runtime
// (parse rejects empty arrays) and at the type level (defs[0] narrows to
// SerializedDefinition rather than SerializedDefinition | undefined under
// noUncheckedIndexedAccess).
export const serializedVariable$ = object({
  id: variableId$,
  name: string(),
  scope: scopeId$,
  identifiers: array(span$),
  references: array(referenceId$),
  defs: pipe(
    tupleWithRest([serializedDefinition$], serializedDefinition$),
    readonly(),
  ),
});

export type SerializedVariable = InferOutput<typeof serializedVariable$>;
