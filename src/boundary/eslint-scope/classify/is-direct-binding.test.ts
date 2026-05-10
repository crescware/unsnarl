import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../../parser/ast-type.js";
import { isDirectBinding } from "./is-direct-binding.js";

describe("isDirectBinding", () => {
  test("VariableDeclarator#id", () => {
    expect(isDirectBinding(AST_TYPE.VariableDeclarator, "id")).toEqual(true);
    expect(isDirectBinding(AST_TYPE.VariableDeclarator, "init")).toEqual(false);
  });

  test("FunctionDeclaration/Expression #id", () => {
    expect(isDirectBinding(AST_TYPE.FunctionDeclaration, "id")).toEqual(true);
    expect(isDirectBinding(AST_TYPE.FunctionExpression, "id")).toEqual(true);
  });

  test("ClassDeclaration/Expression #id", () => {
    expect(isDirectBinding(AST_TYPE.ClassDeclaration, "id")).toEqual(true);
    expect(isDirectBinding(AST_TYPE.ClassExpression, "id")).toEqual(true);
  });

  test("Function/ArrowFunction #params", () => {
    expect(isDirectBinding(AST_TYPE.FunctionDeclaration, "params")).toEqual(
      true,
    );
    expect(isDirectBinding(AST_TYPE.FunctionExpression, "params")).toEqual(
      true,
    );
    expect(isDirectBinding(AST_TYPE.ArrowFunctionExpression, "params")).toEqual(
      true,
    );
  });

  test("CatchClause#param", () => {
    expect(isDirectBinding(AST_TYPE.CatchClause, "param")).toEqual(true);
    expect(isDirectBinding(AST_TYPE.CatchClause, "body")).toEqual(false);
  });

  test("ImportSpecifier-family #local", () => {
    expect(isDirectBinding(AST_TYPE.ImportSpecifier, "local")).toEqual(true);
    expect(isDirectBinding(AST_TYPE.ImportDefaultSpecifier, "local")).toEqual(
      true,
    );
    expect(isDirectBinding(AST_TYPE.ImportNamespaceSpecifier, "local")).toEqual(
      true,
    );
  });

  test("unrelated cases → false", () => {
    expect(isDirectBinding(AST_TYPE.CallExpression, "callee")).toEqual(false);
    expect(isDirectBinding(AST_TYPE.Identifier, null)).toEqual(false);
  });
});
