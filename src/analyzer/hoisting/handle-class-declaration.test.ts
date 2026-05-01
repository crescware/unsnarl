import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { ScopeImpl } from "../scope.js";
import { handleClassDeclaration } from "./handle-class-declaration.js";
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
    type: "module",
    isStrict: true,
    upper: null,
    block: { type: "Program" } as unknown as AstNode,
  });

describe("handleClassDeclaration", () => {
  test("declares the class name with type ClassName", () => {
    const scope = newScope();
    handleClassDeclaration(firstStmt("class C {}"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["C"]);
    expect(scope.variables[0]?.defs[0]?.type).toBe("ClassName");
  });

  test("anonymous class (no id) declares nothing", () => {
    const scope = newScope();
    handleClassDeclaration(
      {
        type: "ClassDeclaration",
        id: null,
        body: { type: "ClassBody", body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });
});
