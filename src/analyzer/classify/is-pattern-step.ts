import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";

export function isPatternStep(
  node: AstNode,
  path: readonly PathEntry[],
  i: number,
): boolean {
  const t = node.type;
  if (
    t === "ObjectPattern" ||
    t === "ArrayPattern" ||
    t === "RestElement" ||
    t === "AssignmentPattern"
  ) {
    return true;
  }
  if (t === "Property") {
    return path[i - 1]?.node.type === "ObjectPattern";
  }
  return false;
}
