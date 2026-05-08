import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";
import type { PathEntry } from "./walk/path-entry.js";

export function enterSwitch(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
  visitor: AnalysisVisitor,
): void {
  const scope = manager.push("switch", node as unknown as AstNode);
  visitor.onScope?.({ scope, parent, key, path });
}
