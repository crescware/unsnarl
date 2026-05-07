import type { Annotations } from "../ir/annotations/annotations.js";
import type { ReferenceAnnotation } from "../ir/annotations/reference-annotation.js";
import type { ScopeAnnotation } from "../ir/annotations/scope-annotation.js";
import type { Reference } from "../ir/reference/reference.js";
import type { Scope } from "../ir/scope/scope.js";
import { CATEGORY } from "../serializer/category.js";

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
  categoryDepths: {
    [CATEGORY.Function]: 0,
    [CATEGORY.If]: 0,
    [CATEGORY.For]: 0,
    [CATEGORY.While]: 0,
    [CATEGORY.Switch]: 0,
    [CATEGORY.TryCatchFinally]: 0,
    [CATEGORY.Block]: 0,
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
