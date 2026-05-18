import type { Scope } from "../../ir/scope/scope.js";

export type EslintScopeAnalysisResult = Readonly<{
  globalScope: Scope;
}>;
