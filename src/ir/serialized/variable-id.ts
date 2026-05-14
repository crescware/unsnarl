import { brand, parse, pipe, type InferOutput } from "valibot";

import { filledString$ } from "../../util/filled-string.js";

export const variableId$ = pipe(filledString$, brand("VariableId"));
export type VariableId = InferOutput<typeof variableId$>;

export function asVariableId(value: string): VariableId {
  return parse(variableId$, value);
}
