import type { Language } from "../../language.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { Scope } from "../../ir/scope/scope.js";

type SourceMeta = Readonly<{
  path: string;
  language: Language;
}>;

export type SerializeContext = Readonly<{
  rootScope: Scope;
  source: SourceMeta;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;
