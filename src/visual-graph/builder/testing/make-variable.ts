import { parse } from "valibot";

import {
  serializedVariable$,
  type SerializedVariable,
} from "../../../ir/serialized/serialized-variable.js";
import { VARIABLE_DECLARATION_KIND } from "../../../serializer/variable-declaration-kind.js";
import { baseDef } from "./make-def.js";
import { span } from "./span.js";

// Mirrors the boundary invariant `defs.length === 0 ⟺ identifiers.length === 0`.
// The serializer drops the only producer of empty defs (declareImplicitArguments)
// before reaching this point, so the schema's non-empty tuple is always
// satisfiable from real input.
export function baseVariable(): SerializedVariable {
  return parse(serializedVariable$, {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [span()],
    references: [],
    defs: [baseDef(VARIABLE_DECLARATION_KIND.Let)],
  });
}
