import type { Annotations } from "../../ir/annotations/annotations.js";
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
  annotations: Annotations,
  raw: string,
): SerializedReference {
  const id = referenceIds.get(r) ?? null;
  if (id === null) {
    throw new Error("Reference id not found");
  }
  const ann = annotations.ofReference(r);
  return {
    id,
    identifier: { name: r.identifier.name, span: spanOf(r.identifier, raw) },
    from: scopeIds.get(r.from) ?? "",
    resolved: r.resolved ? (variableIds.get(r.resolved) ?? null) : null,
    owners: ann.owners
      .map((o) => variableIds.get(o) ?? null)
      .filter((x): x is string => x !== null),
    init: r.init,
    flags: {
      read: r.isRead(),
      write: r.isWrite(),
      call: ann.flags.call,
      receiver: ann.flags.receiver,
    },
    predicateContainer: ann.predicateContainer,
    returnContainer: ann.returnContainer
      ? {
          startSpan: spanFromOffset(raw, ann.returnContainer.startOffset),
          endSpan: spanFromOffset(raw, ann.returnContainer.endOffset),
        }
      : null,
    jsxElement: ann.jsxElement
      ? {
          startSpan: spanFromOffset(raw, ann.jsxElement.startOffset),
          endSpan: spanFromOffset(raw, ann.jsxElement.endOffset),
        }
      : null,
    expressionStatementContainer: ann.expressionStatementContainer
      ? {
          startSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.startOffset,
          ),
          endSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.endOffset,
          ),
          headStartSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.headStartOffset,
          ),
          headEndSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.headEndOffset,
          ),
          isCall: ann.expressionStatementContainer.isCall,
        }
      : null,
  };
}
