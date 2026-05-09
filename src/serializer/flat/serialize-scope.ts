import type { Annotations } from "../../ir/annotations/annotations.js";
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
  annotations: Annotations,
  raw: string,
): SerializedScope {
  const id = scopeIds.get(scope) ?? null;
  if (id === null) {
    throw new Error("Scope id not found");
  }
  const ann = annotations.ofScope(scope);
  return {
    id,
    type: scope.type,
    isStrict: scope.isStrict,
    upper: scope.upper ? (scopeIds.get(scope.upper) ?? null) : null,
    childScopes: scope.childScopes
      .map((v) => scopeIds.get(v) ?? null)
      .filter((v): v is string => v !== null),
    variableScope: scopeIds.get(scope.variableScope) ?? id,
    block: {
      type: scope.block.type,
      span: spanOf(scope.block, raw),
      endSpan: spanFromOffset(raw, scope.block.end ?? scope.block.start ?? 0),
    },
    variables: scope.variables
      .map((v) => variableIds.get(v) ?? null)
      .filter((v): v is string => v !== null),
    references: scope.references
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is string => v !== null),
    through: scope.through
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is string => v !== null),
    functionExpressionScope: scope.functionExpressionScope,
    blockContext: ann.blockContext,
    fallsThrough: ann.fallsThrough,
    exitsFunction: ann.exitsFunction,
    nestingDepths: ann.nestingDepths,
  };
}
