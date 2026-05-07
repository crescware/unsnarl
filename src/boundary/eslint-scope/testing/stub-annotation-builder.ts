import type { AnnotationBuilder } from "../annotation-builder.js";

// Empty AnnotationBuilder for unit tests of the eslint-compat algorithm
// pass that do not exercise unsnarl-specific annotation production.
// Returning zero-value annotations lets handle-enter and
// handle-identifier-reference run without pulling in any annotation
// producer.
export const stubAnnotationBuilder: AnnotationBuilder = {
  buildReferenceAnnotation() {
    return {
      owners: [],
      predicateContainer: null,
      returnContainer: null,
      jsxElement: null,
      expressionStatementContainer: null,
    };
  },
};
