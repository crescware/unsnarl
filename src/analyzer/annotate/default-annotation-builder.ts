import type {
  AnnotationBuilder,
  ReferenceAnnotation,
  ReferenceAnnotationInput,
} from "../eslint-compat/annotation-builder.js";
import { findExpressionStatementContainer } from "../expression-statement-container.js";
import { findJsxElementSpan } from "../jsx-element-span.js";
import { findReferenceOwners } from "../owner/find-reference-owners.js";
import { findPredicateContainer } from "../predicate.js";
import { findReturnContainer } from "../return-container.js";

// Adapter: implements the eslint-compat port by calling the unsnarl
// annotation producers (predicate / return-container / jsx-element-span /
// expression-statement-container / owner). Lives outside eslint-compat
// so the algorithm layer never imports an unsnarl-specific producer
// directly.
export class DefaultAnnotationBuilder implements AnnotationBuilder {
  buildReferenceAnnotation(
    input: ReferenceAnnotationInput,
  ): ReferenceAnnotation {
    const { parent, key, path, scope } = input;
    return {
      owners: findReferenceOwners(path, scope),
      predicateContainer: findPredicateContainer(
        parent as unknown as { type: string; start?: number } | null,
        key,
        path,
      ),
      returnContainer: findReturnContainer(path),
      jsxElement: findJsxElementSpan(path),
      expressionStatementContainer: findExpressionStatementContainer(path),
    };
  }
}
