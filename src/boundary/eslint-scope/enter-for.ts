import type { ScopeManager } from "../../analyzer/manager.js";
import type { PathEntry } from "../../analyzer/walk/path-entry.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { declareForLeft } from "./declare-for-left.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";

export function enterFor(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("for", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
  declareForLeft(node, scope, raw, diagnostics);
}
