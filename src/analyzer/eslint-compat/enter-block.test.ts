import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { enterBlock } from "./enter-block.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
describe("enterBlock", () => {
  test("pushes a block scope with the given blockContext and hoists body declarations", () => {
    const code = "if (x) { let y = 1; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 5,
    } as const satisfies NodeLike;
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterBlock(block, parent, "consequent", manager, code, diagnostics);

    const scope = manager.current();
    expect(scope.type).toBe("block");
    expect(scope.unsnarlBlockContext).toEqual({
      parentType: AST_TYPE.IfStatement,
      key: "consequent",
      parentSpanOffset: 5,
    });
    expect(scope.variables.map((v) => v.name)).toEqual(["y"]);
  });

  test("blockContext is null when parent is null", () => {
    const code = "{ let z = 2; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterBlock(block, null, null, manager, code, diagnostics);

    expect(manager.current().unsnarlBlockContext).toBeNull();
  });

  test("does not hoist when body is missing", () => {
    const block = { type: AST_TYPE.BlockStatement } as const satisfies NodeLike;
    const program = parse("");
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterBlock(block, null, null, manager, "", diagnostics);

    expect(manager.current().variables).toHaveLength(0);
  });
});
