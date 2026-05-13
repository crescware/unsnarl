import type { SerializedVariable } from "../../../ir/serialized/serialized-variable.js";
import { VARIABLE_DECLARATION_KIND } from "../../../serializer/variable-declaration-kind.js";
import { baseDef } from "./make-def.js";
import { span } from "./span.js";

// Mirrors the boundary invariant `defs.length === 0 ⟺ identifiers.length === 0`.
// Defaulting to a populated def keeps test fixtures from constructing states
// that the real pipeline never reaches (the only producer of empty defs is
// declareImplicitArguments, which is filtered out before any node is built).
export function baseVariable(): SerializedVariable {
  return {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [span()],
    references: [],
    defs: [baseDef(VARIABLE_DECLARATION_KIND.Let)],
  };
}
