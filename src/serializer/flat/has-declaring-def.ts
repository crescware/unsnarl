import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { Variable } from "../../ir/scope/variable.js";

export function hasDeclaringDef(v: Variable): boolean {
  return v.defs.some((d) => d.type !== DEFINITION_TYPE.ImplicitGlobalVariable);
}
