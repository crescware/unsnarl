import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/primitive/ast-node.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
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

    enterFunction(fn, manager, code, diagnostics);

    const fnScope = manager.current();
    expect(fnScope.type).toBe("function");
    expect(fnScope.variables.map((v) => v.name).sort()).toEqual([
      "a",
      "b",
      "x",
    ]);
  });

  test("works for arrow function expressions without a body block", () => {
    const code = "const f = (a) => a;";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.ArrowFunctionExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, manager, code, diagnostics);

    const fnScope = manager.current();
    expect(fnScope.type).toBe("function");
    expect(fnScope.variables.map((v) => v.name)).toEqual(["a"]);
  });

  test("does not hoist when body is missing or not a BlockStatement", () => {
    const code = "const f = () => 1;";
    const program = parse(code);
    const fn = findFirst(program, AST_TYPE.ArrowFunctionExpression);
    const manager = new ScopeManager("module", program as unknown as AstNode);
    const diagnostics = new DiagnosticCollector();

    enterFunction(fn, manager, code, diagnostics);

    expect(manager.current().variables).toHaveLength(0);
  });
});
