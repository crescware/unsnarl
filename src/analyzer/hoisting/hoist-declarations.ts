import type { Scope } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { isNodeLike } from "./node-like.js";
import { visit } from "./visit.js";

export function hoistDeclarations(
  body: ReadonlyArray<unknown>,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  for (const stmt of body) {
    if (!isNodeLike(stmt)) {
      continue;
    }
    visit(stmt, scope, raw, diagnostics);
  }
}
