import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { enterSwitchCase } from "./enter-switch-case.js";
import { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
import type { AnalysisVisitor, ScopeVisitInput } from "./visitor.js";

const switchParent = {
  type: AST_TYPE.SwitchStatement,
  start: 0,
} as const satisfies NodeLike;

describe("enterSwitchCase", () => {
  test("pushes a block scope, notifies visitor, and hoists consequent declarations", () => {
    const code = "switch (x) { case 1: var y = 1; break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterSwitchCase(
      caseNode,
      switchParent,
      "cases",
      [],
      manager,
      code,
      diagnostics,
      visitor,
    );

    const scope = manager.current();
    expect(scope.type).toEqual("block");
    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toEqual(switchParent);
    expect(captured[0]?.key).toEqual("cases");
  });

  test("notifies visitor with parent=null when no enclosing context", () => {
    const code = "switch (x) { case 1: break; }";
    const program = parse(code);
    const caseNode = findFirst(program, AST_TYPE.SwitchCase);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterSwitchCase(
      caseNode,
      null,
      null,
      [],
      manager,
      code,
      diagnostics,
      visitor,
    );

    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toEqual(null);
    expect(captured[0]?.key).toEqual(null);
  });
});
