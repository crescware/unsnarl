import { AST_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";

export function isPatternStep(
  node: AstNode,
  path: readonly PathEntry[],
  i: number,
): boolean {
  const t = node.type;
  if (
    t === AST_TYPE.ObjectPattern ||
    t === AST_TYPE.ArrayPattern ||
    t === AST_TYPE.RestElement ||
    t === AST_TYPE.AssignmentPattern
  ) {
    return true;
  }
  if (t === AST_TYPE.Property) {
    return path[i - 1]?.node.type === AST_TYPE.ObjectPattern;
  }
  return false;
}
