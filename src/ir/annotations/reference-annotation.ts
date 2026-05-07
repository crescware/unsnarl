import type { ExpressionStatementContainer } from "../reference/expression-statement-container.js";
import type { JsxElementContainer } from "../reference/jsx-element-container.js";
import type { PredicateContainer } from "../reference/predicate-container.js";
import type { ReturnContainer } from "../reference/return-container.js";
import type { Variable } from "../scope/variable.js";

export type ReferenceAnnotation = Readonly<{
  owners: /* mutable */ Variable[];
  predicateContainer: PredicateContainer | null;
  returnContainer: ReturnContainer | null;
  jsxElement: JsxElementContainer | null;
  expressionStatementContainer: ExpressionStatementContainer | null;
}>;
