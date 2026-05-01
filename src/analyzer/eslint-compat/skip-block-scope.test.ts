import { describe, expect, test } from "vitest";

import { skipBlockScope } from "./skip-block-scope.js";

describe("skipBlockScope", () => {
  test.each([
    { parentType: "FunctionDeclaration", expected: true },
    { parentType: "FunctionExpression", expected: true },
    { parentType: "ArrowFunctionExpression", expected: true },
    { parentType: "CatchClause", expected: true },
    { parentType: "IfStatement", expected: false },
    { parentType: "ForStatement", expected: false },
    { parentType: "BlockStatement", expected: false },
    { parentType: "Program", expected: false },
    { parentType: "", expected: false },
  ])("parentType=$parentType -> $expected", ({ parentType, expected }) => {
    expect(skipBlockScope(parentType)).toBe(expected);
  });
});
