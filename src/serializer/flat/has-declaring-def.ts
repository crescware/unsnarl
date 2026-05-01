import { DEFINITION_TYPE } from "../../definition-type.js";
import type { Variable } from "../../ir/model.js";

export function hasDeclaringDef(v: Variable): boolean {
  return v.defs.some((d) => d.type !== DEFINITION_TYPE.ImplicitGlobalVariable);
}
