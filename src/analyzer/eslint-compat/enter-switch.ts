import type { AstNode } from "../../ir/model.js";
import type { ScopeManager } from "../manager.js";
import { blockContextOf } from "./block-context-of.js";
import type { NodeLike } from "./node-like.js";

export function enterSwitch(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
): void {
  const ctx = blockContextOf(parent, key);
  manager.push("switch", node as unknown as AstNode, ctx);
}
