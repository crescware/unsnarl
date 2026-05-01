import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { DEFINITION_TYPE } from "../definition-type.js";
import { ScopeManager } from "../manager.js";
import { handleEnter } from "./handle-enter.js";
import type { NodeLike } from "./node-like.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

function makeManager(program: NodeLike): ScopeManager {
  return new ScopeManager("module", program as unknown as AstNode);
}

describe("handleEnter", () => {
  test('returns "skip" for type-only subtrees (TS interface declaration)', () => {
    const code = "interface Foo { x: number }";
    const program = parse(code);
    const manager = makeManager(program);
    const diagnostics = new DiagnosticCollector();
    const node = { type: "TSInterfaceDeclaration" } as const satisfies NodeLike;

    const action = handleEnter(
      node,
      null,
      null,
      [],
      manager,
      code,
      diagnostics,
    );

    expect(action).toBe("skip");
  });

  test('returns "skip" when key is a TS type-only key', () => {
    const code = "let x: number = 1;";
    const node = { type: "TSTypeAnnotation" } as const satisfies NodeLike;
    const manager = makeManager(parse(code));

    const action = handleEnter(
      node,
      null,
      "typeAnnotation",
      [],
      manager,
      code,
      new DiagnosticCollector(),
    );

    expect(action).toBe("skip");
  });

  test.each([
    {
      name: "FunctionDeclaration -> pushes function scope",
      code: "function foo() {}",
      type: AST_TYPE.FunctionDeclaration,
      expectedScopeType: "function",
    },
    {
      name: "FunctionExpression -> pushes function scope",
      code: "const f = function() {};",
      type: AST_TYPE.FunctionExpression,
      expectedScopeType: "function",
    },
    {
      name: "ArrowFunctionExpression -> pushes function scope",
      code: "const f = () => 1;",
      type: AST_TYPE.ArrowFunctionExpression,
      expectedScopeType: "function",
    },
    {
      name: "ForStatement -> pushes for scope",
      code: "for (let i = 0; i < 1; i++) {}",
      type: AST_TYPE.ForStatement,
      expectedScopeType: "for",
    },
    {
      name: "ForOfStatement -> pushes for scope",
      code: "for (const x of items) {}",
      type: AST_TYPE.ForOfStatement,
      expectedScopeType: "for",
    },
    {
      name: "ForInStatement -> pushes for scope",
      code: "for (const k in obj) {}",
      type: AST_TYPE.ForInStatement,
      expectedScopeType: "for",
    },
    {
      name: "SwitchStatement -> pushes switch scope",
      code: "switch (x) {}",
      type: AST_TYPE.SwitchStatement,
      expectedScopeType: "switch",
    },
    {
      name: "SwitchCase -> pushes block scope",
      code: "switch (x) { case 1: break; }",
      type: AST_TYPE.SwitchCase,
      expectedScopeType: "block",
    },
    {
      name: "CatchClause -> pushes catch scope",
      code: "try {} catch (e) {}",
      type: DEFINITION_TYPE.CatchClause,
      expectedScopeType: "catch",
    },
  ])("$name", ({ code, type, expectedScopeType }) => {
    const program = parse(code);
    const target = findFirst(program, type);
    const manager = makeManager(program);

    handleEnter(
      target,
      null,
      null,
      [],
      manager,
      code,
      new DiagnosticCollector(),
    );

    expect(manager.current().type).toBe(expectedScopeType);
  });

  test("BlockStatement under FunctionDeclaration is NOT pushed (function body is part of fn scope)", () => {
    const code = "function foo() { let x = 1; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const manager = makeManager(program);
    const fnParent = {
      type: AST_TYPE.FunctionDeclaration,
    } as const satisfies NodeLike;

    handleEnter(
      block,
      fnParent,
      "body",
      [],
      manager,
      code,
      new DiagnosticCollector(),
    );

    expect(manager.current().type).toBe("module");
  });

  test("plain BlockStatement (not under fn/catch) is pushed as block scope", () => {
    const code = "if (x) { let y = 1; }";
    const program = parse(code);
    const block = findFirst(program, AST_TYPE.BlockStatement);
    const manager = makeManager(program);
    const parent = {
      type: AST_TYPE.IfStatement,
      start: 0,
    } as const satisfies NodeLike;

    handleEnter(
      block,
      parent,
      "consequent",
      [],
      manager,
      code,
      new DiagnosticCollector(),
    );

    expect(manager.current().type).toBe("block");
  });

  test("unknown node types do nothing", () => {
    const program = parse("");
    const manager = makeManager(program);
    const before = manager.current();

    handleEnter(
      { type: AST_TYPE.ExpressionStatement },
      null,
      null,
      [],
      manager,
      "",
      new DiagnosticCollector(),
    );

    expect(manager.current()).toBe(before);
  });
});
