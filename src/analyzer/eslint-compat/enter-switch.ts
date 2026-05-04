import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { ScopeManager } from "../manager.js";
import type { PathEntry } from "../walk/walk.js";
import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";

export function enterSwitch(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
  manager: ScopeManager,
): void {
  const ctx = blockContextOf(parent, key, path);
  manager.push("switch", node as unknown as AstNode, ctx);
}
