import { brand, parse, pipe, type InferOutput } from "valibot";

import { filledString$ } from "../../util/filled-string.js";

export const scopeId$ = pipe(filledString$, brand("ScopeId"));
export type ScopeId = InferOutput<typeof scopeId$>;

export function asScopeId(value: string): ScopeId {
  return parse(scopeId$, value);
}
