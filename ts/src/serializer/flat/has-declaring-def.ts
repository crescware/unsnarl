import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { Variable } from "../../ir/scope/variable.js";

export function hasDeclaringDef(variable: Variable): boolean {
  return variable.defs.some(
    (v) => v.type !== DEFINITION_TYPE.ImplicitGlobalVariable,
  );
}
