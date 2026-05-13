import { brand, parse, pipe, string, type InferOutput } from "valibot";

export const scopeId$ = pipe(string(), brand("ScopeId"));
export type ScopeId = InferOutput<typeof scopeId$>;

export function asScopeId(value: string): ScopeId {
  return parse(scopeId$, value);
}
