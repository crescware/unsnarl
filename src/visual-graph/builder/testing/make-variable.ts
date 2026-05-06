import type { SerializedVariable } from "../../../ir/serialized/serialized-variable.js";
import { span } from "./span.js";

export function baseVariable(): SerializedVariable {
  return {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [span()],
    references: [],
    defs: [],
  };
}
