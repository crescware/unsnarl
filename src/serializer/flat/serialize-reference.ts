import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import { spanFromOffset } from "../../util/span.js";
import { spanOf } from "./span-of.js";

export function serializeReference(
  r: Reference,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedReference {
  const id = referenceIds.get(r);
  if (id === undefined) {
    throw new Error("Reference id not found");
  }
  return {
    id,
    identifier: { name: r.identifier.name, span: spanOf(r.identifier, raw) },
    from: scopeIds.get(r.from) ?? "",
    resolved: r.resolved ? (variableIds.get(r.resolved) ?? null) : null,
    owners: (r.unsnarlOwners ?? [])
      .map((o) => variableIds.get(o))
      .filter((x): x is string => x !== undefined),
    init: r.init,
    flags: {
      read: r.isRead(),
      write: r.isWrite(),
      call: r.isCall?.() ?? false,
      receiver: r.isReceiver?.() ?? false,
    },
    predicateContainer: r.unsnarlPredicateContainer ?? null,
    returnContainer: r.unsnarlReturnContainer
      ? {
          startSpan: spanFromOffset(raw, r.unsnarlReturnContainer.startOffset),
          endSpan: spanFromOffset(raw, r.unsnarlReturnContainer.endOffset),
        }
      : null,
    jsxElement: r.unsnarlJsxElement
      ? {
          startSpan: spanFromOffset(raw, r.unsnarlJsxElement.startOffset),
          endSpan: spanFromOffset(raw, r.unsnarlJsxElement.endOffset),
        }
      : null,
    expressionStatementContainer: r.unsnarlExpressionStatementContainer
      ? {
          startSpan: spanFromOffset(
            raw,
            r.unsnarlExpressionStatementContainer.startOffset,
          ),
          endSpan: spanFromOffset(
            raw,
            r.unsnarlExpressionStatementContainer.endOffset,
          ),
          headStartSpan: spanFromOffset(
            raw,
            r.unsnarlExpressionStatementContainer.headStartOffset,
          ),
          headEndSpan: spanFromOffset(
            raw,
            r.unsnarlExpressionStatementContainer.headEndOffset,
          ),
          isCall: r.unsnarlExpressionStatementContainer.isCall,
        }
      : null,
  };
}
