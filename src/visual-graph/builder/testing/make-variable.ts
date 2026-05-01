import type { SerializedVariable } from "../../../ir/model.js";
import { makeDef } from "./make-def.js";
import { span } from "./span.js";

export function makeVariable(
  overrides: Partial<SerializedVariable> = {},
): SerializedVariable {
  return {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [span()],
    references: [],
    defs: [makeDef()],
    ...overrides,
  };
}
