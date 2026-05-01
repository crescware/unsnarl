import { parseSync } from "oxc-parser";
import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { ScopeImpl } from "../scope.js";
import { handleFunctionDeclaration } from "./handle-function-declaration.js";
import type { NodeLike } from "./node-like.js";

const firstStmt = (code: string): NodeLike => {
  const program = parseSync("input.ts", code, { lang: "ts" })
    .program as unknown as {
    body: ReadonlyArray<NodeLike>;
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

describe("handleFunctionDeclaration", () => {
  test("declares the function name with type FunctionName", () => {
    const scope = newScope();
    handleFunctionDeclaration(firstStmt("function f() {}"), scope);
    expect(scope.variables.map((v) => v.name)).toEqual(["f"]);
    expect(scope.variables[0]?.defs[0]?.type).toBe("FunctionName");
  });

  test("anonymous function (no id) declares nothing", () => {
    const scope = newScope();
    handleFunctionDeclaration(
      {
        type: "FunctionDeclaration",
        id: null,
        params: [],
        body: { type: "BlockStatement", body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });

  test("non-Identifier id is ignored", () => {
    const scope = newScope();
    handleFunctionDeclaration(
      {
        type: "FunctionDeclaration",
        id: { type: "Literal", value: "x" },
        params: [],
        body: { type: "BlockStatement", body: [] },
      },
      scope,
    );
    expect(scope.variables).toEqual([]);
  });
});
