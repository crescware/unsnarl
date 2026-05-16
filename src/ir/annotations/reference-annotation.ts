import type { ExpressionStatementContainer } from "../reference/expression-statement-container.js";
import type { JsxElementContainer } from "../reference/jsx-element-container.js";
import type { PredicateContainer } from "../reference/predicate-container.js";
import type { ReferenceCompletion } from "../reference/reference-completion.js";
import type { Variable } from "../scope/variable.js";

export type ReferenceAnnotation = Readonly<{
  owners: /* mutable */ Variable[];
  flags: Readonly<{ call: boolean; receiver: boolean }>;
  predicateContainer: PredicateContainer | null;
  completion: ReferenceCompletion;
  jsxElement: JsxElementContainer | null;
  expressionStatementContainer: ExpressionStatementContainer | null;
}>;
