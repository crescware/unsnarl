import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { ScopeImpl } from "../scope.js";
import { handleImportDeclaration } from "./handle-import-declaration.js";
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
    block: { type: "Program" } as unknown as AstNode,
  });

describe("handleImportDeclaration", () => {
  test("default specifier declares the local name", () => {
    const scope = newScope();
    handleImportDeclaration(firstStmt("import a from 'mod';"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["a"]);
    expect(scope.variables[0]?.defs[0]?.type).toBe("ImportBinding");
  });

  test("named specifiers declare each local name", () => {
    const scope = newScope();
    handleImportDeclaration(
      firstStmt("import { a, b as c } from 'mod';"),
      scope,
    );
    expect(scope.variables.map((v) => v.name)).toEqual(["a", "c"]);
  });

  test("namespace specifier declares the alias", () => {
    const scope = newScope();
    handleImportDeclaration(firstStmt("import * as ns from 'mod';"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["ns"]);
  });

  test("missing specifiers array is a no-op", () => {
    const scope = newScope();
    handleImportDeclaration({ type: "ImportDeclaration" }, scope);
    expect(scope.variables).toEqual([]);
  });
});
