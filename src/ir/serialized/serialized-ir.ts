import type { Language } from "../../cli/language.js";
import type { SerializedIRVersion } from "../../serializer/serialized-ir-version.js";
import type { Diagnostic } from "../diagnostic/diagnostic.js";
import type { VariableId } from "./ids.js";
import type { SerializedReference } from "./serialized-reference.js";
import type { SerializedScope } from "./serialized-scope.js";
import type { SerializedVariable } from "./serialized-variable.js";

export type SerializedIR = Readonly<{
  version: SerializedIRVersion;
  source: Readonly<{ path: string; language: Language }>;
  raw: string;
  scopes: readonly SerializedScope[];
  variables: readonly SerializedVariable[];
  references: readonly SerializedReference[];
  unusedVariableIds: readonly VariableId[];
  diagnostics: readonly Diagnostic[];
}>;
