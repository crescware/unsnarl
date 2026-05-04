import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { spanFromOffset } from "../../util/span.js";
import { spanOf } from "./span-of.js";

export function serializeScope(
  scope: Scope,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedScope {
  const id = scopeIds.get(scope) ?? null;
  if (id === null) {
    throw new Error("Scope id not found");
  }
  return {
    id,
    type: scope.type,
    isStrict: scope.isStrict,
    upper: scope.upper ? (scopeIds.get(scope.upper) ?? null) : null,
    childScopes: scope.childScopes
      .map((c) => scopeIds.get(c) ?? null)
      .filter((x): x is string => x !== null),
    variableScope: scopeIds.get(scope.variableScope) ?? id,
    block: {
      type: scope.block.type,
      span: spanOf(scope.block, raw),
      endSpan: spanFromOffset(raw, scope.block.end ?? scope.block.start ?? 0),
    },
    variables: scope.variables
      .map((v) => variableIds.get(v) ?? null)
      .filter((x): x is string => x !== null),
    references: scope.references
      .map((r) => referenceIds.get(r) ?? null)
      .filter((x): x is string => x !== null),
    through: scope.through
      .map((r) => referenceIds.get(r) ?? null)
      .filter((x): x is string => x !== null),
    functionExpressionScope: scope.functionExpressionScope,
    blockContext: scope.unsnarlBlockContext ?? null,
    fallsThrough: scope.unsnarlFallsThrough ?? false,
    exitsFunction: scope.unsnarlExitsFunction ?? false,
  };
}
