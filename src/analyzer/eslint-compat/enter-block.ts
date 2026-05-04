import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { hoistDeclarations } from "../hoisting/hoist-declarations.js";
import type { ScopeManager } from "../manager.js";
import type { PathEntry } from "../walk/path-entry.js";
import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";

export function enterBlock(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const ctx = blockContextOf(parent, key, path);
  const scope = manager.push("block", node as unknown as AstNode, ctx);
  const stmts = node["body"];
  if (Array.isArray(stmts)) {
    hoistDeclarations(stmts, scope, raw, diagnostics);
  }
}
