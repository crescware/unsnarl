import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { isPatternStep } from "./is-pattern-step.js";

const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("isPatternStep", () => {
  test("ObjectPattern / ArrayPattern / RestElement / AssignmentPattern → true", () => {
    for (const t of ["ObjectPattern", "ArrayPattern", "RestElement", "AssignmentPattern"]) {
      expect(isPatternStep(node(t), [], 0)).toBe(true);
    }
  });

  test("Property is a pattern step only when its parent (path[i-1]) is ObjectPattern", () => {
    const path: PathEntry[] = [
      { node: node("ObjectPattern"), key: null },
      { node: node("Property"), key: "properties" },
    ];
    expect(isPatternStep(node("Property"), path, 1)).toBe(true);
  });

  test("Property under a non-ObjectPattern parent → false", () => {
    const path: PathEntry[] = [
      { node: node("ObjectExpression"), key: null },
      { node: node("Property"), key: "properties" },
    ];
    expect(isPatternStep(node("Property"), path, 1)).toBe(false);
  });

  test("unrelated node types → false", () => {
    expect(isPatternStep(node("Identifier"), [], 0)).toBe(false);
    expect(isPatternStep(node("CallExpression"), [], 0)).toBe(false);
  });
});
