import { brand, parse, pipe, string, type InferOutput } from "valibot";

export const referenceId$ = pipe(string(), brand("ReferenceId"));
export type ReferenceId = InferOutput<typeof referenceId$>;

export function asReferenceId(value: string): ReferenceId {
  return parse(referenceId$, value);
}
