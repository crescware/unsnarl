import { parse } from "valibot";

import type { Annotations } from "../../ir/annotations/annotations.js";
import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { ReferenceId } from "../../ir/serialized/reference-id.js";
import type { ScopeId } from "../../ir/serialized/scope-id.js";
import {
  serializedScope$,
  type SerializedScope,
} from "../../ir/serialized/serialized-scope.js";
import type { VariableId } from "../../ir/serialized/variable-id.js";
import { spanFromOffset } from "../../util/span.js";
import { spanOf } from "./span-of.js";

export function serializeScope(
  scope: Scope,
  scopeIds: Map<Scope, ScopeId>,
  variableIds: Map<Variable, VariableId>,
  referenceIds: Map<Reference, ReferenceId>,
  annotations: Annotations,
  raw: string,
): SerializedScope {
  const id = scopeIds.get(scope) ?? null;
  if (id === null) {
    throw new Error("Scope id not found");
  }
  const ann = annotations.ofScope(scope);
  return parse(serializedScope$, {
    id,
    type: scope.type,
    isStrict: scope.isStrict,
    upper: scope.upper ? (scopeIds.get(scope.upper) ?? null) : null,
    childScopes: scope.childScopes
      .map((v) => scopeIds.get(v) ?? null)
      .filter((v): v is ScopeId => v !== null),
    variableScope: scopeIds.get(scope.variableScope) ?? id,
    block: {
      type: scope.block.type,
      span: spanOf(scope.block, raw),
      endSpan: spanFromOffset(raw, scope.block.end ?? scope.block.start ?? 0),
    },
    variables: scope.variables
      .map((v) => variableIds.get(v) ?? null)
      .filter((v): v is VariableId => v !== null),
    references: scope.references
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is ReferenceId => v !== null),
    through: scope.through
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is ReferenceId => v !== null),
    functionExpressionScope: scope.functionExpressionScope,
    blockContext: ann.blockContext,
    fallsThrough: ann.fallsThrough,
    exitsFunction: ann.exitsFunction,
    nestingDepths: ann.nestingDepths,
  });
}
