import type { Annotations } from "../../ir/annotations/annotations.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { Scope } from "../../ir/scope/scope.js";

export type AnalyzedSource = Readonly<{
  rootScope: Scope;
  annotations: Annotations;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;
