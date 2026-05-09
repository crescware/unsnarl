import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { hoistDeclarations } from "./hoisting/hoist-declarations.js";
import type { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";
import type { PathEntry } from "./walk/path-entry.js";

export function enterBlock(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("block", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
  const stmts = node["body"];
  if (Array.isArray(stmts)) {
    hoistDeclarations(stmts, scope, raw, diagnostics);
  }
}
