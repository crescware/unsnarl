import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { ScopeManager } from "../manager.js";
import { enterSwitch } from "./enter-switch.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
describe("enterSwitch", () => {
  test("pushes a switch scope with the given blockContext", () => {
    const code = "switch (x) { case 1: break; }";
    const program = parse(code);
    const switchNode = findFirst(program, AST_TYPE.SwitchStatement);
    const parent = {
      type: AST_TYPE.Program,
      start: 0,
    } as const satisfies NodeLike;
    const manager = new ScopeManager("module", program as unknown as AstNode);

    enterSwitch(switchNode, parent, "body", [], manager);

    const scope = manager.current();
    expect(scope.type).toBe("switch");
    expect(scope.unsnarlBlockContext).toEqual({
      parentType: AST_TYPE.Program,
      key: "body",
      parentSpanOffset: 0,
      kind: "other",
    });
  });

  test("blockContext is null when parent or key is missing", () => {
    const code = "switch (x) {}";
    const program = parse(code);
    const switchNode = findFirst(program, AST_TYPE.SwitchStatement);
    const manager = new ScopeManager("module", program as unknown as AstNode);

    enterSwitch(switchNode, null, null, [], manager);

    expect(manager.current().unsnarlBlockContext).toBeNull();
  });
});
