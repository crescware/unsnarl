import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import type { AstNode } from "../../ir/model.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { enterSwitchCase } from "./enter-switch-case.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
const switchParent = {
  type: AST_TYPE.SwitchStatement,
  start: 0,
} as const satisfies NodeLike;

describe("enterSwitchCase", () => {
  test("captures caseTest, marks fallsThrough=false when consequent ends in break", () => {
    const code = "switch (x) { case 1: y; break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(
      caseNode,
      switchParent,
      "cases",
      manager,
      code,
      diagnostics,
    );

    const scope = manager.current() as unknown as {
      type: string;
      unsnarlBlockContext: { caseTest: string | null } | null;
      unsnarlFallsThrough: boolean;
      unsnarlExitsFunction: boolean;
    };
    expect(scope.type).toBe("block");
    expect(scope.unsnarlBlockContext?.caseTest).toBe("1");
    expect(scope.unsnarlFallsThrough).toBe(false);
    expect(scope.unsnarlExitsFunction).toBe(false);
  });

  test("marks fallsThrough=true when consequent has no exit statement", () => {
    const code = "switch (x) { case 1: y = 1; case 2: break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(
      caseNode,
      switchParent,
      "cases",
      manager,
      code,
      diagnostics,
    );

    expect(
      (manager.current() as unknown as { unsnarlFallsThrough: boolean })
        .unsnarlFallsThrough,
    ).toBe(true);
  });

  test("marks exitsFunction=true when consequent ends in return", () => {
    const code =
      "function f() { switch (x) { case 1: return 1; case 2: break; } }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(
      caseNode,
      switchParent,
      "cases",
      manager,
      code,
      diagnostics,
    );

    expect(
      (manager.current() as unknown as { unsnarlExitsFunction: boolean })
        .unsnarlExitsFunction,
    ).toBe(true);
  });

  test("default case (no test) sets caseTest to null", () => {
    const code = "switch (x) { default: break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(
      caseNode,
      switchParent,
      "cases",
      manager,
      code,
      diagnostics,
    );

    const scope = manager.current() as unknown as {
      unsnarlBlockContext: { caseTest: string | null } | null;
    };
    expect(scope.unsnarlBlockContext?.caseTest).toBeNull();
  });

  test("blockContext is null when parent or key is missing", () => {
    const code = "switch (x) { case 1: break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(caseNode, null, null, manager, code, diagnostics);

    expect(manager.current().unsnarlBlockContext).toBeNull();
  });
});
