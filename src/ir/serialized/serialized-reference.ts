import type { Span } from "../primitive/span.js";
import type { PredicateContainer } from "../reference/predicate-container.js";
import type { ReferenceId, ScopeId, VariableId } from "./ids.js";

export type SerializedReference = Readonly<{
  id: ReferenceId;
  identifier: Readonly<{ name: string; span: Span }>;
  from: ScopeId;
  resolved: VariableId | null;
  owners: readonly VariableId[];
  init: boolean;
  flags: Readonly<{
    read: boolean;
    write: boolean;
    call: boolean;
    receiver: boolean;
  }>;
  predicateContainer: PredicateContainer | null;
  returnContainer: Readonly<{ startSpan: Span; endSpan: Span }> | null;
  jsxElement: Readonly<{ startSpan: Span; endSpan: Span }> | null;
  expressionStatementContainer: Readonly<{
    startSpan: Span;
    endSpan: Span;
    headStartSpan: Span;
    headEndSpan: Span;
    isCall: boolean;
  }> | null;
}>;
