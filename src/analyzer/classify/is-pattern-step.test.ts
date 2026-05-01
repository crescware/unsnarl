import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { isPatternStep } from "./is-pattern-step.js";

const node = (type: string): AstNode => ({ type }) as unknown as AstNode;

describe("isPatternStep", () => {
  test("ObjectPattern / ArrayPattern / RestElement / AssignmentPattern → true", () => {
    for (const t of [
      AST_TYPE.ObjectPattern,
      AST_TYPE.ArrayPattern,
      AST_TYPE.RestElement,
      AST_TYPE.AssignmentPattern,
    ]) {
      expect(isPatternStep(node(t), [], 0)).toBe(true);
    }
  });

  test("Property is a pattern step only when its parent (path[i-1]) is ObjectPattern", () => {
    const path: readonly PathEntry[] = [
      { node: node(AST_TYPE.ObjectPattern), key: null },
      { node: node(AST_TYPE.Property), key: "properties" },
    ];
    expect(isPatternStep(node(AST_TYPE.Property), path, 1)).toBe(true);
  });

  test("Property under a non-ObjectPattern parent → false", () => {
    const path: readonly PathEntry[] = [
      { node: node(AST_TYPE.ObjectExpression), key: null },
      { node: node(AST_TYPE.Property), key: "properties" },
    ];
    expect(isPatternStep(node(AST_TYPE.Property), path, 1)).toBe(false);
  });

  test("unrelated node types → false", () => {
    expect(isPatternStep(node(AST_TYPE.Identifier), [], 0)).toBe(false);
    expect(isPatternStep(node(AST_TYPE.CallExpression), [], 0)).toBe(false);
  });
});
