import { brand, parse, pipe, string, type InferOutput } from "valibot";

export const variableId$ = pipe(string(), brand("VariableId"));
export type VariableId = InferOutput<typeof variableId$>;

export function asVariableId(value: string): VariableId {
  return parse(variableId$, value);
}
