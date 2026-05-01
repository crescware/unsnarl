import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
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
    "FunctionDeclaration",
    "FunctionExpression",
    "ArrowFunctionExpression",
    "ForStatement",
    "ForOfStatement",
    "ForInStatement",
    "SwitchStatement",
    "SwitchCase",
    "CatchClause",
  ])("pops the current scope for type=%s", (type) => {
    const manager = freshManager();
    const block: NodeLike = { type };
    manager.push("function", block as unknown as AstNode);
    const before = manager.current();

    handleLeave(block, null, null, manager);

    expect(manager.current()).not.toBe(before);
    expect(manager.current().type).toBe("module");
  });

  test("BlockStatement under FunctionDeclaration does NOT pop", () => {
    const manager = freshManager();
    const block: NodeLike = { type: "BlockStatement" };
    const before = manager.current();
    const parent: NodeLike = { type: "FunctionDeclaration" };

    handleLeave(block, parent, "body", manager);

    expect(manager.current()).toBe(before);
  });

  test("plain BlockStatement (not under fn/catch) pops the current scope", () => {
    const manager = freshManager();
    manager.push("block", { type: "BlockStatement" } as unknown as AstNode);
    const block: NodeLike = { type: "BlockStatement" };
    const parent: NodeLike = { type: "IfStatement" };

    handleLeave(block, parent, "consequent", manager);

    expect(manager.current().type).toBe("module");
  });

  test("unknown node types do nothing", () => {
    const manager = freshManager();
    const before = manager.current();

    handleLeave({ type: "ExpressionStatement" }, null, null, manager);

    expect(manager.current()).toBe(before);
  });
});
