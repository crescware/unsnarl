import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { NodeLike } from "./node-like.js";

type ReferenceVisitInput = Readonly<{
  ref: Reference;
  parent: NodeLike | null;
  key: string | null;
  path: readonly PathEntry[];
  scope: Scope;
}>;

export type ScopeVisitInput = Readonly<{
  scope: Scope;
  parent: NodeLike | null;
  key: string | null;
  path: readonly PathEntry[];
}>;

export type AnalysisVisitor = Readonly<{
  onReference?(input: ReferenceVisitInput): void;
  onScope?(input: ScopeVisitInput): void;
  onDiagnostic?(diag: Diagnostic): void;
}>;
