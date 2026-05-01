import type { SerializedVariable } from "../../../ir/model.js";
import { baseDef } from "./make-def.js";
import { span } from "./span.js";

export function baseVariable(): SerializedVariable {
  return {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [span()],
    references: [],
    defs: [baseDef()],
  };
}
