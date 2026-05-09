import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { serializeDefinition } from "./serialize-definition.js";
import { spanOf } from "./span-of.js";

export function serializeVariable(
  variable: Variable,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedVariable {
  const id = variableIds.get(variable) ?? null;
  if (id === null) {
    throw new Error("Variable id not found");
  }
  return {
    id,
    name: variable.name,
    scope: scopeIds.get(variable.scope) ?? "",
    identifiers: variable.identifiers.map((v) => spanOf(v, raw)),
    references: variable.references
      .map((v) => referenceIds.get(v) ?? null)
      .filter((v): v is string => v !== null),
    defs: variable.defs.map((v) => serializeDefinition(v, raw)),
  };
}
