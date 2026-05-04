import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { serializeDefinition } from "./serialize-definition.js";
import { spanOf } from "./span-of.js";

export function serializeVariable(
  v: Variable,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedVariable {
  const id = variableIds.get(v);
  if (id === undefined) {
    throw new Error("Variable id not found");
  }
  return {
    id,
    name: v.name,
    scope: scopeIds.get(v.scope) ?? "",
    identifiers: v.identifiers.map((i) => spanOf(i, raw)),
    references: v.references
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    defs: v.defs.map((d) => serializeDefinition(d, raw)),
  };
}
