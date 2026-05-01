import type { Variable } from "../../ir/model.js";

export function hasDeclaringDef(v: Variable): boolean {
  return v.defs.some((d) => d.type !== "ImplicitGlobalVariable");
}
