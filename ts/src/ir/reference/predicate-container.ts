import { number, object, pipe, readonly, type InferOutput } from "valibot";

import { predicateContainerType$ } from "../../analyzer/predicate-container-type.js";

export const predicateContainer$ = pipe(
  object({
    type: predicateContainerType$,
    offset: number(),
  }),
  readonly(),
);

export type PredicateContainer = InferOutput<typeof predicateContainer$>;
