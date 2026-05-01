export const DIAGNOSTIC_KIND = {
  VarDetected: "var-detected",
  UnresolvedIdentifier: "unresolved-identifier",
  ParseError: "parse-error",
} as const;
export type DiagnosticKind =
  (typeof DIAGNOSTIC_KIND)[keyof typeof DIAGNOSTIC_KIND];
