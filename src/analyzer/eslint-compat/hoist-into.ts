import type { Scope } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { hoistDeclarations } from "../hoisting/hoist-declarations.js";
import type { NodeLike } from "./node-like.js";

export function hoistInto(
  program: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const body = program["body"];
  if (Array.isArray(body)) {
    hoistDeclarations(body, scope, raw, diagnostics);
  }
}
