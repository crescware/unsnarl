import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { Scope } from "../../ir/scope/scope.js";

export type AnalyzedSource = Readonly<{
  rootScope: Scope;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;
