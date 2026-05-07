import { describe, expect, test } from "vitest";

import { ScopeManager } from "../../analyzer/manager.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
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

    const scope = manager.current();
    const ann = manager.annotations.ofScope(scope);
    expect(scope.type).toBe("block");
    expect(ann.blockContext?.kind).toBe("case-clause");
    if (ann.blockContext?.kind === "case-clause") {
      expect(ann.blockContext.caseTest).toBe("1");
    }
    expect(ann.fallsThrough).toBe(false);
    expect(ann.exitsFunction).toBe(false);
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

    expect(manager.annotations.ofScope(manager.current()).fallsThrough).toBe(
      true,
    );
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

    expect(manager.annotations.ofScope(manager.current()).exitsFunction).toBe(
      true,
    );
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

    const ann = manager.annotations.ofScope(manager.current());
    expect(ann.blockContext?.kind).toBe("case-clause");
    if (ann.blockContext?.kind === "case-clause") {
      expect(ann.blockContext.caseTest).toBeNull();
    }
  });

  test("blockContext is null when parent or key is missing", () => {
    const code = "switch (x) { case 1: break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterSwitchCase(caseNode, null, null, manager, code, diagnostics);

    expect(
      manager.annotations.ofScope(manager.current()).blockContext,
    ).toBeNull();
  });
});
