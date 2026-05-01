import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { ScopeManager } from "../manager.js";
import { handleLeave } from "./handle-leave.js";
import type { NodeLike } from "./node-like.js";
import { parse } from "./testing/parse.js";

function freshManager(): ScopeManager {
  const program = parse("");
  return new ScopeManager("module", program as unknown as AstNode);
}

describe("handleLeave", () => {
  test.each([
    AST_TYPE.FunctionDeclaration,
    AST_TYPE.FunctionExpression,
    AST_TYPE.ArrowFunctionExpression,
    AST_TYPE.ForStatement,
    "ForOfStatement",
    "ForInStatement",
    AST_TYPE.SwitchStatement,
    AST_TYPE.SwitchCase,
    AST_TYPE.CatchClause,
  ])("pops the current scope for type=%s", (type) => {
    const manager = freshManager();
    const block = { type } as const satisfies NodeLike;
    manager.push("function", block as unknown as AstNode);
    const before = manager.current();

    handleLeave(block, null, null, manager);

    expect(manager.current()).not.toBe(before);
    expect(manager.current().type).toBe("module");
  });

  test("BlockStatement under FunctionDeclaration does NOT pop", () => {
    const manager = freshManager();
    const block = { type: AST_TYPE.BlockStatement } as const satisfies NodeLike;
    const before = manager.current();
    const parent = {
      type: AST_TYPE.FunctionDeclaration,
    } as const satisfies NodeLike;

    handleLeave(block, parent, "body", manager);

    expect(manager.current()).toBe(before);
  });

  test("plain BlockStatement (not under fn/catch) pops the current scope", () => {
    const manager = freshManager();
    manager.push("block", {
      type: AST_TYPE.BlockStatement,
    } as unknown as AstNode);
    const block = { type: AST_TYPE.BlockStatement } as const satisfies NodeLike;
    const parent = { type: AST_TYPE.IfStatement } as const satisfies NodeLike;

    handleLeave(block, parent, "consequent", manager);

    expect(manager.current().type).toBe("module");
  });

  test("unknown node types do nothing", () => {
    const manager = freshManager();
    const before = manager.current();

    handleLeave({ type: AST_TYPE.ExpressionStatement }, null, null, manager);

    expect(manager.current()).toBe(before);
  });
});
