import type { Variable } from "../ir/scope/variable.js";

// Source of truth for `Annotations.ofVariable(v).isUnused`. The analysis
// pipeline calls this once per variable to populate the VariableAnnotation
// side-table; downstream consumers should read through
// `Annotations.ofVariable` rather than calling this directly.
//
// A variable is considered unused when every reference to it is an
// initialization-only write (declaration with initializer, parameter
// default, etc.).
export function isUnused(variable: Variable): boolean {
  return variable.references.every((r) => r.init === true && r.isWriteOnly());
}
