import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { isDirectBinding } from "./is-direct-binding.js";

describe("isDirectBinding", () => {
  test("VariableDeclarator#id", () => {
    expect(isDirectBinding(AST_TYPE.VariableDeclarator, "id")).toBe(true);
    expect(isDirectBinding(AST_TYPE.VariableDeclarator, "init")).toBe(false);
  });

  test("FunctionDeclaration/Expression #id", () => {
    expect(isDirectBinding(AST_TYPE.FunctionDeclaration, "id")).toBe(true);
    expect(isDirectBinding(AST_TYPE.FunctionExpression, "id")).toBe(true);
  });

  test("ClassDeclaration/Expression #id", () => {
    expect(isDirectBinding(AST_TYPE.ClassDeclaration, "id")).toBe(true);
    expect(isDirectBinding(AST_TYPE.ClassExpression, "id")).toBe(true);
  });

  test("Function/ArrowFunction #params", () => {
    expect(isDirectBinding(AST_TYPE.FunctionDeclaration, "params")).toBe(true);
    expect(isDirectBinding(AST_TYPE.FunctionExpression, "params")).toBe(true);
    expect(isDirectBinding(AST_TYPE.ArrowFunctionExpression, "params")).toBe(
      true,
    );
  });

  test("CatchClause#param", () => {
    expect(isDirectBinding(AST_TYPE.CatchClause, "param")).toBe(true);
    expect(isDirectBinding(AST_TYPE.CatchClause, "body")).toBe(false);
  });

  test("ImportSpecifier-family #local", () => {
    expect(isDirectBinding(AST_TYPE.ImportSpecifier, "local")).toBe(true);
    expect(isDirectBinding(AST_TYPE.ImportDefaultSpecifier, "local")).toBe(
      true,
    );
    expect(isDirectBinding(AST_TYPE.ImportNamespaceSpecifier, "local")).toBe(
      true,
    );
  });

  test("unrelated cases → false", () => {
    expect(isDirectBinding(AST_TYPE.CallExpression, "callee")).toBe(false);
    expect(isDirectBinding(AST_TYPE.Identifier, null)).toBe(false);
  });
});
