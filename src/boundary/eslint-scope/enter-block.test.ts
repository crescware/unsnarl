import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { enterBlock } from "./enter-block.js";
import { ScopeManager } from "./manager.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";
import type { AnalysisVisitor, ScopeVisitInput } from "./visitor.js";

describe("enterBlock", () => {
  test("pushes a block scope, notifies visitor, and hoists body declarations", () => {
    const code = "if (x) { let y = 1; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 5,
    } as const satisfies NodeLike;
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterBlock(
      block,
      parent,
      "consequent",
      [],
      manager,
      code,
      diagnostics,
      visitor,
    );

    const scope = manager.current();
    expect(scope.type).toEqual("block");
    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toEqual(parent);
    expect(captured[0]?.key).toEqual("consequent");
    expect(scope.variables.map((v) => v.name)).toEqual(["y"]);
  });

  test("notifies visitor with parent=null when no enclosing context", () => {
    const code = "{ let z = 2; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();
    const captured: ScopeVisitInput[] = [];
    const visitor: AnalysisVisitor = {
      onScope(input) {
        captured.push(input);
      },
    };

    enterBlock(block, null, null, [], manager, code, diagnostics, visitor);

    expect(captured).toHaveLength(1);
    expect(captured[0]?.parent).toEqual(null);
    expect(captured[0]?.key).toEqual(null);
  });

  test("does not hoist when body is missing", () => {
    const block = { type: AST_TYPE.BlockStatement } as const satisfies NodeLike;
    const program = parse("");
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterBlock(block, null, null, [], manager, "", diagnostics, {});

    expect(manager.current().variables).toHaveLength(0);
  });
});
