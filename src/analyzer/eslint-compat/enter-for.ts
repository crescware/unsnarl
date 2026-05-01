import type { AstNode } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import type { ScopeManager } from "../manager.js";
import { blockContextOf } from "./block-context-of.js";
import { declareForLeft } from "./declare-for-left.js";
import type { NodeLike } from "./node-like.js";

export function enterFor(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const ctx = blockContextOf(parent, key);
  const scope = manager.push("for", node as unknown as AstNode, ctx);
  declareForLeft(node, scope, raw, diagnostics);
}
