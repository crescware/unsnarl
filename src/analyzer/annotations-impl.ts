import type { Annotations } from "../ir/annotations/annotations.js";
import type { ReferenceAnnotation } from "../ir/annotations/reference-annotation.js";
import type { ScopeAnnotation } from "../ir/annotations/scope-annotation.js";
import type { Reference } from "../ir/reference/reference.js";
import type { Scope } from "../ir/scope/scope.js";
import { NESTING_KIND } from "../serializer/nesting-kind.js";

const EMPTY_REFERENCE_ANNOTATION: ReferenceAnnotation = {
  owners: [],
  predicateContainer: null,
  returnContainer: null,
  jsxElement: null,
  expressionStatementContainer: null,
};

const EMPTY_SCOPE_ANNOTATION: ScopeAnnotation = {
  blockContext: null,
  fallsThrough: false,
  exitsFunction: false,
  nestingDepths: {
    [NESTING_KIND.Function]: 0,
    [NESTING_KIND.If]: 0,
    [NESTING_KIND.For]: 0,
    [NESTING_KIND.While]: 0,
    [NESTING_KIND.Switch]: 0,
    [NESTING_KIND.TryCatchFinally]: 0,
    [NESTING_KIND.Block]: 0,
  },
};

export class AnnotationsImpl implements Annotations {
  private readonly references: Map<Reference, ReferenceAnnotation> = new Map();
  private readonly scopes: Map<Scope, ScopeAnnotation> = new Map();

  ofReference(ref: Reference): ReferenceAnnotation {
    return this.references.get(ref) ?? EMPTY_REFERENCE_ANNOTATION;
  }

  ofScope(scope: Scope): ScopeAnnotation {
    return this.scopes.get(scope) ?? EMPTY_SCOPE_ANNOTATION;
  }

  setReference(ref: Reference, annotation: ReferenceAnnotation): void {
    this.references.set(ref, annotation);
  }

  setScope(scope: Scope, annotation: ScopeAnnotation): void {
    this.scopes.set(scope, annotation);
  }
}
