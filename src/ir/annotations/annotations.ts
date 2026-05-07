import type { Reference } from "../reference/reference.js";
import type { Scope } from "../scope/scope.js";
import type { ReferenceAnnotation } from "./reference-annotation.js";
import type { ScopeAnnotation } from "./scope-annotation.js";

// Missing entries return zero-value defaults so callers do not need to
// special-case absence.
export type Annotations = Readonly<{
  ofReference(ref: Reference): ReferenceAnnotation;
  ofScope(scope: Scope): ScopeAnnotation;
}>;
