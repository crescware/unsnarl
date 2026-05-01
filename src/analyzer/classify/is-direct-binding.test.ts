import { describe, expect, test } from "vitest";

import { isDirectBinding } from "./is-direct-binding.js";

describe("isDirectBinding", () => {
  test("VariableDeclarator#id", () => {
    expect(isDirectBinding("VariableDeclarator", "id")).toBe(true);
    expect(isDirectBinding("VariableDeclarator", "init")).toBe(false);
  });

  test("FunctionDeclaration/Expression #id", () => {
    expect(isDirectBinding("FunctionDeclaration", "id")).toBe(true);
    expect(isDirectBinding("FunctionExpression", "id")).toBe(true);
  });

  test("ClassDeclaration/Expression #id", () => {
    expect(isDirectBinding("ClassDeclaration", "id")).toBe(true);
    expect(isDirectBinding("ClassExpression", "id")).toBe(true);
  });

  test("Function/ArrowFunction #params", () => {
    expect(isDirectBinding("FunctionDeclaration", "params")).toBe(true);
    expect(isDirectBinding("FunctionExpression", "params")).toBe(true);
    expect(isDirectBinding("ArrowFunctionExpression", "params")).toBe(true);
  });

  test("CatchClause#param", () => {
    expect(isDirectBinding("CatchClause", "param")).toBe(true);
    expect(isDirectBinding("CatchClause", "body")).toBe(false);
  });

  test("ImportSpecifier-family #local", () => {
    expect(isDirectBinding("ImportSpecifier", "local")).toBe(true);
    expect(isDirectBinding("ImportDefaultSpecifier", "local")).toBe(true);
    expect(isDirectBinding("ImportNamespaceSpecifier", "local")).toBe(true);
  });

  test("unrelated cases → false", () => {
    expect(isDirectBinding("CallExpression", "callee")).toBe(false);
    expect(isDirectBinding("Identifier", null)).toBe(false);
  });
});
