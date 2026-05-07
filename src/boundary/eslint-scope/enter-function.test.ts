import { describe, expect, test } from "vitest";

import { ScopeManager } from "../../analyzer/manager.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { enterFunction } from "./enter-function.js";
import { findFirst } from "./testing/find-first.js";
import { parse } from "./testing/parse.js";

describe("enterFunction", () => {
  test("pushes a function scope, declares params, and hoists body declarations", () => {
    const code = "function foo(a, b) { let x = 1; }";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.FunctionDeclaration);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, null, null, [], manager, code, diagnostics, {});

    const fnScope = manager.current();
    expect(fnScope.type).toBe("function");
    expect(fnScope.variables.map((v) => v.name).sort()).toEqual([
      "a",
      "arguments",
      "b",
      "x",
    ]);
  });

  test("registers the implicit 'arguments' binding for function declarations", () => {
    const code = "function foo() {}";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.FunctionDeclaration);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, null, null, [], manager, code, diagnostics, {});

    const args = manager.current().set.get("arguments");
    expect(args?.name).toBe("arguments");
    expect(args?.identifiers).toEqual([]);
    expect(args?.defs).toEqual([]);
  });

  test("works for arrow function expressions without a body block", () => {
    const code = "const f = (a) => a;";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.ArrowFunctionExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, null, null, [], manager, code, diagnostics, {});

    const fnScope = manager.current();
    expect(fnScope.type).toBe("function");
    expect(fnScope.variables.map((v) => v.name)).toEqual(["a"]);
  });

  test("does not register 'arguments' for arrow function expressions", () => {
    const code = "const f = () => 1;";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.ArrowFunctionExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, null, null, [], manager, code, diagnostics, {});

    expect(manager.current().set.has("arguments")).toBe(false);
  });

  test("does not hoist when body is missing or not a BlockStatement", () => {
    const code = "const f = () => 1;";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.ArrowFunctionExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, null, null, [], manager, code, diagnostics, {});

    expect(manager.current().variables).toHaveLength(0);
  });
});
