import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { DIAGNOSTIC_KIND } from "../diagnostic-kind.js";
import { SCOPE_TYPE } from "../scope-type.js";
import { ScopeImpl } from "../scope.js";
import { handleVariableDeclaration } from "./handle-variable-declaration.js";
import type { NodeLike } from "./node-like.js";

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
    blockContext: null,
  });

describe("handleVariableDeclaration", () => {
  test("const declares each binding identifier", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    handleVariableDeclaration(
      firstStmt("const x = 1, { y, z } = obj;"),
      scope,
      "",
      diags,
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["x", "y", "z"]);
    expect(diags.list()).toEqual([]);
  });

  test("let declares each binding identifier", () => {
    const scope = newScope();
    handleVariableDeclaration(
      firstStmt("let a, b;"),
      scope,
      "",
      new DiagnosticCollector(),
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["a", "b"]);
  });

  test("var emits a var-detected diagnostic and declares nothing", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    const raw = "var x = 1;";
    handleVariableDeclaration(firstStmt(raw), scope, raw, diags);
    expect(scope.variables).toEqual([]);
    const items = diags.list();
    expect(items).toHaveLength(1);
    expect(items[0]?.kind).toBe(DIAGNOSTIC_KIND.VarDetected);
  });

  test("non-var/const/let kind is silently ignored", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    handleVariableDeclaration(
      { type: AST_TYPE.VariableDeclaration, kind: "using", declarations: [] },
      scope,
      "",
      diags,
    );
    expect(scope.variables).toEqual([]);
    expect(diags.list()).toEqual([]);
  });

  test("missing or non-array declarations is a no-op", () => {
    const scope = newScope();
    const diags = new DiagnosticCollector();
    handleVariableDeclaration(
      { type: AST_TYPE.VariableDeclaration, kind: "const" },
      scope,
      "",
      diags,
    );
    expect(scope.variables).toEqual([]);
  });
});
