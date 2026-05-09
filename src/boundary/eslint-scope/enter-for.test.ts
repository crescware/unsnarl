import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { enterFor } from "./enter-for.js";
import { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
import type { AnalysisVisitor, ScopeVisitInput } from "./visitor.js";

describe("enterFor", () => {
  test("pushes a for scope, notifies visitor, and declares loop bindings", () => {
    const code = "for (let i = 0; i < 10; i++) {}";
    const program = parse(code);
    const forNode = findFirst(program, AST_TYPE.ForStatement);
    const parent = {
      type: AST_TYPE.Program,
      start: 0,
    } as const satisfies NodeLike;
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterFor(forNode, parent, "body", [], manager, code, diagnostics, visitor);

    const scope = manager.current();
    expect(scope.type).toBe("for");
    expect(scope.variables.map((v) => v.name)).toEqual(["i"]);
    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toBe(parent);
    expect(captured[0]?.key).toBe("body");
  });

  test("works for ForOfStatement", () => {
    const code = "for (const x of items) {}";
    const program = parse(code);
    const forNode = findFirst(program, AST_TYPE.ForOfStatement);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFor(forNode, null, null, [], manager, code, diagnostics, {});

    expect(manager.current().variables.map((v) => v.name)).toEqual(["x"]);
  });
});
