import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { SCOPE_TYPE } from "../scope-type.js";
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
    type: SCOPE_TYPE.Module,
    isStrict: true,
    upper: null,
    block: { type: AST_TYPE.Program } as unknown as AstNode,
  });

describe("handleClassDeclaration", () => {
  test("declares the class name with type ClassName", () => {
    const scope = newScope();
    handleClassDeclaration(firstStmt("class C {}"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["C"]);
    expect(scope.variables[0]?.defs[0]?.type).toBe(DEFINITION_TYPE.ClassName);
  });

  test("anonymous class (no id) declares nothing", () => {
    const scope = newScope();
    handleClassDeclaration(
      {
        type: AST_TYPE.ClassDeclaration,
        id: null,
        body: { type: AST_TYPE.ClassBody, body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });
});
