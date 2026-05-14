import { parse } from "valibot";

import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { ReferenceId } from "../../ir/serialized/reference-id.js";
import type { ScopeId } from "../../ir/serialized/scope-id.js";
import {
  serializedVariable$,
  type SerializedVariable,
} from "../../ir/serialized/serialized-variable.js";
import type { VariableId } from "../../ir/serialized/variable-id.js";
import { serializeDefinition } from "./serialize-definition.js";
import { spanOf } from "./span-of.js";

export function serializeVariable(
  variable: Variable,
  scopeIds: Map<Scope, ScopeId>,
  variableIds: Map<Variable, VariableId>,
  referenceIds: Map<Reference, ReferenceId>,
  raw: string,
): SerializedVariable {
  const id = variableIds.get(variable) ?? null;
  if (id === null) {
    throw new Error("Variable id not found");
  }
  const scope = scopeIds.get(variable.scope) ?? null;
  if (scope === null) {
    throw new Error(`Scope id not found for variable ${variable.name}`);
  }
  return parse(serializedVariable$, {
    id,
    name: variable.name,
    scope,
    identifiers: variable.identifiers.map((v) => spanOf(v, raw)),
    references: variable.references
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is ReferenceId => v !== null),
    defs: variable.defs.map((v) => serializeDefinition(v, raw)),
  });
}
