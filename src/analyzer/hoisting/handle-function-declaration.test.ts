import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { SCOPE_TYPE } from "../scope-type.js";
import { ScopeImpl } from "../scope.js";
import { handleFunctionDeclaration } from "./handle-function-declaration.js";
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

describe("handleFunctionDeclaration", () => {
  test("declares the function name with type FunctionName", () => {
    const scope = newScope();
    handleFunctionDeclaration(firstStmt("function f() {}"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["f"]);
    expect(scope.variables[0]?.defs[0]?.type).toBe(
      DEFINITION_TYPE.FunctionName,
    );
  });

  test("anonymous function (no id) declares nothing", () => {
    const scope = newScope();
    handleFunctionDeclaration(
      {
        type: AST_TYPE.FunctionDeclaration,
        id: null,
        params: [],
        body: { type: AST_TYPE.BlockStatement, body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });

  test("non-Identifier id is ignored", () => {
    const scope = newScope();
    handleFunctionDeclaration(
      {
        type: AST_TYPE.FunctionDeclaration,
        id: { type: AST_TYPE.Literal, value: "x" },
        params: [],
        body: { type: AST_TYPE.BlockStatement, body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });
});
