import type { Span } from "../primitive/span.js";
import type { ReferenceId, ScopeId, VariableId } from "./ids.js";
import type { SerializedDefinition } from "./serialized-definition.js";

export type SerializedVariable = Readonly<{
  id: VariableId;
  name: string;
  scope: ScopeId;
  identifiers: readonly Span[];
  references: readonly ReferenceId[];
  defs: readonly SerializedDefinition[];
}>;
