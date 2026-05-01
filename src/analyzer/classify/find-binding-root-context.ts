import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { isPatternStep } from "./is-pattern-step.js";

export function findBindingRootContext(
  parent: AstNode | null,
  key: string | null,
  path: readonly PathEntry[],
): "var" | "param" | "catch" | "assign" | null {
  let curParent: AstNode | null = parent;
  let curKey = key;
  let i = path.length - 1;
  while (curParent) {
    const t = curParent.type;
    const isPattern = isPatternStep(curParent, path, i);
    if (!isPattern) {
      switch (t) {
        case "VariableDeclarator":
          return curKey === "id" ? "var" : null;
        case "CatchClause":
          return curKey === "param" ? "catch" : null;
        case "FunctionDeclaration":
        case "FunctionExpression":
        case "ArrowFunctionExpression":
          return curKey === "params" ? "param" : null;
        case "AssignmentExpression":
          return curKey === "left" ? "assign" : null;
        default:
          return null;
      }
    }
    i -= 1;
    if (i < 0) {
      return null;
    }
    const next = path[i];
    if (!next) {
      return null;
    }
    curParent = next.node;
    curKey = path[i + 1]?.key ?? null;
  }
  return null;
}
