import type {
  Reference,
  Scope,
  SerializedScope,
  Variable,
} from "../../ir/model.js";
import { spanFromOffset } from "../../util/span.js";
import { spanOf } from "./span-of.js";

export function serializeScope(
  scope: Scope,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedScope {
  const id = scopeIds.get(scope);
  if (id === undefined) {
    throw new Error("Scope id not found");
  }
  return {
    id,
    type: scope.type,
    isStrict: scope.isStrict,
    upper: scope.upper ? (scopeIds.get(scope.upper) ?? null) : null,
    childScopes: scope.childScopes
      .map((c) => scopeIds.get(c))
      .filter((x): x is string => x !== undefined),
    variableScope: scopeIds.get(scope.variableScope) ?? id,
    block: {
      type: scope.block.type,
      span: spanOf(scope.block, raw),
      endSpan: spanFromOffset(raw, scope.block.end ?? scope.block.start ?? 0),
    },
    variables: scope.variables
      .map((v) => variableIds.get(v))
      .filter((x): x is string => x !== undefined),
    references: scope.references
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    through: scope.through
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    functionExpressionScope: scope.functionExpressionScope,
    blockContext: scope.unsnarlBlockContext ?? null,
    fallsThrough: scope.unsnarlFallsThrough ?? false,
    exitsFunction: scope.unsnarlExitsFunction ?? false,
  };
}
