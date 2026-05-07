import type { Variable } from "../ir/scope/variable.js";

// A variable is considered unused when every reference to it is an
// initialization-only write (declaration with initializer, parameter
// default, etc.).
export function isUnused(variable: Variable): boolean {
  return variable.references.every((r) => r.init === true && r.isWriteOnly());
}
