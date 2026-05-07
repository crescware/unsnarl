import { describe, expect, test } from "vitest";

import { ScopeManager } from "../../analyzer/manager.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { enterSwitch } from "./enter-switch.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
import type { AnalysisVisitor, ScopeVisitInput } from "./visitor.js";

describe("enterSwitch", () => {
  test("pushes a switch scope and notifies visitor", () => {
    const code = "switch (x) { case 1: break; }";
    const program = parse(code);
    const switchNode = findFirst(program, AST_TYPE.SwitchStatement);
    const parent = {
      type: AST_TYPE.Program,
      start: 0,
    } as const satisfies NodeLike;
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterSwitch(switchNode, parent, "body", [], manager, visitor);

    const scope = manager.current();
    expect(scope.type).toBe("switch");
    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toBe(parent);
    expect(captured[0]?.key).toBe("body");
  });

  test("notifies visitor with null parent / key when missing", () => {
    const code = "switch (x) {}";
    const program = parse(code);
    const switchNode = findFirst(program, AST_TYPE.SwitchStatement);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterSwitch(switchNode, null, null, [], manager, visitor);

    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toBeNull();
    expect(captured[0]?.key).toBeNull();
  });
});
