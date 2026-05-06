import type { Diagnostic } from "../ir/diagnostic/diagnostic.js";

// Returns the lines a var-detected diagnostic consists of, sharing the
// exact wording between the stderr emitter and the markdown Notice
// section so both stay in lock-step.
export function formatVarDiagnostic(diagnostic: Diagnostic): string[] {
  const { line, column } = diagnostic.span;
  return [`uns: warning: L${line}:${column}: ${diagnostic.message}`];
}
