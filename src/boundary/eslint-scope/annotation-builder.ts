import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { ExpressionStatementContainer } from "../../ir/reference/expression-statement-container.js";
import type { JsxElementContainer } from "../../ir/reference/jsx-element-container.js";
import type { PredicateContainer } from "../../ir/reference/predicate-container.js";
import type { ReturnContainer } from "../../ir/reference/return-container.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { NodeLike } from "./node-like.js";

// Port interface: declared inside eslint-compat so the algorithm layer
// can call out to unsnarl-specific annotation producers (predicate /
// return-container / jsx-element-span / expression-statement-container /
// owner) without importing them. The concrete adapter lives outside
// eslint-compat (src/analyzer/annotate/) and is wired at the composition
// root.
export type ReferenceAnnotationInput = Readonly<{
  parent: NodeLike | null;
  key: string | null;
  path: readonly PathEntry[];
  scope: Scope;
}>;

export type ReferenceAnnotation = Readonly<{
  owners: /* mutable */ Variable[];
  predicateContainer: PredicateContainer | null;
  returnContainer: ReturnContainer | null;
  jsxElement: JsxElementContainer | null;
  expressionStatementContainer: ExpressionStatementContainer | null;
}>;

export type AnnotationBuilder = Readonly<{
  buildReferenceAnnotation(
    input: ReferenceAnnotationInput,
  ): ReferenceAnnotation;
}>;
