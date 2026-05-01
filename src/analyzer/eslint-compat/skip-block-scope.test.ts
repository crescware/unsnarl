import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { skipBlockScope } from "./skip-block-scope.js";

describe("skipBlockScope", () => {
  test.each([
    { parentType: AST_TYPE.FunctionDeclaration, expected: true },
    { parentType: AST_TYPE.FunctionExpression, expected: true },
    { parentType: AST_TYPE.ArrowFunctionExpression, expected: true },
    { parentType: AST_TYPE.CatchClause, expected: true },
    { parentType: AST_TYPE.IfStatement, expected: false },
    { parentType: AST_TYPE.ForStatement, expected: false },
    { parentType: AST_TYPE.BlockStatement, expected: false },
    { parentType: AST_TYPE.Program, expected: false },
    { parentType: "", expected: false },
  ])("parentType=$parentType -> $expected", ({ parentType, expected }) => {
    expect(skipBlockScope(parentType)).toBe(expected);
  });
});
