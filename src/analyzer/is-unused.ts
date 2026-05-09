import type { Variable } from "../ir/scope/variable.js";

// Source of truth for `Annotations.ofVariable(v).isUnused`. The analysis
// pipeline calls this once per variable to populate the VariableAnnotation
// side-table; downstream consumers should read through
// `Annotations.ofVariable` rather than calling this directly.
//
// A variable is considered unused when no reference to it ever reads its
// value. Writes (including the init Write and any later re-assignments) do
// not count as usage. See #45 for the rationale.
//
// Recursive-only references (e.g. `function foo() { foo(); }`) currently
// count as Read and therefore keep the variable not-unused. Whether to
// exclude self-resolving Read references is tracked by #68.
export function isUnused(variable: Variable): boolean {
  return !variable.references.some((r) => r.isRead());
}
