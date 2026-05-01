import type {
  Reference,
  Scope,
  SerializedReference,
  Variable,
} from "../../ir/model.js";
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
    writeExpr: r.writeExpr ? spanOf(r.writeExpr, raw) : null,
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
  };
}
