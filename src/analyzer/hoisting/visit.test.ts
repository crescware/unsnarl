import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import { AST_TYPE, SCOPE_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeImpl } from "../scope.js";
import type { NodeLike } from "./node-like.js";
import { visit } from "./visit.js";

const firstStmt = (code: string): NodeLike => {
  const program = parseSync("input.ts", code, { lang: "ts" })
    .program as unknown as {
    body: readonly NodeLike[];
  };
  const stmt = program.body[0];
  if (stmt === undefined) {
    throw new Error("test fixture missing first statement");
  }
  return stmt;
};

const newScope = (): ScopeImpl =>
  new ScopeImpl({
    type: SCOPE_TYPE.Module,
    isStrict: true,
    upper: null,
    block: { type: AST_TYPE.Program } as unknown as AstNode,
  });

describe("visit dispatch", () => {
  test("VariableDeclaration → handleVariableDeclaration", () => {
    const scope = newScope();
    visit(firstStmt("const x = 1;"), scope, "", new DiagnosticCollector());
    expect(scope.variables.map((v) => v.name)).toEqual(["x"]);
  });

  test("FunctionDeclaration → handleFunctionDeclaration", () => {
    const scope = newScope();
    visit(firstStmt("function f() {}"), scope, "", new DiagnosticCollector());
    expect(scope.variables.map((v) => v.name)).toEqual(["f"]);
  });

  test("ClassDeclaration → handleClassDeclaration", () => {
    const scope = newScope();
    visit(firstStmt("class C {}"), scope, "", new DiagnosticCollector());
    expect(scope.variables.map((v) => v.name)).toEqual(["C"]);
  });

  test("ImportDeclaration → handleImportDeclaration", () => {
    const scope = newScope();
    visit(
      firstStmt("import a from 'mod';"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["a"]);
  });

  test("ExportNamedDeclaration unwraps and recurses into declaration", () => {
    const scope = newScope();
    visit(
      firstStmt("export const x = 1;"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["x"]);
  });

  test("ExportNamedDeclaration without inner declaration is a no-op", () => {
    const scope = newScope();
    visit(firstStmt("export { foo };"), scope, "", new DiagnosticCollector());
    expect(scope.variables).toEqual([]);
  });

  test("ExportDefaultDeclaration unwraps FunctionDeclaration", () => {
    const scope = newScope();
    visit(
      firstStmt("export default function f() {}"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["f"]);
  });

  test("ExportDefaultDeclaration unwraps ClassDeclaration", () => {
    const scope = newScope();
    visit(
      firstStmt("export default class C {}"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["C"]);
  });

  test("ExportDefaultDeclaration of an expression is a no-op", () => {
    const scope = newScope();
    visit(
      firstStmt("export default 42;"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables).toEqual([]);
  });

  test("unrelated statement type is silently ignored", () => {
    const scope = newScope();
    visit(firstStmt("foo();"), scope, "", new DiagnosticCollector());
    expect(scope.variables).toEqual([]);
  });
});
