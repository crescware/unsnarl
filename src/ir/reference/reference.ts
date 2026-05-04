import type { AstIdentifier } from "../primitive/ast-identifier.js";
import type { Scope } from "../scope/scope.js";
import type { Variable } from "../scope/variable.js";
import type { ExpressionStatementContainer } from "./expression-statement-container.js";
import type { JsxElementContainer } from "./jsx-element-container.js";
import type { PredicateContainer } from "./predicate-container.js";
import type { ReferenceFlagBits } from "./reference-flags.js";
import type { ReturnContainer } from "./return-container.js";

// Reference / Variable / Scope keep mutable fields and arrays because the
// builder mutates them in place during scope analysis (ScopeImpl pushes
// onto `variables`, `references`, etc.; bindReference reassigns
// `resolved`). Wrapping in Readonly<...> would break those algorithms.
export type Reference = {
  identifier: AstIdentifier;
  from: Scope;
  resolved: Variable | null;
  init: boolean;
  isWrite(): boolean;
  isRead(): boolean;
  isReadOnly(): boolean;
  isWriteOnly(): boolean;
  isReadWrite(): boolean;
  isCall(): boolean;
  isReceiver(): boolean;
  unsnarlFlags: ReferenceFlagBits;
  unsnarlOwners: /* mutable */ Variable[];
  unsnarlPredicateContainer: PredicateContainer | null;
  unsnarlReturnContainer: ReturnContainer | null;
  unsnarlJsxElement: JsxElementContainer | null;
  unsnarlExpressionStatementContainer: ExpressionStatementContainer | null;
};
