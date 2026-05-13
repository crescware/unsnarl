import { brand, parse, pipe, type InferOutput } from "valibot";

import { filledString$ } from "../../util/filled-string.js";

export const referenceId$ = pipe(filledString$, brand("ReferenceId"));
export type ReferenceId = InferOutput<typeof referenceId$>;

export function asReferenceId(value: string): ReferenceId {
  return parse(referenceId$, value);
}
