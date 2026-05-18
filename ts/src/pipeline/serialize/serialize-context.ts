import type { Annotations } from "../../ir/annotations/annotations.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Language } from "../../language.js";

type SourceMeta = Readonly<{
  path: string;
  language: Language;
}>;

export type SerializeContext = Readonly<{
  rootScope: Scope;
  annotations: Annotations;
  source: SourceMeta;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;
